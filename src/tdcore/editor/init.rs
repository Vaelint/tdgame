// Editor initialization code

use bevy::prelude::*;

use super::load_screen;

/// Bevy plugin for handling editor initialization
pub struct EditorInitPlug;

impl Plugin for EditorInitPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_screen::setup.system())
            .init_resource::<load_screen::LoadScreenResources>();
    }
    fn name(&self) -> &str {
        "EditorInitPlugin"
    }
}
