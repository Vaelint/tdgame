//! Game state management and implementation module

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub use stages::*;

/// Project scenes module
///
/// Implementation of Bevy Scenes for project
mod loading_state;

/// Project main / startup menu state for project
mod menu_startup_state;

/// Plugin group for whole module
pub struct ProjectStatePlugs;

/// Plugin adding the game's states
pub struct GameStatePlug;

/// Stages module
#[allow(unused)]
mod stages {
    /// Name of loading stage
    pub const STAGE_LOADING: &str = "ST_LOAD";
    /// Name of menu state
    pub const STAGE_MENU: &str = "ST_MENU";
    /// Name of main gameplay state
    pub const STATE_GAME: &str = "ST_GAME";
}

/// Game state enum
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum AppStates {
    Menu,
    Load,
    Game,
}

/// Material handles for state buttons
pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

impl PluginGroup for ProjectStatePlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(GameStatePlug)
            .add(loading_state::LoadStatePlugin)
            .add(menu_startup_state::StateMenuStartupPlugin);
    }
}

impl Plugin for GameStatePlug {
    fn build(&self, app: &mut AppBuilder) {
        // Start application in load state.
        app.add_resource(State::new(AppStates::Load))
            .init_resource::<ButtonMaterials>()
            .add_stage_after(
                stage::UPDATE,
                STAGE_LOADING,
                StateStage::<AppStates>::default(),
            );
    }
}
