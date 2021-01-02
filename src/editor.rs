//! Level editor module

#![warn(clippy::all)]

use bevy::prelude::*;

use tdcore::prelude::*;

mod tdcore;

fn init_world() {
    init_camera();
    init_ui();
}

fn init_camera() {
    unimplemented!();
}

fn init_ui() {
    unimplemented!();
}

fn main() {
    // Init logging
    init_logger();

    // Start Bevy App
    let mut app_builder = App::build();

    // Add core Plugins
    app_builder
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::TowerPlug);

    // Add debugging plugins
    #[cfg(debug_assertions)]
        app_builder.add_plugins(plugins::DbgPlugs);

    // Run app
    app_builder.run();
}