//! Project State common functionality module

use bevy::prelude::*;

/// Spawns loading screen's camera
pub fn setup_state_world(commands: &mut Commands) -> (Entity, Entity) {
    // Spawn 2D camera
    commands.spawn(Camera2dBundle {
        /*
        camera: Default::default(),
        orthographic_projection: Default::default(),
        visible_entities: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
         */
        ..Default::default()
    });

    // Store ID of Main camera entity
    let cam_main = commands.current_entity().unwrap();

    // Spawn UI camera
    commands.spawn(CameraUiBundle {
        /*
        camera: Default::default(),
        orthographic_projection: Default::default(),
        visible_entities: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
         */
        ..Default::default()
    });

    // Return ent ID's as a tuple
    (cam_main, commands.current_entity().unwrap())
}
