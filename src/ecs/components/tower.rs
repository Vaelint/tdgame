//! Tower implementation code

use bevy::app::AppBuilder;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use log::debug;

// Import simple ECS components
use super::simple::*;

/// Bevy plugin for Tower Module
pub struct TowerPlug;

impl Plugin for TowerPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Tower::spawn_test_towers.system())
            .add_system(Tower::log_towers.system())
            .register_type::<Tower>()
            .register_type::<TowerType>();
    }
}

/// Type of the tower
///
/// Used to specify what type of tower a tower entity is
#[derive(Debug, Clone, Reflect)]
pub enum TowerType {
    Dart,
}

impl Default for TowerType {
    fn default() -> Self {
        Self::Dart
    }
}

/// Tower Component
///
/// TODO Make less granular
#[derive(Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Tower {
    tower_type: TowerType,
}

impl Tower {
    /// Spawns a tower entity
    fn spawn_tower(commands: &mut Commands, tower: Tower) {
        #[cfg(not(debug_assertions))]
            commands.spawn((tower, 0));

        #[cfg(debug_assertions)]
            commands.spawn((tower, DebugSwitch::new(true)));
    }

    /// Spawns a default tower
    ///
    /// Used for debugging purposes
    fn spawn_test_towers(commands: &mut Commands) {
        Self::spawn_tower(commands, Self::default());
    }

    /// Print out the debug representation of all changed towers
    fn log_towers(query: Query<(&Tower, &DebugSwitch), Changed<Tower>>) {
        for tower in query.iter() {
            debug!("{:?}", tower);
        }
    }
}
