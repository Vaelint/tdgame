//! Project startup state Bevy Plugin module

use crate::state::menu_startup::sidebar::SidebarEnts;
use bevy::prelude::*;

use crate::state::{AppStates, STAGE_LOADING};

use super::background::spawn_sprite_main;
use super::exit_diag::{event_exitconf_sys, ExitConfirmDiagEnts, ExitConfirmDiagEvent};
use super::resources::StartupMenuRes;
use super::sidebar::{button::startup_butt_sys, spawn::spawn_sidebar};
use super::styles::StateUiResources;

/// Bevy plugin for project startup state
#[derive(Debug)]
pub struct StateMenuStartupPlugin;

impl StateMenuStartupPlugin {
    /// Adds startup state's resources to an AppBuilder
    fn add_resources(app: &mut AppBuilder) -> &mut AppBuilder {
        app.init_resource::<StartupMenuRes>()
            .init_resource::<SidebarEnts>()
            .init_resource::<StateUiResources>()
            .init_resource::<ExitConfirmDiagEnts>()
    }

    /// Adds startup state's events to an AppBuilder
    fn add_events(app: &mut AppBuilder) -> &mut AppBuilder {
        app.add_event::<ExitConfirmDiagEvent>()
    }

    /// Adds startup state's startup systems to an AppBuilder
    fn add_startup_sys(app: &mut AppBuilder) -> &mut AppBuilder {
        app.on_state_enter(STAGE_LOADING, AppStates::Menu, spawn_sidebar.system())
            // Spawn example game sprite
            .on_state_enter(STAGE_LOADING, AppStates::Menu, spawn_sprite_main.system())
    }

    /// Adds startup state's update systems to an AppBuilder
    fn add_update_sys(app: &mut AppBuilder) -> &mut AppBuilder {
        app
            // Add update systems
            .on_state_update(STAGE_LOADING, AppStates::Menu, startup_butt_sys.system())
            .on_state_update(STAGE_LOADING, AppStates::Menu, event_exitconf_sys.system())
    }

    /// Adds startup state's exit systems to an AppBuilder
    fn add_exit_sys(app: &mut AppBuilder) -> &mut AppBuilder {
        app
    }
}

impl Plugin for StateMenuStartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        Self::add_resources(app);
        Self::add_events(app);
        Self::add_startup_sys(app);
        Self::add_update_sys(app);
        Self::add_exit_sys(app);
    }
}
