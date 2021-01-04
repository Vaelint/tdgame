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

mod stages {
    use bevy::app::{AppBuilder, Plugin};

    use names::*;
    use stages::*;

    /// Bevy plugin which adds project stages to app
    pub struct ProjStagePlug;

    impl Plugin for ProjStagePlug {
        fn build(&self, app: &mut AppBuilder) {
            app.add_stage(LOADING, LoadingStage)
                .add_stage(GAME, GameStage);
        }
    }

    mod names {
        /// The APP stage representing the loading screen
        pub const LOADING: &'static str = "PROJ_LOADING";
        /// App stage for game logic
        pub const GAME: &'static str = "PROJ_GAME";
    }

    /// Project Bevy stages module
    mod stages {
        use bevy::ecs::Stage;
        use bevy::prelude::*;

        pub struct LoadingStage;

        impl Stage for LoadingStage {
            fn initialize(&mut self, _world: &mut World, _resources: &mut Resources) {
                // Do nothing
            }

            fn run(&mut self, _world: &mut World, _resources: &mut Resources) {
                // Do nothing
            }
        }

        pub struct GameStage;

        impl Stage for GameStage {
            fn initialize(&mut self, _world: &mut World, _resources: &mut Resources) {
                // Do nothing
            }

            fn run(&mut self, _world: &mut World, _resources: &mut Resources) {
                // Do nothing
            }
        }
    }
}
