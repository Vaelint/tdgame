//! TDGame ECS implementation module

mod components;
mod dbg;

/// Plugin reexport module
///
/// Makes it easier to import the plugins in this ecs module.
/// Done by reexporting all the plugins in this plugin module.
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
    pub use components::tower::TowerPlug;
    pub use dbg::DbgPlugs;

    use super::*;
}

pub mod prelude {
    pub use super::dbg::logging::*;
    pub use super::plugins;
}
