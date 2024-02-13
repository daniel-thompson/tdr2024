// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use super::{physics, Player, Racer};
use bevy::prelude::*;

#[derive(Component, Debug)]
struct LapMeter;

#[derive(Component, Debug)]
struct Speedometer;

#[derive(Default)]
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dashboard)
            .add_systems(Update, (update_lap_counter, update_speedo));
    }
}

fn spawn_dashboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                left: Val::Percent(-40.0),
                bottom: Val::Percent(-70.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::VMax(100.0 * 256.0 / 1920.0),
                                height: Val::VMax(100.0 * 256.0 / 1920.0),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load("embedded://tdr2024/assets/speeddial.png")),
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Speedometer,
                        NodeBundle {
                            style: Style {
                                width: Val::VMax(100.0 * 256.0 / 1920.0),
                                height: Val::VMax(100.0 * 256.0 / 1920.0),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(
                            asset_server.load("embedded://tdr2024/assets/speedneedle.png"),
                        ),
                    ));
                });
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                left: Val::Percent(40.0),
                bottom: Val::Percent(-75.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        LapMeter,
                        TextBundle::from_section(
                            "Lap 0",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    ));
                });
        });
}

fn update_lap_counter(
    player: Query<(&Racer, With<Player>)>,
    mut label: Query<(&mut Text, With<LapMeter>)>,
) {
    let Some((car, _)) = player.iter().next() else {
        return;
    };
    let (mut txt, _) = label.single_mut();

    // TODO: This is rather cumbersome but I'd rather not work the memory
    //       allocator every frame...
    let label = &txt.sections[0].value;
    let len = label.len();
    let value = label[len - 1..len].parse::<u32>();
    if let Ok(value) = value {
        if value != car.lap_count {
            txt.sections[0].value = format!("Lap {}", car.lap_count);
        }
    } else {
        error!("Invalid lap count: {label}");
        txt.sections[0].value = format!("Lap {}", car.lap_count);
    }
}

fn update_speedo(
    player: Query<(&physics::Velocity, With<Player>)>,
    mut speedo: Query<(&mut Transform, With<Speedometer>)>,
) {
    let Some((vp, _)) = player.iter().next() else {
        return;
    };
    let (mut needle, _) = speedo.single_mut();

    needle.rotation = Quat::from_rotation_z(vp.0.length() / 100.0);
}
