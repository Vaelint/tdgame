use bevy::prelude::*;

/// Spawns loading screen's camera
pub fn setup_world(com: &mut Commands) {
    // Spawn 2D camera
    com.spawn(Camera2dBundle {
        /*
        camera: Default::default(),
        orthographic_projection: Default::default(),
        visible_entities: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
         */
        ..Default::default()
    });

    // Spawn UI camera
    com.spawn(CameraUiBundle {
        /*
        camera: Default::default(),
        orthographic_projection: Default::default(),
        visible_entities: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
         */
        ..Default::default()
    });
}
