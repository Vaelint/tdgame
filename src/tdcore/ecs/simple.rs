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
        app.register_type::<Accumulator>()
            .register_type::<DebugSwitch>()
            .register_type::<DebugSwitch>();
    }
}

/// Accumulator component
#[derive(Debug, Display, From, Into, Constructor, Reflect, Default)]
#[reflect(Component)]
pub struct Accumulator(pub isize);

/// Debug switch component
#[derive(Debug, Display, From, Into, Constructor, Reflect)]
pub struct DebugSwitch(pub bool);

/// Entity enabling/disabling component
#[derive(Debug, Display, From, Into, Constructor, Reflect)]
pub struct Active(pub bool);
