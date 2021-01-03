//! TDGame ECS implementation module

pub(crate) mod client;
mod common;
pub mod dbg;
pub mod ecs;
pub(crate) mod editor;

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
    pub use common::InitWorldPlug;
    pub use dbg::DbgPlugs;
    pub use ecs::tower::TowerPlug;

    use super::*;
}

/// Public exports module
///
/// Also imports plugins module
pub mod prelude {
    pub use super::dbg::logging::*;
    pub use super::plugins::*;
}
