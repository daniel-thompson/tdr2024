// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{math::vec2, prelude::*};
use slicetools::*;
use std::f32::consts::PI;

use crate::{mapping, objectmap, util::IteratorToArrayExt, Preferences};

#[derive(Component, Debug, Reflect)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Debug, Reflect)]
pub struct Angle(pub f32);

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

fn same_side(p1: Vec2, p2: Vec2, line: (Vec2, Vec2)) -> bool {
    let p1 = Vec3::from((p1, 0.0));
    let p2 = Vec3::from((p2, 0.0));
    let line = (Vec3::from((line.0, 0.0)), Vec3::from((line.1, 0.0)));

    let cp1 = (line.1 - line.0).cross(p1 - line.0);
    let cp2 = (line.1 - line.0).cross(p2 - line.0);

    cp1.dot(cp2) >= 0.0
}

/// Calculate the length of a line between two points.
///
/// This is is a simple application of the Pythagorean theorem.
fn length_of_line(line: (Vec2, Vec2)) -> f32 {
    ((line.1.x - line.0.x).powi(2) + (line.1.y - line.0.y).powi(2)).sqrt()
}

/// Calculate the area of a triangle defined by three points.
fn area_of_triangle(triangle: (Vec2, Vec2, Vec2)) -> f32 {
    (((triangle.0.x - triangle.2.x) * (triangle.1.y - triangle.0.y))
        - ((triangle.0.x - triangle.1.x) * (triangle.2.y - triangle.0.y)))
        .abs()
        / 2.0
}

/// Calculate the shortest distance from the point to a line.
fn distance_to_line(pt: Vec2, line: (Vec2, Vec2)) -> f32 {
    2.0 * area_of_triangle((pt, line.0, line.1)) / length_of_line(line)
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

fn closest_edge_to_point(pt: Vec2, shape: &[Vec2]) -> (Vec2, Vec2) {
    let n = shape.len();
    shape
        .windows(2)
        .chain(std::iter::once([shape[n - 1], shape[0]].as_slice()))
        .map(|line| (line[0], line[1]))
        .min_by(|a, b| {
            distance_to_line(pt, *a)
                .partial_cmp(&distance_to_line(pt, *b))
                .expect("Floating point numbers must be comparable")
        })
        .expect("Shape must not be empty")
}

fn reflect_against_line(v: Vec2, line: (Vec2, Vec2)) -> Vec2 {
    let normal = (line.1 - line.0).perp().normalize();

    v - ((2.0 * v.dot(normal)) * normal)
}

pub struct CollisionBox<const L: usize> {
    points: [Vec2; L],
}

impl CollisionBox<8> {
    pub fn from_transform(tf: &Transform, sz: &Vec2) -> Self {
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
}

impl<const L: usize> CollisionBox<L> {
    /// Test whether two rectangles are touching.
    pub fn is_touching<const M: usize>(&self, other: &CollisionBox<M>) -> bool {
        other
            .points
            .iter()
            .any(|pt| point_in_polygon(*pt, &self.points))
            || self
                .points
                .iter()
                .any(|pt| point_in_polygon(*pt, &other.points))
    }

    pub fn draw(&self, gizmos: &mut Gizmos) {
        for w in self.points.windows(2) {
            gizmos.line_2d(w[0], w[1], Color::BLUE);
        }
        gizmos.line_2d(self.points[L - 1], self.points[0], Color::BLUE);
    }
}

pub fn collision_detection(
    mut query: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Velocity)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    prefs: Res<Preferences>,
    mut gizmos: Gizmos,
) {
    let mut colliders = query.iter_mut().collect::<Vec<_>>();
    let mut pairs = colliders.pairs_mut();
    // pairs_mut() does not return an iterator (due to borrowing rules) but we
    // create a similar loop using while-let
    while let Some(((atf, atx, av), (btf, btx, bv))) = pairs.next() {
        let (atx, btx) = match (texture_atlases.get(*atx), texture_atlases.get(*btx)) {
            (Some(atx), Some(btx)) => (atx, btx),
            _ => continue,
        };

        let mut abox = CollisionBox::from_transform(&atf, &atx.size);
        let mut bbox = CollisionBox::from_transform(&btf, &btx.size);
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

                abox = CollisionBox::from_transform(&atf, &atx.size);
                bbox = CollisionBox::from_transform(&btf, &btx.size);
            }
        }
    }
}

pub fn fixed_collision_detection(
    mut cars: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Velocity)>,
    scenery: Query<(&mut Transform, &objectmap::Collider, Without<Velocity>)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    _prefs: Res<Preferences>,
    mut _gizmos: Gizmos,
) {
    for (mut car_tf, car_tx, mut car_vel) in cars.iter_mut() {
        let car_tx = match texture_atlases.get(car_tx) {
            Some(car_tx) => car_tx,
            _ => continue,
        };
        let mut car_box = CollisionBox::from_transform(&car_tf, &car_tx.size);

        for (obj_tf, collider, _) in scenery.iter() {
            let obj_box = match collider {
                objectmap::Collider::Tree => CollisionBox::<8> {
                    points: [
                        vec2(-25.0, 50.0),
                        vec2(25.0, 50.0),
                        vec2(50.0, 25.0),
                        vec2(50.0, -25.0),
                        vec2(25.0, -50.0),
                        vec2(-25.0, -50.0),
                        vec2(-50.0, -25.0),
                        vec2(-50.0, 25.0),
                    ]
                    .iter()
                    .map(|v2| {
                        let v3 = Vec3::from((*v2, 0.0));
                        let pt = obj_tf.transform_point(v3);
                        vec2(pt.x, pt.y)
                    })
                    .to_array(),
                },
                objectmap::Collider::Block => CollisionBox::<8> {
                    points: [
                        vec2(-224.0, 112.0),
                        vec2(0.0, 112.0),
                        vec2(224.0, 112.0),
                        vec2(224.0, 0.0),
                        vec2(224.0, -112.0),
                        vec2(0.0, -112.0),
                        vec2(-224.0, -112.0),
                        vec2(-224.0, 0.0),
                    ]
                    .iter()
                    .map(|v2| {
                        let v3 = Vec3::from((*v2, 0.0));
                        let pt = obj_tf.transform_point(v3);
                        vec2(pt.x, pt.y)
                    })
                    .to_array(),
                },
            };

            if car_box
                .points
                .iter()
                .any(|pt| point_in_polygon(*pt, &obj_box.points))
            {
                //car_vel.0 = vec2(-car_vel.0.x, -car_vel.0.y);
                let pt = car_box
                    .points
                    .iter()
                    .find(|pt| point_in_polygon(**pt, &obj_box.points))
                    .unwrap();
                let line = closest_edge_to_point(*pt, &obj_box.points);
                car_vel.0 = reflect_against_line(car_vel.0, line);

                while car_box.is_touching(&obj_box) {
                    car_tf.translation += Vec3::from((car_vel.0.normalize(), 0.0));
                    car_box = CollisionBox::from_transform(&car_tf, &car_tx.size);
                }
            } else if obj_box
                .points
                .iter()
                .any(|pt| point_in_polygon(*pt, &car_box.points))
            {
                car_vel.0 = vec2(-car_vel.0.x, -car_vel.0.y);

                while car_box.is_touching(&obj_box) {
                    car_tf.translation += Vec3::from((car_vel.0.normalize(), 0.0));
                    car_box = CollisionBox::from_transform(&car_tf, &car_tx.size);
                }
            }
        }
    }
}
