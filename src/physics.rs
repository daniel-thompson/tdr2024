// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#![allow(clippy::type_complexity)]

use bevy::{math::vec2, prelude::*};
use slicetools::*;
use std::f32::consts::PI;

use crate::{mapping, util::IteratorToArrayExt, Preferences};

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

pub struct CollisionBox {
    points: [Vec2; 8],
}

impl CollisionBox {
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

    /// Test whether two rectangles are touching.
    pub fn is_touching(&self, other: &CollisionBox) -> bool {
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
        gizmos.line_2d(self.points[7], self.points[0], Color::BLUE);
    }
}

pub fn collision_detection(
    mut query: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Velocity)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    prefs: Res<Preferences>,
    mut gizmos: Gizmos,
) {
    let mut colliders = query.iter_mut().collect::<Vec<_>>();
    let mut it = colliders.pairs_mut();
    while let Some((a, b)) = it.next() {
        let atx = match texture_atlases.get(a.1) {
            Some(tx) => tx,
            None => continue,
        };
        let btx = match texture_atlases.get(b.1) {
            Some(tx) => tx,
            None => continue,
        };

        let mut abox = CollisionBox::from_transform(&a.0, &atx.size);
        let mut bbox = CollisionBox::from_transform(&b.0, &btx.size);
        if prefs.debug_low() {
            abox.draw(&mut gizmos);
            bbox.draw(&mut gizmos);
        }

        if abox.is_touching(&bbox) {
            std::mem::swap(&mut a.2 .0, &mut b.2 .0);

            let a2 = vec2(a.0.translation.x, a.0.translation.y);
            let b2 = vec2(b.0.translation.x, b.0.translation.y);
            let nudge = Vec3::from(((b2 - a2).normalize() * 0.5, 0.0));
            while abox.is_touching(&bbox) {
                a.0.translation -= nudge;
                b.0.translation += nudge;

                abox = CollisionBox::from_transform(&a.0, &atx.size);
                bbox = CollisionBox::from_transform(&b.0, &btx.size);
            }
        }
    }
}
