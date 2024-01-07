// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2024 Daniel Thompson

use bevy::asset::embedded_asset;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let p = if cfg!(windows) {
            "src\\"
        } else {
            "src/"
        };

        embedded_asset!(app, p, "level1.tmx");

        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_red_5.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_blue_1.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_yellow_3.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_green_4.png");
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/Spritesheets/spritesheet_tiles.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/Spritesheets/spritesheet_tiles.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/Spritesheets/spritesheet_tiles.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/Spritesheets/spritesheet_tiles.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass01.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass02.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass03.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass04.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass05.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass06.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass07.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass08.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass09.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass10.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass11.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass12.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass13.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Grass/land_grass14.png"
        );
    }
}
