//! General ECS impl module

pub use plugins::*;
pub use simple::*;
pub use tower::*;
pub use window::*;

mod simple;
mod tower;
mod window;

pub mod plugins {
    use bevy::app::{PluginGroup, PluginGroupBuilder};

    use super::*;

    /// ECS Components plugin group
    pub struct ProjectECSPlugins;

    // Add plugins to plugin group
    impl PluginGroup for ProjectECSPlugins {
        fn build(&mut self, group: &mut PluginGroupBuilder) {
            group
                .add(TowerPlug)
                .add(ECSWindowPlug)
                .add(SimpleComponentsPlug);
        }
    }
}
