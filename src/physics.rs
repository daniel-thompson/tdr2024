// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{math::vec2, prelude::*};
use slicetools::*;
use std::f32::consts::PI;

use crate::{geometry::*, mapping, Preferences};

#[derive(Component, Debug, Reflect)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Debug, Reflect)]
pub struct Angle(pub f32);

#[derive(Component, Clone, Debug)]
pub struct CollisionBox(pub Polygon);

impl Angle {
    pub fn normalize(&mut self) {
        while self.0 > PI {
            self.0 -= 2.0 * PI;
        }
        while self.0 < -PI {
            self.0 += 2.0 * PI;
        }
    }

    pub fn to_quat(&self) -> Quat {
        Quat::from_rotation_z(self.0 - PI / 2.0)
    }
}

pub fn apply_friction(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    time: Res<Time>,
    guide: Option<Res<mapping::GuidanceField>>,
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

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (v, mut t) in query.iter_mut() {
        t.translation.x += delta * v.0.x;
        t.translation.y += delta * v.0.y;
    }
}

pub fn collision_detection(
    mut query: Query<(&CollisionBox, &mut Transform, &mut Velocity)>,
    prefs: Res<Preferences>,
    mut gizmos: Gizmos,
) {
    let mut colliders = query.iter_mut().collect::<Vec<_>>();
    let mut pairs = colliders.pairs_mut();
    // pairs_mut() does not return an iterator (due to borrowing rules) but we
    // create a similar loop using while-let
    while let Some(((CollisionBox(apoly), atf, av), (CollisionBox(bpoly), btf, bv))) = pairs.next()
    {
        let mut abox = apoly.transform(&atf);
        let mut bbox = bpoly.transform(&btf);
        if prefs.debug_low() {
            abox.draw(&mut gizmos);
            bbox.draw(&mut gizmos);
        }

        if abox.is_touching(&bbox) {
            std::mem::swap(&mut av.0, &mut bv.0);

            let a2 = vec2(atf.translation.x, atf.translation.y);
            let b2 = vec2(btf.translation.x, btf.translation.y);
            let nudge = Vec3::from(((b2 - a2).normalize() * 0.5, 0.0));
            while abox.is_touching(&bbox) {
                atf.translation -= nudge;
                btf.translation += nudge;

                abox = apoly.transform(&atf);
                bbox = bpoly.transform(&btf);
            }
        }
    }
}

pub fn fixed_collision_detection(
    mut cars: Query<(&CollisionBox, &mut Transform, &mut Velocity)>,
    scenery: Query<(&CollisionBox, &mut Transform, Without<Velocity>)>,
    _prefs: Res<Preferences>,
    mut _gizmos: Gizmos,
) {
    for (CollisionBox(car_poly), mut car_tf, mut car_vel) in cars.iter_mut() {
        let mut car_box = car_poly.transform(&car_tf);

        for (CollisionBox(obj_poly), obj_tf, _) in scenery.iter() {
            let obj_box = obj_poly.transform(&obj_tf);

            // This can be a single if/let
            if car_box.shape.iter().any(|pt| obj_box.contains_point(*pt)) {
                //car_vel.0 = vec2(-car_vel.0.x, -car_vel.0.y);
                let pt = car_box
                    .shape
                    .iter()
                    .find(|pt| obj_box.contains_point(**pt))
                    .unwrap();
                let line = obj_box.closest_edge_to_point(*pt);
                car_vel.0 = reflect_against_line(car_vel.0, line);

                while car_box.is_touching(&obj_box) {
                    car_tf.translation += Vec3::from((car_vel.0.normalize(), 0.0));
                    car_box = car_poly.transform(&car_tf);
                }
            } else if obj_box.shape.iter().any(|pt| car_box.contains_point(*pt)) {
                car_vel.0 = vec2(-car_vel.0.x, -car_vel.0.y);

                while car_box.is_touching(&obj_box) {
                    car_tf.translation += Vec3::from((car_vel.0.normalize(), 0.0));
                    car_box = car_poly.transform(&car_tf);
                }
            }
        }
    }
}
