// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2024 Daniel Thompson

use bevy::asset::embedded_asset;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let p = if cfg!(windows) { "src\\" } else { "src/" };

        embedded_asset!(app, p, "level1.tmx");
        embedded_asset!(app, p, "level2.tmx");

        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_red_5.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_blue_1.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_yellow_3.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Cars/car_green_4.png");

        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt01.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt02.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt03.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt04.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt05.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt06.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt07.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt08.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt09.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt10.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt11.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt12.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt13.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt14.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt15.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt16.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt17.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt18.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt19.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt20.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt21.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt22.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt23.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt24.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt25.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt26.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt27.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt28.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt29.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt30.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt31.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt32.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt33.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt34.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt35.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt36.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt37.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt38.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt39.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt40.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt41.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt42.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt43.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt44.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt45.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt46.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt47.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt48.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt49.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt50.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt51.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt52.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt53.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt54.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt55.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt56.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt57.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt58.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt59.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt60.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt61.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt62.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt63.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt64.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt65.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt66.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt67.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt68.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt69.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt70.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt71.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt72.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt73.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt74.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt75.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt76.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt77.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt78.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt79.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt80.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt81.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt82.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt83.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt84.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt85.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt86.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt87.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt88.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt89.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Asphalt road/road_asphalt90.png"
        );

        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt01.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt02.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt03.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt04.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt05.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt06.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt07.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt08.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt09.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt10.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt11.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt12.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt13.png");
        embedded_asset!(app, p, "kenney_racing-pack/PNG/Tiles/Dirt/land_dirt14.png");

        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt01.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt02.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt03.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt04.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt05.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt06.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt07.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt08.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt09.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt10.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt11.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt12.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt13.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt14.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt15.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt16.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt17.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt18.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt19.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt20.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt21.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt22.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt23.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt24.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt25.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt26.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt27.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt28.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt29.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt30.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt31.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt32.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt33.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt34.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt35.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt36.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt37.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt38.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt39.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt40.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt41.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt42.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt43.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt44.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt45.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt46.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt47.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt48.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt49.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt50.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt51.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt52.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt53.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt54.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt55.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt56.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt57.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt58.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt59.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt60.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt61.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt62.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt63.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt64.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt65.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt66.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt67.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt68.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt69.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt70.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt71.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt72.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt73.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt74.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt75.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt76.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt77.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt78.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt79.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt80.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt81.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt82.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt83.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt84.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt85.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt86.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt87.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt88.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt89.png"
        );
        embedded_asset!(
            app,
            p,
            "kenney_racing-pack/PNG/Tiles/Dirt road/road_dirt90.png"
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
