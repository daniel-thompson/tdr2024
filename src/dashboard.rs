// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use super::{physics, Player};
use bevy::prelude::*;

#[derive(Component, Debug)]
struct Speedometer;

#[derive(Default)]
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dashboard)
            .add_systems(Update, update_speedo);
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
}

fn update_speedo(
    player: Query<(&physics::Velocity, With<Player>)>,
    mut speedo: Query<(&mut Transform, With<Speedometer>)>,
) {
    let (vp, _) = match player.iter().next() {
        Some(t) => t,
        None => return,
    };
    let (mut needle, _) = speedo.single_mut();

    needle.rotation = Quat::from_rotation_z(vp.0.length() / 100.0);
}
