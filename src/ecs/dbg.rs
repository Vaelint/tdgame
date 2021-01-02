//! General debug systems module

use bevy::app::{AppBuilder, PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

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
            app.add_system(print_hello_system.system())
                .add_system(print_world_system.system());
        }
    }

    fn print_hello_system() {
        println!("hello");
    }

    fn print_world_system() {
        println!("world");
    }
}
