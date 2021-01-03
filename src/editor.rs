//! Level editor module

#![warn(clippy::all)]

use bevy::prelude::*;

use tdcore::prelude::*;

mod tdcore;

fn main() {
    // Create Bevy AppBuilder
    let mut app_builder = App::build();

    // Add core Plugins
    app_builder
        .add_plugins(DefaultPlugins)
        .add_plugin(TowerPlug)
        .add_plugin(InitWorldPlug);

    // Add Editor plugins
    app_builder.add_plugin(EditorInitPlug);

    // Add debugging plugins
    #[cfg(debug_assertions)]
    app_builder.add_plugins(DbgPlugs);

    // Build & run app
    app_builder.run();
}
