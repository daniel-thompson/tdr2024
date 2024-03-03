// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{log, prelude::*, render::camera::ScalingMode, window};
use bevy_ecs_tilemap::prelude as ecs_tilemap;
use clap::Parser;
use std::f32::consts::PI;

mod assets;
mod dashboard;
mod editor;
mod geometry;
mod mapping;
mod menu;
mod objectmap;
mod physics;
mod tilemap;
mod util;

#[derive(Clone, Debug, Parser, Resource)]
#[command(author, version, about, long_about = None)]
struct Preferences {
    /// Turn debugging visualizations on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Keep the car still while the world revolves around you
    #[arg(short, long)]
    jenny_mode: bool,

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
                    ..default()
                }),
                ..default()
            }),
            assets::Plugin,
            editor::Plugin,
            ecs_tilemap::TilemapPlugin,
            mapping::Plugin,
            menu::MenuPlugin,
            objectmap::Plugin,
            tilemap::TiledMapPlugin,
            dashboard::Plugin,
        ))
        .insert_resource(ClearColor(Color::rgb_linear(0.153, 0.682, 0.376)))
        .insert_resource(args)
        .add_systems(Startup, (spawn_camera, load_maps))
        .add_systems(OnEnter(GameState::Game), hide_cursor)
        .add_systems(
            OnEnter(GameState::NextLevel),
            (
                despawn_level,
                load_maps.after(despawn_level),
                set_game_state,
            ),
        )
        .add_systems(OnEnter(GameState::Menu), show_cursor)
        .add_systems(
            Update,
            (
                trigger_menu,
                handle_human_player.run_if(in_state(GameState::Game)),
                handle_ai_players.run_if(in_state(GameState::Game)),
                handle_lap_counter,
                physics::apply_velocity
                    .after(handle_ai_players)
                    .after(handle_human_player),
                physics::apply_friction.after(physics::apply_velocity),
                track_player.after(physics::apply_velocity),
                physics::collision_detection
                    .after(physics::apply_velocity)
                    .after(handle_human_player)
                    .after(handle_ai_players),
                physics::fixed_collision_detection.after(physics::collision_detection),
            ),
        )
        .run();
}

pub fn despawn_query<T: Component>(q: Query<Entity, With<T>>, commands: &mut Commands) {
    for entity in &q {
        log::debug!("Despawning {entity:?}");
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_level(level_entities: Query<Entity, With<LevelComponent>>, mut commands: Commands) {
    log::info!("Despawning current level");
    despawn_query(level_entities, &mut commands);
}

fn show_cursor(mut q: Query<&mut Window>) {
    for mut w in q.iter_mut() {
        w.cursor.visible = true;
    }
}

fn hide_cursor(mut q: Query<&mut Window>) {
    for mut w in q.iter_mut() {
        w.cursor.visible = false;
    }
}

#[derive(Component, Debug)]
struct LapCounter(u32);

#[derive(Component, Debug)]
struct LevelComponent;

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug, Default)]
struct Racer {
    lap_count: u32,
    sub_count: u32,
    start_finish: u32,

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

fn load_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut prefs: ResMut<Preferences>,
) {
    let name = format!("level{}", prefs.level);
    log::info!("Spawning objects for {name}");
    prefs.level = if prefs.level == 2 { 1 } else { prefs.level + 1 };

    let map_handle: Handle<tilemap::TiledMap> =
        asset_server.load(format!("embedded://tdr2024/assets/{name}.tmx"));

    commands.spawn((
        LevelComponent,
        Name::new(name),
        tilemap::TiledMapBundle {
            tiled_map: map_handle,
            ..default()
        },
    ));
}

fn set_game_state(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);
}

fn trigger_menu(input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}

fn handle_human_player(
    mut query: Query<(
        &mut physics::Angle,
        &mut physics::Velocity,
        &mut Transform,
        &mut Racer,
        With<Player>,
    )>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let delta = time.delta_seconds();

    let Some((mut a, mut v, mut t, mut r, _)) = query.iter_mut().next() else {
        return;
    };

    if r.lap_count >= 5 {
        game_state.set(GameState::Menu);
        return;
    }

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
        v.0 += delta * 560.0 * Vec2::from_angle(a.0);
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
    let Some(guide) = guide else {
        return;
    };

    let delta = time.delta_seconds();

    for (mut a, mut v, mut t, mut r, _) in query.iter_mut() {
        if r.lap_count >= 5 {
            continue;
        }
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
            v.0 += delta * 580.0 * Vec2::from_angle(a.0);
        }

        a.normalize();
        t.rotation = a.to_quat();
    }
}

fn handle_lap_counter(
    checkpoints: Query<(&physics::ShapeBox, &LapCounter, &Transform)>,
    mut cars: Query<(&physics::CollisionBox, &mut Racer, &Transform)>,
    prefs: Res<Preferences>,
    mut gizmos: Gizmos,
) {
    if prefs.debug_low() {
        for (physics::ShapeBox(poly), _, xform) in checkpoints.iter() {
            poly.transform(xform).draw(&mut gizmos);
        }
    }

    let mask = checkpoints
        .iter()
        .map(|(_, c, _)| c.0)
        .reduce(|a, b| a | b)
        .unwrap_or(0);

    for (physics::CollisionBox(poly), mut car, xform) in cars.iter_mut() {
        let car_box = poly.transform(xform);

        for (physics::ShapeBox(poly), LapCounter(bit), xform) in checkpoints.iter() {
            if car_box.is_touching(&poly.transform(xform)) {
                if car.start_finish == 0 {
                    car.start_finish = *bit;
                }
                car.sub_count |= *bit;
                if car.sub_count == mask && car.start_finish == *bit {
                    car.lap_count += 1;
                    car.sub_count = *bit;
                }
            }
        }
    }
}

fn track_player(
    player: Query<(&Transform, &physics::Velocity, With<Player>)>,
    mut camera: Query<(&mut Transform, With<Camera>, Without<Player>)>,
    prefs: Res<Preferences>,
) {
    for (txp, _, _) in player.iter() {
        for (mut txc, _, _) in camera.iter_mut() {
            txc.translation.x = txp.translation.x;
            txc.translation.y = txp.translation.y;
            if prefs.jenny_mode {
                txc.rotation = txp.rotation;
            }
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    Game,
    NextLevel,
    #[default]
    Menu,
}
