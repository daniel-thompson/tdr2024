// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{
    log,
    math::{vec2, vec3},
    prelude::*,
};
use std::f32::consts::PI;

use crate::{geometry::Polygon, physics, tilemap, Player, Racer};

#[derive(Default)]
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_map_events,));
    }
}

fn spawn_object(
    map: &tiled::Map,
    obj: &tiled::Object,
    img: &tiled::Image,
    commands: &mut Commands,
    texture_atlas: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
) {
    let img_src = img.source.to_str().unwrap();

    let sz = vec2(img.width as f32, img.height as f32);
    let polygon = if img_src.contains("tree") {
        Polygon::from_vec_with_rounding(&(sz * 0.5), 40.)
    } else if img_src.contains("tires") {
        Polygon::from_vec_with_rounding(&sz, 40.)
    } else if img_src.contains("car") {
        Polygon::from_vec_with_rounding(&sz, 60.)
    } else {
        Polygon::from_vec(&sz)
    };

    let (w, h) = (
        (map.width * map.tile_width) as f32,
        (map.height * map.tile_height) as f32,
    );

    // tiled rotates objects from the bottom-left but bevy rotates objects
    // from the centre. that means we need to fix up the translation.
    let rotation = Quat::from_rotation_z(-obj.rotation * PI / 4.0);
    let shift = Vec3::from((sz / 2.0, 0.0));
    let restore = rotation.mul_vec3(shift);
    let translation = vec3(
        obj.x - ((w - img.width as f32) / 2.0),
        -obj.y + ((h + img.height as f32) / 2.0),
        2.0,
    ) - shift
        + restore;

    let mut path = std::path::PathBuf::from("embedded://");
    path.push(&img.source);

    let handle = asset_server.load(path.to_str().expect("tile_path is not UTF-8").to_string());

    if img_src.contains("car") {
        if img_src.contains("red") {
            commands.spawn((
                Player,
                Racer::default(),
                physics::Angle(PI / 12.0),
                physics::CollisionBox(polygon.clone()),
                physics::Velocity(Vec2::new(0.0, 20.0)),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas
                        .add(TextureAtlas::from_grid(handle, sz, 1, 1, None, None)),
                    transform: Transform {
                        translation,
                        rotation,
                        scale: Vec3::splat(1.),
                    },
                    ..default()
                },
            ));
        } else {
            commands.spawn((
                Racer::default(),
                physics::Angle(PI / 12.0),
                physics::CollisionBox(polygon.clone()),
                physics::Velocity(Vec2::new(0.0, 20.0)),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas
                        .add(TextureAtlas::from_grid(handle, sz, 1, 1, None, None)),
                    transform: Transform {
                        translation,
                        rotation,
                        scale: Vec3::splat(1.),
                    },
                    ..default()
                },
            ));
        }
    } else {
        commands.spawn((
            physics::CollisionBox(polygon),
            SpriteSheetBundle {
                texture_atlas: texture_atlas
                    .add(TextureAtlas::from_grid(handle, sz, 1, 1, None, None)),
                transform: Transform {
                    translation: translation + vec3(0., 0., 3.),
                    rotation,
                    scale: Vec3::splat(1.),
                },
                ..default()
            },
        ));
    }
}

/// Grub about in the bowels of the tiled data, iterating over each
/// object and trying to figure out what sprite to create.
///
/// Once we finish wading through the tiled data we call out to
/// `spawn_object()` to do the bevy actions!
fn spawn_objects(
    map: &tiled::Map,
    commands: &mut Commands,
    texture_atlas: &mut Assets<TextureAtlas>,
    asset_server: &AssetServer,
) {
    let layer = map
        .layers()
        .find(|layer| layer.name == "Objects")
        .and_then(|layer| layer.as_object_layer());

    if let Some(layer) = layer {
        for object in layer.objects() {
            let tile_data = object.tile_data();
            if tile_data.is_none() {
                log::error!("Tile data is missing");
                continue;
            }
            let tile_data = tile_data.unwrap();

            let tileset = tile_data.tileset_location();
            if let tiled::TilesetLocation::Map(tileset) = tileset {
                let id = tile_data.id();
                let tile = map.tilesets()[*tileset].get_tile(id);

                if let Some(tile) = tile {
                    if let Some(image) = &tile.image {
                        spawn_object(map, &object, image, commands, texture_atlas, asset_server);
                    } else {
                        log::error!("Tile image missing from tile data");
                    }
                } else {
                    log::error!("Tile id missing from tile data");
                }
            } else {
                log::error!("Tile data isn't using a map ID as the tileset location");
            }
        }
    }
}

pub fn handle_map_events(
    mut map_events: EventReader<AssetEvent<tilemap::TiledMap>>,
    maps: Res<Assets<tilemap::TiledMap>>,
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                if let Some(map) = maps.get(*id) {
                    spawn_objects(&map.map, &mut commands, &mut texture_atlas, &asset_server);
                }
            }
            _ => continue,
        }
    }
}
