//! General debug systems module

use bevy::app::{PluginGroup, PluginGroupBuilder};

// Public Exports
#[cfg(logging)]
pub use logging::*;

// Universal Modules
mod basic;
pub mod sprite_dbg;
pub mod world_dbg;

// Optional Modules
#[cfg(logging)]
pub mod logging;

/// Collection of debug plugins
///
/// A Bevy PluginGroup for general debugging
pub struct DbgPlugs;

impl PluginGroup for DbgPlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(basic::BasicDbgPlug)
            .add(sprite_dbg::SpriteDbgPlug);
    }
}
