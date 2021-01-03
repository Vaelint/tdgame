use std::fs::File;
use std::io::Write;

use bevy::prelude::*;
use bevy::reflect::TypeRegistry;

pub fn save_active_world(world: &mut World, resources: &mut Resources) {
    // Get type registry
    let type_registry = resources.get::<TypeRegistry>().unwrap();
    // Get Dynamic Scene from world and type registry
    let scene = DynamicScene::from_world(&world, &type_registry);

    // Serialize scene
    let scene_data = scene.serialize_ron(&type_registry).unwrap();

    // Write scene to disk
    // TODO Don't hardcode path
    let mut handle = File::create("lvls/dbgworld.ron").unwrap();
    let write = write!(handle, "{}", scene_data);

    match write {
        // TODO hardcoding
        Ok(_) => {
            info!("Successfully saved dbgworld.ron")
        }
        Err(_) => {
            error!("Failed to save dbgworld.ron")
        }
    }
}
