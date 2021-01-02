//! Level editor module
#![warn(clippy::all)]

use bevy::prelude::*;
use tdcore::plugins;

mod tdcore;

pub struct LevelEditorPlug;

impl Plugin for LevelEditorPlug {
    fn build(&self, app: &mut AppBuilder) {
        unimplemented!()
    }
}

pub struct LevelEditor;

impl LevelEditor {
    fn init_world() {
        Self::init_camera();
        Self::init_ui()
    }

    fn init_camera() {
        unimplemented!();
    }

    fn init_ui() {
        unimplemented!();
    }
}

fn main() {
    tdcore::dbg::logging::init_logger();

    // Start Bevy App
    let mut app_builder = App::build();

    // Add core Plugins
    app_builder
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::TowerPlug);

    // Add debugging plugins
    #[cfg(debug_assertions)]
        app_builder.add_plugins(plugins::DbgPlugs);

    // Run app
    app_builder.run();
}