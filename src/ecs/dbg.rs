//! General debug systems module

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
    use super::*;
    pub struct BasicDbgPlug;

    impl Plugin for BasicDbgPlug {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system(hello_world.system())
                .add_startup_system(hello_log.system());
        }
    }

    fn hello_world() {
        println!("Hello, World!");
    }

    fn hello_log() {
        info!("Hello, Log!");
    }
}
