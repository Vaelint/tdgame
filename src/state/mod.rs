use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod appstate;
/// Project scenes module
///
/// Implementation of Bevy Scenes for project
mod loading_state;
mod plugin;

pub struct ProjectStatePlugs;

impl PluginGroup for ProjectStatePlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(plugin::GameStatePlug)
            .add(loading_state::LoadStatePlugin);
    }
}

pub const STAGE_LOADING: &'static str = "ST_LOAD";
