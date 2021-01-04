//! Prototype tower defense game written in bevy

use bevy::prelude::*;

use ecs::ProjectECSPlugins;
use scenes::ProjectScenePlugs;

mod ecs;
mod scenes;

fn main() {
    // Start Bevy App
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProjectECSPlugins)
        .add_plugins(ProjectScenePlugs)
        .run();
}
