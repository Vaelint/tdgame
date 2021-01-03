//! TDGame ECS implementation module

pub(crate) mod client;
mod common;
pub mod ecs;
pub(crate) mod editor;

// Optional deps
#[cfg(debug_assertions)]
pub mod dbg;

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
    #[cfg(debug_assertions)]
    pub use dbg::DbgPlugs;
    pub use ecs::tower::TowerPlug;
    pub use editor::EditorInitPlug;

    use super::*;
}

/// Public exports module
///
/// Also imports plugins module
pub mod prelude {
    #[cfg(logging)]
    pub use super::dbg::logging::*;
    pub use super::ecs::simple::*;
    pub use super::plugins::*;
}
