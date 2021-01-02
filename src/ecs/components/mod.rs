//! ECS Components module

pub mod simple;
pub mod tower;

pub mod plugins {
    use super::*;
    use bevy::app::{PluginGroup, PluginGroupBuilder};

    /// ECS Components plugin group
    pub struct ComponentPlugs;

    // Add plugins to plugin group
    impl PluginGroup for ComponentPlugs {
        fn build(&mut self, group: &mut PluginGroupBuilder) {
            group.add(tower::TowerPlug);
        }
    }
}
