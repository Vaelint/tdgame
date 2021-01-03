//! Common initialization code

use bevy::prelude::*;

use crate::tdcore::prelude::*;

pub struct InitWorldPlug;

impl Plugin for InitWorldPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_world.system())
            .add_startup_system(init_camera.system())
            .add_startup_system(init_ui_camera.system());
    }

    fn name(&self) -> &str {
        "Init World Plugin"
    }
}

/// Initializes editor app
fn init_world(commands: &mut Commands) {
    // Initialize logger
    init_logger();
}

/// Initializes 2D camera
fn init_camera(commands: &mut Commands) {
    // Spawn Game Camera
    commands.spawn(Camera2dBundle::default());
}

/// Initializes UI Camera
fn init_ui_camera(commands: &mut Commands) {
    // Spawn UI Camera
    commands.spawn(CameraUiBundle::default());
}
