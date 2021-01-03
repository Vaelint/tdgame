//! General debug systems module

use std::fs::File;
use std::io::Write;

use bevy::app::{AppBuilder, PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use log::info;

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
