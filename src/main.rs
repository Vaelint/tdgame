//! Prototype tower defense game written in bevy

// TODO setup build system to run clippy without cfg
#![warn(clippy::all)]
#![allow(unused)]

use std::fs::File;

use bevy::prelude::*;
use simplelog::*;

use ecs::plugins;

mod ecs;

fn main() {
    // Setup logger
    #[cfg(logging)]
        CombinedLogger::init(vec![
        // Terminal Output
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        // File Output
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("debug.log").unwrap(),
        ),
    ])
    .unwrap();

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
