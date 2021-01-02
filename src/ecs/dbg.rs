//! General debug systems module

use std::fs::File;
use std::io::Write;

use bevy::app::{AppBuilder, PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use log::info;

/// A group of plugins that produce the "hello world" behavior
pub struct DbgPlugs;

impl PluginGroup for DbgPlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(basic::BasicDbgPlug);
    }
}

mod basic {
    use bevy::reflect::TypeRegistry;

    use super::*;

    pub struct BasicDbgPlug;

    impl Plugin for BasicDbgPlug {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system(hello_world.system())
                .add_startup_system(hello_log.system())
                .add_startup_system(save_world.system());
        }
    }

    fn hello_world() {
        println!("Hello, World!");
    }

    fn hello_log() {
        info!("Hello, Log!");
    }

    fn save_world(world: &mut World, resources: &mut Resources) {
        // Get type registry
        let type_registry = resources.get::<TypeRegistry>().unwrap();
        // Get Dynamic Scene from world and type registry
        let scene = DynamicScene::from_world(&world, &type_registry);

        // Serialize scene
        let scene_data = scene.serialize_ron(&type_registry).unwrap();

        // Write scene to disk
        // TODO Don't hardcode path
        let mut handle = File::create("levels/dbgworld.ron").unwrap();
        write!(handle, "{}", scene_data);
    }
}
