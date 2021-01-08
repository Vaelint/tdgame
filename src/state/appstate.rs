use bevy::prelude::*;

/// Trait for implementing the logic needed for a simple state in bevy
pub trait AppState<S: Resource> {
    fn create(com: &mut Commands, res: Res<'_, S>);
    fn update(com: &mut Commands, res: Res<'_, S>);
    fn destroy(com: &mut Commands, res: Res<'_, S>);
}

/// Game state enum
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum AppStates {
    Menu,
    Load,
    Game,
}

impl Default for AppStates {
    fn default() -> Self {
        AppStates::Load
    }
}
