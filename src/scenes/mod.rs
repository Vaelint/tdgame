use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Project scenes module
///
/// Implementation of Bevy Scenes for project
mod loading;

pub struct ProjectScenePlugs;

impl PluginGroup for ProjectScenePlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(loading::LoadScreenPlug);
    }
}
