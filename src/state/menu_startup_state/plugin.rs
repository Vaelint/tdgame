use super::resources::*;
use super::systems::*;
use super::StateMenuStartup;
/// Project startup state Bevy Plugin module
use bevy::prelude::*;

use crate::state::{AppStates, STAGE_LOADING};

/// Bevy plugin for project startup state
#[derive(Debug)]
pub struct StateMenuStartupPlugin;

impl Plugin for StateMenuStartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<StateMenuStartupResources>()
            .init_resource::<StateMenuStartupEnts>()
            .init_resource::<StateUiResources>()
            // FIXME control spawn order of buttons?
            // Add startup systems
            .on_state_enter(STAGE_LOADING, AppStates::Menu, spawn_sidebar.system())
            // Spawn example game sprite
            .on_state_enter(STAGE_LOADING, AppStates::Menu, spawn_sprite_main.system())
            // Add update systems
            .on_state_update(
                STAGE_LOADING,
                AppStates::Menu,
                StateMenuStartup::update.system(),
            )
            .on_state_update(STAGE_LOADING, AppStates::Menu, button_system.system())
            // Add exit systems
            .on_state_exit(
                STAGE_LOADING,
                AppStates::Menu,
                StateMenuStartup::kill.system(),
            );
    }
}
