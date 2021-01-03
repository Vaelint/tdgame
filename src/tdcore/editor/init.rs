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
        let material_handle = materials.add(texture_handle.into());

        spawn_main_sprite(commands, material_handle);

    }

    fn spawn_main_sprite(commands: &mut Commands,mat: Handle<ColorMaterial>) {
        // Spawn sprite using provided texture
        commands.spawn(SpriteBundle {
            material: mat,
            transform: Transform::from_scale(Vec3::new(10.0, 10.0, 1.0)),
            ..Default::default()
        });
    }
}
