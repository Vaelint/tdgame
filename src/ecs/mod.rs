//! TDGame ECS implementation module

mod components;
mod dbg;

/// Plugin rexport module
///
/// Makes it easier to import the plugins in this ecs module.
/// Done by rexporting all the plugins in this plugin module.
///
/// # Examples:
/// Allows use of:
/// ```
/// use ecs::plugins::{DbgPlugs, ...};
/// ```
/// instead of:
/// ```
/// use ecs::dbg::DbgPlugs;
/// use ecs::etc::...;
/// ```
///
pub mod plugins {
    use super::*;

    pub use components::tower::TowerPlug;
    pub use dbg::DbgPlugs;
}
