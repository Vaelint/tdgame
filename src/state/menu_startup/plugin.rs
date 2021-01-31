use super::resources::*;
use super::systems::*;
use super::StateMenuStartup;
/// Project startup state Bevy Plugin module
use bevy::prelude::*;

use crate::state::{AppStates, STAGE_LOADING};

/// Bevy plugin for project startup state
#[derive(Debug)]
pub struct StateMenuStartupPlugin;

impl StateMenuStartupPlugin {
    /// Adds startup state's resources to an AppBuilder
    fn init_resources(app: &mut AppBuilder) -> &mut AppBuilder {
        app.init_resource::<StateMenuStartupResources>()
            .init_resource::<StateMenuStartupEnts>()
            .init_resource::<StateUiResources>()
    }

    /// Adds startup state's startup systems to an AppBuilder
    fn init_startup_systems(app: &mut AppBuilder) -> &mut AppBuilder {
        app.on_state_enter(STAGE_LOADING, AppStates::Menu, spawn_sidebar.system())
            // Spawn example game sprite
            .on_state_enter(STAGE_LOADING, AppStates::Menu, spawn_sprite_main.system())
    }

    /// Adds startup state's update systems to an AppBuilder
    fn init_update_systems(app: &mut AppBuilder) -> &mut AppBuilder {
        app
            // Add update systems
            .on_state_update(
                STAGE_LOADING,
                AppStates::Menu,
                StateMenuStartup::update.system(),
            )
            .on_state_update(STAGE_LOADING, AppStates::Menu, startup_butt_sys.system())
    }

    /// Adds startup state's exit systems to an AppBuilder
    fn init_exit_systems(app: &mut AppBuilder) -> &mut AppBuilder {
        app
            // Add exit systems
            .on_state_exit(
                STAGE_LOADING,
                AppStates::Menu,
                StateMenuStartup::kill.system(),
            )
    }
}

impl Plugin for StateMenuStartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        Self::init_resources(app);
        Self::init_startup_systems(app);
        Self::init_update_systems(app);
        Self::init_exit_systems(app);
    }
}
