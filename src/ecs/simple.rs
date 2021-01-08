//! Simple bevy components plugin
//!
//! Used to wrap basic types withing structs for special functionality
//! that don't require complex implementation.

use bevy::prelude::*;
use derive_more::{Constructor, Display, From, Into};

pub struct SimpleComponentsPlug;

impl Plugin for SimpleComponentsPlug {
    fn build(&self, app: &mut AppBuilder) {
        // Register types for reflection
        app.register_type::<Rotating>();

        // Register component systems
        app.add_system(Rotating::rotate_sys.system());
    }
}

/// Entity enabling/disabling component
#[derive(Debug, Display, From, Into, Constructor, Reflect)]
pub struct Rotating(pub f32);

impl Rotating {
    /// Rotates an entity using provided speed
    ///
    /// # Note
    /// Positive is ccw & neg is cw rotation
    pub fn rotate_sys(time: Res<'_, Time>, mut query: Query<'_, (&Rotating, &mut Transform), ()>) {
        for (rot, mut trans) in query.iter_mut() {
            trans.rotate(Quat::from_rotation_z(rot.0 * time.delta_seconds()));
        }
    }
}
