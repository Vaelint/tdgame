use bevy::prelude::*;

use crate::tdcore::prelude::*;

use super::prelude::*;

/// Bevy plugin for handling editor initialization
pub(crate) struct EditorInitPlug();

impl Plugin for EditorInitPlug {
    fn build(&self, app: &mut AppBuilder) {}
    fn name(&self) -> &str {
        "EditorInitPlugin"
    }
}
