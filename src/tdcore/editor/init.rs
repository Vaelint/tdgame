use bevy::prelude::*;

/// Bevy plugin for handling editor initialization
pub(crate) struct EditorInitPlug();

impl Plugin for EditorInitPlug {
    fn build(&self, _app: &mut AppBuilder) {}
    fn name(&self) -> &str {
        "EditorInitPlugin"
    }
}
