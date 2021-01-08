use bevy::prelude::*;

use crate::state::{appstate::AppStates, STAGE_LOADING};

/// Bevy plugin Which adds the gamestates in the GameState struct
pub struct GameStatePlug;

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
