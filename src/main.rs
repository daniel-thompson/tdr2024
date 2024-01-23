// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{math::vec3, prelude::*, render::camera::ScalingMode, window};
use bevy_ecs_tilemap::prelude as ecs_tilemap;
use clap::Parser;
use std::f32::consts::PI;

mod assets;
mod dashboard;
mod editor;
mod mapping;
mod physics;
mod tilemap;
mod util;

#[derive(Clone, Debug, Parser, Resource)]
#[command(author, version, about, long_about = None)]
struct Preferences {
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

impl Preferences {
    fn debug_low(&self) -> bool {
        self.debug >= 1
    }

    fn debug_high(&self) -> bool {
        self.debug >= 2
    }
}

fn main() {
    let args = Preferences::parse();

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
            assets::Plugin,
            editor::Plugin,
            ecs_tilemap::TilemapPlugin,
            mapping::Plugin,
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
                handle_keyboard,
                handle_ai_players,
                physics::apply_velocity
                    .after(handle_ai_players)
                    .after(handle_keyboard),
                physics::apply_friction.after(physics::apply_velocity),
                track_player.after(physics::apply_velocity),
                physics::collision_detection
                    .after(physics::apply_velocity)
                    .after(handle_keyboard)
                    .after(handle_ai_players),
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

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    // Request a constant width projection. 24 is the width in world units.
    camera.projection.scaling_mode = ScalingMode::FixedHorizontal(1920.0);
    commands.spawn(camera);
}

fn load_maps(mut commands: Commands, asset_server: Res<AssetServer>, prefs: Res<Preferences>) {
    let p = format!("embedded://tdr2024/assets/level{}.tmx", prefs.level);
    let map_handle: Handle<tilemap::TiledMap> = asset_server.load(p);

    commands.spawn(tilemap::TiledMapBundle {
        tiled_map: map_handle,
        ..default()
    });
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
        physics::Angle(0.0),
        physics::Velocity(Vec2::new(0.0, 20.0)),
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
        physics::Angle(PI / 12.0),
        physics::Velocity(Vec2::new(0.0, 20.0)),
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
        physics::Angle(PI / 12.0),
        physics::Velocity(Vec2::new(0.0, 20.0)),
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
        physics::Angle(PI / 12.0),
        physics::Velocity(Vec2::new(0.0, 20.0)),
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

fn handle_keyboard(
    mut query: Query<(
        &mut physics::Angle,
        &mut physics::Velocity,
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
        &mut physics::Angle,
        &mut physics::Velocity,
        &mut Transform,
        &mut Racer,
        Without<Player>,
    )>,
    time: Res<Time>,
    guide: Option<Res<mapping::GuidanceField>>,
    prefs: Res<Preferences>,
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

        if prefs.debug_high() {
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
    player: Query<(&Transform, &physics::Velocity, With<Player>)>,
    mut camera: Query<(&mut Transform, With<Camera>, Without<Player>)>,
) {
    let (txp, _, _) = player.single();

    for (mut txc, _, _) in camera.iter_mut() {
        txc.translation.x = txp.translation.x;
        txc.translation.y = txp.translation.y;
        //txc.rotation = txp.rotation;
    }
}
