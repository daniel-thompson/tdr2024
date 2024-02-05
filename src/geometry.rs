// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

use bevy::{math::vec2, prelude::*};
use itertools::Itertools;
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

pub fn reflect_against_segment(v: Vec2, segment: (Vec2, Vec2, Vec2)) -> Vec2 {
    let n1 = (segment.1 - segment.0).perp().normalize();
    let n2 = (segment.2 - segment.1).perp().normalize();
    let semi_normal = (n1 + n2).normalize();

    v - ((2.0 * v.dot(semi_normal)) * semi_normal)
}

/// A polygon, represented as a series of points.
///
/// In principle we could support any number of sides. However the internal
/// representation is private so only shapes supported by the factory
/// functions can ever exist. At present this means all shapes are either
/// rectangles or octagons (meaning the internal SmallVec is always allocated
/// on the stack.
///
/// Some of the algorithms used require that the polygon be convex. This
/// property is guarantees by all current factory functions.
#[derive(Clone, Debug)]
pub struct Polygon {
    shape: SmallVec<[Vec2; 8]>,
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
        assert!(sz.x > 0. && sz.y > 0.);
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
        assert!(sz.x > 0. && sz.y > 0.);
        assert!(percent > 0. && percent < 100.);
        let (w, h) = (sz.x / 2., sz.y / 2.);
        let m = w.min(h);
        let c = m - (0.01 * m * percent);

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
        self.iter_segments()
            .all(|(&a, &b, &c)| same_side(pt, a, (b, c)))
    }

    pub fn closest_edge_to_point(&self, pt: Vec2) -> (Vec2, Vec2) {
        self.iter_lines()
            .map(|(&a, &b)| (a, b))
            .min_by(|&a, &b| {
                distance_to_line(pt, a)
                    .partial_cmp(&distance_to_line(pt, b))
                    .expect("Floating point numbers must be comparable")
            })
            .expect("Shape must not be empty")
    }

    pub fn draw(&self, gizmos: &mut Gizmos) {
        for (&a, &b) in self.iter_lines() {
            gizmos.line_2d(a, b, Color::BLUE);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec2> + '_ {
        self.shape.iter()
    }

    pub fn iter_lines(&self) -> impl Iterator<Item = (&Vec2, &Vec2)> {
        let slice = self.shape.as_slice();
        slice.iter().chain(&slice[0..1]).tuple_windows()
    }

    pub fn iter_segments(&self) -> impl Iterator<Item = (&Vec2, &Vec2, &Vec2)> {
        let slice = self.shape.as_slice();
        slice.iter().chain(&slice[0..2]).tuple_windows()
    }

    pub fn _iter_mut(&mut self) -> std::slice::IterMut<'_, Vec2> {
        self.shape.iter_mut()
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
