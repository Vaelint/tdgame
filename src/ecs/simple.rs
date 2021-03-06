//! Simple bevy components plugin
//!
//! Used to wrap basic types withing structs for special functionality
//! that don't require complex implementation.

use bevy::prelude::*;
use derive_more::{Constructor, Display, From, Into};

pub struct BasicECSPlugin;

impl Plugin for BasicECSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Register types for reflection
        app.register_type::<Rotating>()
            .add_system(Rotating::sys_rotate.system());
    }
}

// Components which causes entities to rotate on z axis
#[derive(Debug, Display, From, Into, Constructor, Reflect, Default)]
#[reflect(Component)]
pub struct Rotating(pub f32);

impl Rotating {
    /// Rotates an entity using provided speed
    ///
    /// # Note
    ///  Positive is ccw & neg is cw rotation
    ///  Rotation is on the axis perpendicular to the screen
    pub fn sys_rotate(time: Res<'_, Time>, mut query: Query<'_, (&Rotating, &mut Transform), ()>) {
        for (rot, mut trans) in query.iter_mut() {
            trans.rotate(Quat::from_rotation_z(rot.0 * time.delta_seconds()));
        }
    }
}
