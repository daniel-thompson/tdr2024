// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{math::vec2, prelude::*};
use itertools::Itertools;

use crate::{tilemap, Player, Racer};

#[derive(Resource)]
pub struct GuidanceField {
    image: image::GrayImage,
}

impl GuidanceField {
    pub fn from_map(map: &tiled::Map) -> Option<Self> {
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

    pub fn get(&self, pos: &Vec2) -> i32 {
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

pub fn generate_guidance_field(
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

/// Track whether the racer has skipped more than one tile and apply a time
/// penalty if this is seen.
///
/// The `With<Player>` is temporary. We need it because the current guidance
/// system isn't able to navigate some courses (esp. the "level 1" development
/// level) when time penalties are applied.
pub fn apply_time_penalties(
    mut query: Query<(&mut Transform, &mut Racer, With<Player>)>,
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

    for (t, mut r, _) in query.iter_mut() {
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
