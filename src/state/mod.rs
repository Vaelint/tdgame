use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Project scenes module
///
/// Implementation of Bevy Scenes for project
mod loading_state;

/// Plugin group for whole module
pub struct ProjectStatePlugs;

/// Plugin adding the game's states
pub struct GameStatePlug;

/// Name of loading stage
pub const STAGE_LOADING: &'static str = "ST_LOAD";

/// Game state enum
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum AppStates {
    Menu,
    Load,
    Game,
}

impl PluginGroup for ProjectStatePlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(GameStatePlug).add(loading_state::LoadStatePlugin);
    }
}

impl Plugin for GameStatePlug {
    fn build(&self, app: &mut AppBuilder) {
        // Start application in load state.
        app.add_resource(State::new(AppStates::Load))
            .add_stage_after(
                stage::UPDATE,
                STAGE_LOADING,
                StateStage::<AppStates>::default(),
            );
    }
}
