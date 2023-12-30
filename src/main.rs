use bevy::{prelude::*, render::camera::ScalingMode, window};
use bevy_ecs_tilemap::prelude::*;
use itertools::Itertools;
use std::f32::consts::PI;

mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Orcombe Point".to_string(),
                    resolution: (1920.0, 1080.0).into(),
                    present_mode: window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            bevy_editor_pls::prelude::EditorPlugin::default(),
        ))
        .add_plugins((TilemapPlugin, helpers::tiled::TiledMapPlugin))
        .insert_resource(ClearColor(Color::rgb(0.153, 0.682, 0.376)))
        .add_systems(
            Startup,
            (load_maps, spawn_camera, spawn_player, spawn_ai_players),
        )
        .add_systems(
            Update,
            (
                handle_keyboard,
                handle_ai_players,
                apply_friction,
                apply_velocity,
                track_player,
                generate_guidance_field,
            ),
        )
        .run();
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Racer;

#[derive(Component, Default, Debug)]
struct Track;

#[derive(Component, Debug)]
struct Velocity(Vec2);

#[derive(Component, Clone, Debug)]
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

fn load_maps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<helpers::tiled::TiledMap> = asset_server.load("level1.tmx");

    commands.spawn(helpers::tiled::TiledMapBundle {
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
        let layer = map.get_layer(1)?.as_tile_layer()?;

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
        let pos = shift + pos.clone();

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
    mut map_events: EventReader<AssetEvent<helpers::tiled::TiledMap>>,
    maps: Res<Assets<helpers::tiled::TiledMap>>,
) {
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                if let Some(map) = maps.get(id.clone()) {
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
        asset_server.load("kenney_racing-pack/PNG/Cars/car_red_5.png"),
        Vec2::new(70., 121.),
        1,
        1,
        None,
        None,
    );

    commands.spawn((
        Player,
        Racer,
        Angle(0.0),
        Velocity(Vec2::new(0.0, 20.0)),
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
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
    let atlas = TextureAtlas::from_grid(
        asset_server.load("kenney_racing-pack/PNG/Cars/car_blue_1.png"),
        Vec2::new(70., 121.),
        1,
        1,
        None,
        None,
    );

    commands.spawn((
        Racer,
        Angle(0.0),
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
}

fn apply_friction(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut v in query.iter_mut() {
        v.0 *= 1.0 - (delta * 1.2);
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (v, mut t) in query.iter_mut() {
        t.translation.x += delta * v.0.x;
        t.translation.y += delta * v.0.y;
    }
}

fn handle_keyboard(
    mut query: Query<(&mut Angle, &mut Velocity, &mut Transform, With<Player>)>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let delta = time.delta_seconds();

    let (mut a, mut v, mut t, _) = query.single_mut();
    if input.pressed(KeyCode::Z) {
        a.0 += delta * 3.0;
    }
    if input.pressed(KeyCode::X) {
        a.0 -= delta * 3.0;
    }
    if input.pressed(KeyCode::ShiftRight) {
        v.0 += delta * 400.0 * Vec2::from_angle(a.0);
    }

    a.normalize();
    t.rotation = a.to_quat();
}

fn handle_ai_players(
    mut query: Query<(
        &mut Angle,
        &mut Velocity,
        &mut Transform,
        With<Racer>,
        Without<Player>,
    )>,
    time: Res<Time>,
    guide: Option<Res<GuidanceField>>,
) {
    if guide.is_none() {
        return;
    }
    let guide = guide.unwrap();

    let delta = time.delta_seconds();

    for (mut a, mut v, mut t, _, _) in query.iter_mut() {
        let pos = Vec2::new(t.translation.x, t.translation.y);

        let left_whisker = pos + (240.0 * Vec2::from_angle(a.0 + (PI / 8.)));
        let left_pixel = guide.get(&left_whisker);
        let right_whisker = pos + (240.0 * Vec2::from_angle(a.0 - (PI / 8.)));
        let right_pixel = guide.get(&right_whisker);

        if (left_pixel - 10) > right_pixel {
            a.0 += delta * 3.0;
        }
        if (right_pixel - 10) > left_pixel {
            a.0 -= delta * 3.0;
        }
        if true {
            v.0 += delta * 400.0 * Vec2::from_angle(a.0);
        }

        a.normalize();
        t.rotation = a.to_quat();
    }
}

fn track_player(
    player: Query<(&Transform, With<Player>)>,
    mut camera: Query<(&mut Transform, With<Camera>, Without<Player>)>,
) {
    let (txp, _) = player.single();

    for (mut txc, _, _) in camera.iter_mut() {
        txc.translation.x = txp.translation.x;
        txc.translation.y = txp.translation.y;
    }
}
