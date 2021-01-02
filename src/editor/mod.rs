/// Level editor module

use bevy::prelude::*;

pub struct LevelEditorPlug;

impl Plugin for LevelEditorPlug {
    fn build(&self, app: &mut AppBuilder) {
        unimplemented!()
    }
}

pub struct LevelEditor;

impl LevelEditor {
    fn init_world() {
        Self::init_camera();
        Self::init_ui()
    }

    fn init_camera() {
        unimplemented!();
    }

    fn init_ui() {
        unimplemented!();
    }
}