/// Project startup menu state module
use bevy::prelude::*;

use resources::StateMenuStartupResources;

mod plugin;
mod resources;
mod systems;

// Re-export plugins module
pub use plugin::StateMenuStartupPlugin;

/// Bevy State for project startup screen
#[derive(Debug)]
pub struct StateMenuStartup;

impl StateMenuStartup {
    fn update(_com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // Do nothing
    }
    fn kill(_com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // TODO clean up spawn ents
    }
}
