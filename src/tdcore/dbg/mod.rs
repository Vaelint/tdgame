//! General debug systems module

use bevy::app::{PluginGroup, PluginGroupBuilder};

mod basic;
pub mod logging;

/// Collection of debug plugins
///
/// A Bevy PluginGroup for general debugging
pub struct DbgPlugs;

impl PluginGroup for DbgPlugs {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(basic::BasicDbgPlug);
    }
}
