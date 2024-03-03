// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

use bevy::prelude::*;
use std::convert::AsMut;
use std::default::Default;

/// Extension trait to allow arrays to be created from an iterator.
///
/// The code is heavily inspired by
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d246edd7c9619a82aaa0ab06a5f9d3fa
/// (from a pseudonymous contribution by
/// [liliii1](https://stackoverflow.com/users/3476782/1i1iii1)).
pub trait IteratorToArrayExt<T, U: Default + AsMut<[T]>>: Sized + Iterator<Item = T> {
    /// Capture in iterator into an array.
    ///
    /// Technically speaking to_array() can be used with anything that can provide
    /// a mutable reference to a slice. However in practice this is useful as a way
    /// to construct/initialize an array from an iterator!
    fn to_array(mut self) -> U {
        let mut array: U = U::default();
        for elem in array.as_mut() {
            match self.next() {
                None => panic!("Iterator doesn't have enough items"),
                Some(v) => *elem = v,
            }
        }
        assert!(self.next().is_none(), "Iterator has too many items");
        array
    }
}

impl<T, U: Iterator<Item = T>, V: Default + AsMut<[T]>> IteratorToArrayExt<T, V> for U {}

/// Recursively despawn all entities that match the generic parameter, T.
pub fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
