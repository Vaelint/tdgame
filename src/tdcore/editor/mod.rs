//! Editor specific core functionality module

pub use init::EditorInitPlug;

pub mod init;
pub mod load_screen;

/// Public reexports module.
pub mod prelude {
    pub use super::init::*;
}
