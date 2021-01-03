//! Editor specific core functionality mod

use bevy::prelude::*;

use init::*;

use super::dbg::*;

pub(crate) mod init;

pub mod prelude {
    pub use super::init::*;
}
