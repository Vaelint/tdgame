//! Prototype tower defense game written in bevy

// TODO setup build system to run clippy without cfg
#![warn(clippy::all)]
#![allow(unused)]

use bevy::prelude::*;

mod ecs;

use ecs::plugins;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::DbgPlugs)
        .add_plugin(plugins::TowerPlug)
        .run();
}
