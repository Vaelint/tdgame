use bevy::app::{AppBuilder, Plugin};

use super::prelude::*;

fn init_world() {
    init_camera();
    init_ui();
}

fn init_camera() {
    unimplemented!();
}

fn init_ui() {
    unimplemented!();
}

/// Bevy plugin for handling editor initialization
pub(crate) struct EditorInitPlug();

impl Plugin for EditorInitPlug {
    fn build(&self, app: &mut AppBuilder) {
        unimplemented!()
    }
    fn name(&self) -> &str {
        "EditorInitPlugin"
    }
}
