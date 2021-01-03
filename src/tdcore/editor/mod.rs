//! Editor specific core functionality module

pub use init::EditorInitPlug;

pub mod init;

/// Public reexports module.
pub mod prelude {
    pub use super::init::*;
}
