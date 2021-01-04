//! Prototype tower defense game written in bevy

// TODO setup build system to run clippy without cfg
#![warn(clippy::all)]

use bevy::prelude::*;

use ecs::plugins::ProjectECSPlugins;

mod ecs;
mod scenes;

fn main() {
    // Start Bevy App
    let mut app_builder = App::build();

    // Add core Plugins
    app_builder
        .add_plugins(DefaultPlugins)
        .add_plugins(ProjectECSPlugins);

    // Run app
    app_builder.run();
}
