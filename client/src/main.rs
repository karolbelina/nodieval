// Disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use bevy::prelude::{App, ClearColor, Color, WindowDescriptor, Msaa};
use bevy::DefaultPlugins;
use game::GamePlugin;

#[cfg(target_arch = "wasm32")]
use full_viewport::FullViewportPlugin;

#[cfg(target_arch = "wasm32")]
mod full_viewport;

mod game;

fn main() {
    let mut app = App::build();
    app
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "Nodieval".to_owned(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);

    #[cfg(target_arch = "wasm32")]
    app
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(FullViewportPlugin);

    app.run();
}
