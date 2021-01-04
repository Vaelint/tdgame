use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use std::fs::File;
use std::io::Write;

use crate::coremod::ecs::simple::DebugSwitch;

pub struct BasicDbgPlug;

impl Plugin for BasicDbgPlug {
    fn build(&self, app: &mut AppBuilder) {
        // Universal Plugins
        app.add_startup_system(hello_world.system())
            .add_startup_system(hello_log.system());
    }

    fn name(&self) -> &str {
        "Basic Debug Plugin"
    }
}

fn hello_world() {
    println!("Hello, World!");
}

fn hello_log() {
    info!("Hello, Log!");
}
