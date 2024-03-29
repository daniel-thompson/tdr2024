// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{
    log,
    math::{vec2, vec3},
    prelude::*,
};
use std::f32::consts::PI;

use crate::{geometry::Polygon, physics, tilemap, LapCounter, LevelComponent, Player, Racer};

#[derive(Default)]
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_map_events,));
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
        log::info!("{:?}", &event);
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
    let mut shape_number = 0;

    for layer in map.layers().filter_map(|layer| layer.as_object_layer()) {
        for obj in layer.objects() {
            let Some(tile_data) = obj.tile_data() else {
                spawn_shape(map, &obj, shape_number, commands);
                shape_number += 1;
                continue;
            };

            let tiled::TilesetLocation::Map(tileset) = tile_data.tileset_location() else {
                error!("Tile data isn't using a map ID as the tileset location");
                continue;
            };

            let id = tile_data.id();
            let Some(tile) = map.tilesets()[*tileset].get_tile(id) else {
                error!("Tile id missing from tile data");
                continue;
            };

            let Some(image) = &tile.image else {
                error!("Tile image missing from tile data");
                continue;
            };

            spawn_object(map, &obj, image, commands, texture_atlas, asset_server);
        }
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
    let Some(img_src) = img.source.to_str() else {
        error!("Cannot convert image name");
        return;
    };
    let is_car = img_src.contains("car");
    let is_player = is_car && img_src.contains("red");

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

    let translation = vec3(
        obj.x - (((map.width * map.tile_width) as f32 - img.width as f32) / 2.0),
        -obj.y + (((map.height * map.tile_height) as f32 + img.height as f32) / 2.0),
        if is_car { 2.0 } else { 5.0 },
    );
    let rotation = Quat::from_rotation_z(-obj.rotation * PI / 4.0);

    // tiled rotates objects from the bottom-left but bevy rotates objects
    // from the centre. that means we need to fix up the translation.
    let shift = Vec3::from((sz / 2.0, 0.0));
    let restore = rotation.mul_vec3(shift);

    let mut path = std::path::PathBuf::from("embedded://");
    path.push(&img.source);

    let handle = asset_server.load(path.to_str().expect("tile_path is not UTF-8").to_string());
    let mut entity = commands.spawn((
        LevelComponent,
        physics::CollisionBox(polygon),
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(TextureAtlas::from_grid(handle, sz, 1, 1, None, None)),
            transform: Transform {
                translation: translation - shift + restore,
                rotation,
                scale: Vec3::ONE,
            },
            ..default()
        },
    ));

    if is_car {
        entity.insert((
            Racer::default(),
            physics::Angle((90.0 - obj.rotation) * PI / 4.0),
            physics::Velocity(Vec2::new(0.0, 0.0)),
        ));

        if is_player {
            entity.insert((Name::new("Human"), Player));
        } else {
            entity.insert(Name::new("AI"));
        }
    } else {
        entity.insert(Name::new("Scenery"));
    }
}

fn spawn_shape(map: &tiled::Map, obj: &tiled::Object, num: u32, commands: &mut Commands) {
    match obj.shape {
        tiled::ObjectShape::Rect { width, height } => {
            let sz = vec2(width, height);
            let bbox = Polygon::from_vec(&sz);

            let translation = vec3(
                obj.x - (((map.width * map.tile_width) as f32 - width as f32) / 2.0),
                -obj.y + (((map.height * map.tile_height) as f32 + height as f32) / 2.0) - height,
                0.0,
            );
            let rotation = Quat::from_rotation_z(-obj.rotation * PI / 4.0);
            let shift = Vec3::from((sz / 2.0, 0.0));
            let restore = rotation.mul_vec3(shift);
            let transform = Transform {
                translation: translation - shift + restore,
                rotation,
                scale: Vec3::ONE,
            };

            commands.spawn((
                Name::new("Checkpoint"),
                LapCounter(1 << num),
                LevelComponent,
                physics::ShapeBox(bbox),
                transform,
            ));
        }
        _ => {
            error!("Unsupported shape: {:?}", (&obj.name, &obj.shape));
        }
    }
}
