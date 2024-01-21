// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2023-2024 Daniel Thompson

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        #[cfg(feature = "editor")]
        _app.add_plugins(bevy_editor_pls::prelude::EditorPlugin::default());
    }
}
