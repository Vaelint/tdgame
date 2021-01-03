//! Prototype tower defense game written in bevy

// TODO setup build system to run clippy without cfg
#![warn(clippy::all)]

use bevy::prelude::*;

use tdcore::prelude::*;

mod tdcore;

fn main() {
    // Init logging
    init_logger();

    // Start Bevy App
    let mut app_builder = App::build();

    // Add core Plugins
    app_builder
        .add_plugins(DefaultPlugins)
        .add_plugin(TowerPlug)
        .add_plugin(InitWorldPlug);

    // Add debugging plugins
    #[cfg(debug_assertions)]
    app_builder.add_plugins(DbgPlugs);

    // Run app
    app_builder.run();
}
