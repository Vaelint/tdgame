use crate::state::{AppStates, STAGE_MENU};
/// Project startup menu state module
use bevy::prelude::*;

/// Bevy State for project startup screen
pub struct StateMenuStartup;

/// Bevy plugin for project startup state
pub struct StateMenuStartupPlugin;

/// Resources for project startup state
pub struct StateMenuStartupResources;

impl StateMenuStartup {
    fn spawn(com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // Setup the game world
        super::world::setup_world(com);
    }
    fn update(_com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // Do nothing
    }
    fn kill(_com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // Do nothing
    }
}
impl Plugin for StateMenuStartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<StateMenuStartupResources>()
            .on_state_enter(
                STAGE_MENU,
                AppStates::Menu,
                StateMenuStartup::spawn.system(),
            )
            .on_state_update(
                STAGE_MENU,
                AppStates::Menu,
                StateMenuStartup::update.system(),
            )
            .on_state_exit(STAGE_MENU, AppStates::Menu, StateMenuStartup::kill.system());
    }
}

impl FromResources for StateMenuStartupResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let mut res_mat_clr = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_srv = resources.get_mut::<AssetServer>().unwrap();

        Self {}
    }
}
