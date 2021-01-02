//! Prototype tower defense game written in bevy

// TODO setup build system to run clippy without cfg
#![warn(clippy::all)]
#![allow(unused)]

use bevy::prelude::*;
use ecs::plugins;
use simplelog::*;
use std::fs::File;

mod ecs;

fn main() {
    // Setup logger
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
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
    #[cfg(not(debug_assertions))]
    app_builder.add_plugins(plugins::DbgPlugs);

    // Run app
    app_builder.run();
}
