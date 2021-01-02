//! Simple bevy components plugin
//!
//! Used to wrap basic types withing structs for special functionality
//! that don't require complex implementation.

use derive_more::{Add, AsMut, AsRef, Constructor, Display, From, Into};

/// Accumulator component
#[derive(Debug, Display, From, Into, Constructor)]
pub struct Accumulator(isize);

/// Debug switch component
#[derive(Debug, Display, From, Into, Constructor)]
pub struct DebugSwitch(bool);

/// Entity enabling/disabling component
#[derive(Debug, Display, From, Into, Constructor)]
pub struct Active(bool);
