//! Prototype tower defense game written in bevy

#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(missing_docs, missing_debug_implementations)]

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

use ecs::ProjectECSPlugins;
use state::ProjectStatePlugs;

mod ecs;
mod stage;
mod state;

fn main() {
    // Start Bevy App
    App::build()
        // Add Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(ProjectECSPlugins)
        .add_plugins(ProjectStatePlugs)
        .run();
}
