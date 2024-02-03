// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

use bevy::{math::vec2, prelude::*};
use smallvec::SmallVec;

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

pub fn reflect_against_line(v: Vec2, line: (Vec2, Vec2)) -> Vec2 {
    let normal = (line.1 - line.0).perp().normalize();

    v - ((2.0 * v.dot(normal)) * normal)
}

#[derive(Clone, Debug)]
pub struct Polygon {
    pub shape: SmallVec<[Vec2; 8]>,
}

impl FromIterator<Vec2> for Polygon {
    fn from_iter<I: IntoIterator<Item = Vec2>>(iter: I) -> Self {
        Self {
            shape: SmallVec::from_iter(iter),
        }
    }
}

impl Polygon {
    pub fn from_vec(sz: &Vec2) -> Self {
        let (w, h) = (sz.x / 2., sz.y / 2.);
        [vec2(-w, h), vec2(w, h), vec2(w, -h), vec2(-w, -h)]
            .into_iter()
            .collect()
    }

    /// Create an octagon from a single vector and a roundness factor.
    ///
    /// The roundness factor is, effectively, the percentage of the
    /// shortest edge that will be preserved on each side.
    pub fn from_vec_with_rounding(sz: &Vec2, percent: f32) -> Self {
        let (w, h) = (sz.x / 2., sz.y / 2.);
        let m = w.min(h);
        let c = m - (m * percent);

        [
            vec2(c - w, h),
            vec2(w - c, h),
            vec2(w, h - c),
            vec2(w, c - h),
            vec2(w - c, -h),
            vec2(c - w, -h),
            vec2(-w, c - h),
            vec2(-w, h - c),
        ]
        .into_iter()
        .collect()
    }

    pub fn contains_point(&self, pt: Vec2) -> bool {
        let shape = self.shape.as_slice();
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

    pub fn closest_edge_to_point(&self, pt: Vec2) -> (Vec2, Vec2) {
        let shape = self.shape.as_slice();
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

    pub fn draw(&self, gizmos: &mut Gizmos) {
        let shape = self.shape.as_slice();
        let n = shape.len();
        for w in shape.windows(2) {
            gizmos.line_2d(w[0], w[1], Color::BLUE);
        }
        gizmos.line_2d(shape[n - 1], shape[0], Color::BLUE);
    }

    /// Test whether two rectangles are touching.
    pub fn is_touching(&self, other: &Polygon) -> bool {
        other.shape.iter().any(|pt| self.contains_point(*pt))
            || self.shape.iter().any(|pt| other.contains_point(*pt))
    }

    pub fn transform(&self, tf: &Transform) -> Self {
        self.shape
            .iter()
            .map(|v2| {
                let v3 = Vec3::from((*v2, 0.0));
                let pt = tf.transform_point(v3);
                vec2(pt.x, pt.y)
            })
            .collect()
    }
}
