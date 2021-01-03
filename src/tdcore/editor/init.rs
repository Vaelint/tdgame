// Editor initialization code

use bevy::prelude::*;

/// Bevy plugin for handling editor initialization
pub struct EditorInitPlug;

impl Plugin for EditorInitPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_screen::setup.system());
    }
    fn name(&self) -> &str {
        "EditorInitPlugin"
    }
}

mod load_screen {
    use bevy::prelude::*;

    pub fn setup(
        commands: &mut Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // Load App Icon
        let texture_handle = asset_server.load("tex/icon.png");

        // Spawn sprite using app icon
        commands.spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
    }
}
