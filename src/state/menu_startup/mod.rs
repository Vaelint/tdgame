mod assets;
mod background;
/// Project startup menu state module
pub mod exit_diag;
mod plugin;
pub mod sidebar;
mod styles;
mod txtbuilder;

// Re-export plugins module
pub use plugin::StateMenuStartupPlugin;

/// Bevy State for project startup screen
#[derive(Debug)]
pub struct StateMenuStartup;
