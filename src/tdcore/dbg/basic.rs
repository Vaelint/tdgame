use bevy::reflect::TypeRegistry;

use super::*;

pub struct BasicDbgPlug;

impl Plugin for BasicDbgPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(hello_world.system())
            .add_startup_system(hello_log.system())
            .add_default_stages()
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
    let mut handle = File::create("res/levels/dbgworld.ron").unwrap();
    write!(handle, "{}", scene_data);
}
