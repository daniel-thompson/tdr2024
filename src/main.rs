// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::camera::ScalingMode,
    window,
};
use bevy_ecs_tilemap::prelude as ecs_tilemap;
use clap::Parser;
use itertools::Itertools;
use slicetools::*;
use std::f32::consts::PI;

mod assets;
mod dashboard;
mod editor;
mod tilemap;
mod util;
use util::IteratorToArrayExt;

#[derive(Clone, Debug, Parser, Resource)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Turn debugging visualizations on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Jump to the selected level
    #[arg(short, long, default_value_t = 1)]
    level: u32,

    /// Enable windowed mode (for debugging try: -wdd)
    #[arg(short, long)]
    window: bool,
}

impl Args {
    fn debug_low(&self) -> bool {
        self.debug >= 1
    }

    fn debug_high(&self) -> bool {
        self.debug >= 2
    }
}

fn main() {
    let args = Args::parse();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TDR2024 - Orcombe Point edition".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    present_mode: window::PresentMode::AutoVsync,
                    mode: if args.window {
                        window::WindowMode::default()
                    } else {
                        window::WindowMode::BorderlessFullscreen
                    },
                    cursor: window::Cursor {
                        visible: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            editor::Plugin,
            ecs_tilemap::TilemapPlugin,
            assets::Plugin,
            tilemap::TiledMapPlugin,
            dashboard::Plugin,
        ))
        .insert_resource(ClearColor(Color::rgb_linear(0.153, 0.682, 0.376)))
        .insert_resource(args)
        .add_systems(
            Startup,
            (load_maps, spawn_camera, spawn_player, spawn_ai_players),
        )
        .add_systems(
            Update,
            (
                generate_guidance_field,
                handle_keyboard,
                handle_ai_players,
                apply_velocity
                    .after(handle_ai_players)
                    .after(handle_keyboard),
                apply_friction.after(apply_velocity),
                track_player.after(apply_velocity),
                collision_detection
                    .after(apply_velocity)
                    .after(handle_keyboard)
                    .after(handle_ai_players),
                apply_time_penalties,
            ),
        )
        .run();
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug, Default)]
struct Racer {
    penalty: f32,
    last_tile: Option<Vec2>,
}

#[derive(Component, Default, Debug)]
struct Track;

#[derive(Component, Debug, Reflect)]
struct Velocity(Vec2);

#[derive(Component, Clone, Debug, Reflect)]
struct Angle(f32);

impl Angle {
    fn normalize(&mut self) {
        while self.0 > PI {
            self.0 -= 2.0 * PI;
        }
        while self.0 < -PI {
            self.0 += 2.0 * PI;
        }
    }

    fn to_quat(&self) -> Quat {
        Quat::from_rotation_z(self.0 - PI / 2.0)
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    // Request a constant width projection. 24 is the width in world units.
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(1920.0);
    commands.spawn(camera);
}

fn load_maps(mut commands: Commands, asset_server: Res<AssetServer>, args: Res<Args>) {
    let p = format!("embedded://tdr2024/assets/level{}.tmx", args.level);
    let map_handle: Handle<tilemap::TiledMap> = asset_server.load(p);

    commands.spawn(tilemap::TiledMapBundle {
        tiled_map: map_handle,
        ..default()
    });
}

#[derive(Resource)]
struct GuidanceField {
    image: image::GrayImage,
}

impl GuidanceField {
    fn from_map(map: &tiled::Map) -> Option<Self> {
        let layer = map
            .get_layer(1)
            .unwrap_or(map.get_layer(0)?)
            .as_tile_layer()?;

        let w = layer.width()?;
        let h = layer.height()?;

        let micro_map = (0..h)
            .cartesian_product(0..w)
            .map(|(y, x)| layer.get_tile(x as i32, y as i32).is_some() as u8 * 255)
            .collect::<Vec<u8>>();
        let micro_map = image::GrayImage::from_vec(w, h, micro_map)?;

        // The ideal guidance field is upscaled using nearest pixel and a 128-pixel
        // gaussian blur applied. However the blur in the image crate isn't very
        // inefficient for large radius blurs. Instead we work in multiple stages
        // allowing a (fast) 8-pixel blur before doing a second upscale with a
        // gaussian filter.
        let mini_map = image::imageops::resize(
            &micro_map,
            w * 8,
            h * 8,
            image::imageops::FilterType::Nearest,
        );
        let mini_field = image::imageops::blur(&mini_map, 8.0);

        let field = image::imageops::resize(
            &mini_field,
            w * 128,
            h * 128,
            image::imageops::FilterType::Gaussian,
        );

        Some(Self { image: field })
    }

    fn get(&self, pos: &Vec2) -> i32 {
        let (w, h) = self.image.dimensions();
        let shift = Vec2::new(w as f32 * 0.5, h as f32 * 0.5);
        let pos = shift + *pos;

        if (pos.y as u32) < h {
            let x = pos.x as u32;
            let y = h - (pos.y as u32);
            self.image
                .get_pixel_checked(x, y)
                .map(|luma| luma.0[0])
                .unwrap_or(0) as i32
        } else {
            0
        }
    }
}

fn generate_guidance_field(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<tilemap::TiledMap>>,
    maps: Res<Assets<tilemap::TiledMap>>,
) {
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                if let Some(map) = maps.get(*id) {
                    commands.insert_resource(
                        GuidanceField::from_map(&map.map)
                            .expect("Track cannot be converted to GuidanceField"),
                    );
                }
            }
            //AssetEvent::Modified { id } => {
            //    println!("Map changed!");
            //}
            //AssetEvent::Removed { id } => {
            //    println!("Map removed!");
            //}
            _ => continue,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("embedded://tdr2024/assets/kenney_racing-pack/PNG/Cars/car_red_5.png"),
        Vec2::new(70., 121.),
        1,
        1,
        None,
        None,
    );

    commands.spawn((
        Player,
        Racer::default(),
        Angle(0.0),
        Velocity(Vec2::new(0.0, 20.0)),
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            transform: Transform {
                translation: vec3(-1000.0, 0.0, 3.0),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_ai_players(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let handle =
        asset_server.load("embedded://tdr2024/assets/kenney_racing-pack/PNG/Cars/car_blue_1.png");
    let atlas = TextureAtlas::from_grid(handle, Vec2::new(70., 121.), 1, 1, None, None);

    commands.spawn((
        Racer::default(),
        Angle(PI / 12.0),
        Velocity(Vec2::new(0.0, 20.0)),
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        },
    ));

    let handle =
        asset_server.load("embedded://tdr2024/assets/kenney_racing-pack/PNG/Cars/car_yellow_3.png");
    let atlas = TextureAtlas::from_grid(handle, Vec2::new(70., 121.), 1, 1, None, None);
    commands.spawn((
        Racer::default(),
        Angle(PI / 12.0),
        Velocity(Vec2::new(0.0, 20.0)),
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            transform: Transform {
                translation: Vec3::new(-333.3, 0.0, 2.0),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        },
    ));

    let handle =
        asset_server.load("embedded://tdr2024/assets/kenney_racing-pack/PNG/Cars/car_green_4.png");
    let atlas = TextureAtlas::from_grid(handle, Vec2::new(70., 121.), 1, 1, None, None);
    commands.spawn((
        Racer::default(),
        Angle(PI / 12.0),
        Velocity(Vec2::new(0.0, 20.0)),
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            transform: Transform {
                translation: Vec3::new(-666.6, 0.0, 2.0),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        },
    ));
}

fn apply_time_penalties(
    mut query: Query<(&mut Transform, &mut Racer)>,
    maps: Res<Assets<tilemap::TiledMap>>,
) {
    let map = match maps.iter().next() {
        Some(map) => &map.1.map,
        None => return,
    };

    let layer = map
        .get_layer(1)
        .or(map.get_layer(0))
        .and_then(|layer| layer.as_tile_layer())
        .expect("Failed to lookup track layer");

    for (t, mut r) in query.iter_mut() {
        let x = (t.translation.x / map.tile_width as f32) + (map.width as f32 / 2.0);
        let y = (-t.translation.y / map.tile_height as f32) + (map.height as f32 / 2.0);

        let on_track = layer.get_tile(x as i32, y as i32).is_some();
        if on_track {
            let now = vec2(x, y);
            if let Some(prev) = r.last_tile {
                let delta = now.distance(prev).abs();
                if delta > 1.0 {
                    r.penalty += delta;
                }
            }
            r.last_tile = Some(now);
        }
    }
}

fn apply_friction(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    time: Res<Time>,
    guide: Option<Res<GuidanceField>>,
) {
    let delta = time.delta_seconds();
    for (mut v, t) in query.iter_mut() {
        v.0 *= 1.0 - (delta * 1.2);

        if let Some(guide) = &guide {
            let pos = Vec2::new(t.translation.x, t.translation.y);
            let pixel = guide.get(&pos);
            if pixel < 140 {
                let factor = 1.2 + 1.2 * (1.0 - (pixel as f32 / 140.0));
                v.0 *= 1.0 - (delta * factor);
            }
        }
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (v, mut t) in query.iter_mut() {
        t.translation.x += delta * v.0.x;
        t.translation.y += delta * v.0.y;
    }
}

fn same_side(p1: Vec2, p2: Vec2, line: (Vec2, Vec2)) -> bool {
    let p1 = Vec3::from((p1, 0.0));
    let p2 = Vec3::from((p2, 0.0));
    let line = (Vec3::from((line.0, 0.0)), Vec3::from((line.1, 0.0)));

    let cp1 = (line.1 - line.0).cross(p1 - line.0);
    let cp2 = (line.1 - line.0).cross(p2 - line.0);

    cp1.dot(cp2) >= 0.0
}

fn point_in_polygon(pt: Vec2, shape: &[Vec2]) -> bool {
    let n = shape.len();
    shape
        .windows(3)
        .chain(std::iter::once(
            [shape[n - 2], shape[n - 1], shape[0]].as_slice(),
        ))
        .chain(std::iter::once(
            [shape[n - 1], shape[0], shape[1]].as_slice(),
        ))
        .all(|x| same_side(pt, x[0], (x[1], x[2])))
}

struct CollisionBox {
    points: [Vec2; 8],
}

impl CollisionBox {
    fn from_transform(tf: &Transform, sz: &Vec2) -> Self {
        let w = sz.x * 0.5;
        let h = sz.y * 0.5;

        // c is used to round the corners of the box, choosing
        // 2.5 is a little arbitrary but it gives a good "feel"
        // for most artwork... and you could handle special cases
        // by creating the box by hand.
        let c = w.min(h) / 2.5;

        Self {
            points: [
                vec2(c - w, h),
                vec2(w - c, h),
                vec2(w, h - c),
                vec2(w, c - h),
                vec2(w - c, -h),
                vec2(c - w, -h),
                vec2(-w, c - h),
                vec2(-w, h - c),
            ]
            .iter()
            .map(|v2| {
                let v3 = Vec3::from((*v2, 0.0));
                let pt = tf.transform_point(v3);
                vec2(pt.x, pt.y)
            })
            .to_array(),
        }
    }

    /// Test whether two rectangles are touching.
    fn is_touching(&self, other: &CollisionBox) -> bool {
        other
            .points
            .iter()
            .any(|pt| point_in_polygon(*pt, &self.points))
            || self
                .points
                .iter()
                .any(|pt| point_in_polygon(*pt, &other.points))
    }

    fn draw(&self, gizmos: &mut Gizmos) {
        for w in self.points.windows(2) {
            gizmos.line_2d(w[0], w[1], Color::BLUE);
        }
        gizmos.line_2d(self.points[7], self.points[0], Color::BLUE);
    }
}

fn collision_detection(
    mut query: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Velocity)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    args: Res<Args>,
    mut gizmos: Gizmos,
) {
    let mut colliders = query.iter_mut().collect::<Vec<_>>();
    let mut it = colliders.pairs_mut();
    while let Some((a, b)) = it.next() {
        let atx = match texture_atlases.get(a.1) {
            Some(tx) => tx,
            None => continue,
        };
        let btx = match texture_atlases.get(b.1) {
            Some(tx) => tx,
            None => continue,
        };

        let mut abox = CollisionBox::from_transform(&a.0, &atx.size);
        let mut bbox = CollisionBox::from_transform(&b.0, &btx.size);
        if args.debug_low() {
            abox.draw(&mut gizmos);
            bbox.draw(&mut gizmos);
        }

        if abox.is_touching(&bbox) {
            std::mem::swap(&mut a.2 .0, &mut b.2 .0);

            let a2 = vec2(a.0.translation.x, a.0.translation.y);
            let b2 = vec2(b.0.translation.x, b.0.translation.y);
            let nudge = Vec3::from(((b2 - a2).normalize() * 0.5, 0.0));
            while abox.is_touching(&bbox) {
                a.0.translation -= nudge;
                b.0.translation += nudge;

                abox = CollisionBox::from_transform(&a.0, &atx.size);
                bbox = CollisionBox::from_transform(&b.0, &btx.size);
            }
        }
    }
}

fn handle_keyboard(
    mut query: Query<(
        &mut Angle,
        &mut Velocity,
        &mut Transform,
        &mut Racer,
        With<Player>,
    )>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let delta = time.delta_seconds();

    let (mut a, mut v, mut t, mut r, _) = query.single_mut();

    if r.penalty > 0.0 {
        r.penalty = if r.penalty < delta {
            0.0
        } else {
            r.penalty - delta
        };
        return;
    }

    if input.pressed(KeyCode::Z) {
        a.0 += delta * 3.0;
    }
    if input.pressed(KeyCode::X) {
        a.0 -= delta * 3.0;
    }
    if input.pressed(KeyCode::ShiftRight) || input.pressed(KeyCode::ShiftLeft) {
        v.0 += delta * 580.0 * Vec2::from_angle(a.0);
    }

    a.normalize();
    t.rotation = a.to_quat();
}

fn handle_ai_players(
    mut query: Query<(
        &mut Angle,
        &mut Velocity,
        &mut Transform,
        &mut Racer,
        Without<Player>,
    )>,
    time: Res<Time>,
    guide: Option<Res<GuidanceField>>,
    args: Res<Args>,
    mut gizmos: Gizmos,
) {
    if guide.is_none() {
        return;
    }
    let guide = guide.unwrap();

    let delta = time.delta_seconds();

    for (mut a, mut v, mut t, mut r, _) in query.iter_mut() {
        if r.penalty > 0.0 {
            r.penalty = if r.penalty < delta {
                0.0
            } else {
                r.penalty - delta
            };
            continue;
        }
        let pos = Vec2::new(t.translation.x, t.translation.y);

        let left_whisker = pos + (425.0 * Vec2::from_angle(a.0 + (PI / 12.)));
        let left_pixel = guide.get(&left_whisker);
        let right_whisker = pos + (425.0 * Vec2::from_angle(a.0 - (PI / 12.)));
        let right_pixel = guide.get(&right_whisker);

        let left_whisker2 = pos + (200.0 * Vec2::from_angle(a.0 + (PI / 6.)));
        let left_pixel2 = guide.get(&left_whisker2);
        let right_whisker2 = pos + (200.0 * Vec2::from_angle(a.0 - (PI / 6.)));
        let right_pixel2 = guide.get(&right_whisker2);

        let front_whisker = pos + (425.0 * Vec2::from_angle(a.0));
        let front_pixel = guide.get(&front_whisker);

        if args.debug_high() {
            for v in [
                left_whisker,
                right_whisker,
                left_whisker2,
                right_whisker2,
                front_whisker,
            ] {
                gizmos.circle_2d(v, 2.0, Color::BLUE);
                gizmos.line_2d(pos, v, Color::BLUE);
            }
        }

        if ((left_pixel - 10) > right_pixel) || ((left_pixel2 - 10) > right_pixel2) {
            a.0 += delta * 3.0;
        }
        if ((right_pixel - 10) > left_pixel) || ((right_pixel2 - 10) > left_pixel2) {
            a.0 -= delta * 3.0;
        }

        if front_pixel > 50 {
            v.0 += delta * 600.0 * Vec2::from_angle(a.0);
        }

        a.normalize();
        t.rotation = a.to_quat();
    }
}

fn track_player(
    player: Query<(&Transform, &Velocity, With<Player>)>,
    mut camera: Query<(&mut Transform, With<Camera>, Without<Player>)>,
) {
    let (txp, _, _) = player.single();

    for (mut txc, _, _) in camera.iter_mut() {
        txc.translation.x = txp.translation.x;
        txc.translation.y = txp.translation.y;
        //txc.rotation = txp.rotation;
    }
}
