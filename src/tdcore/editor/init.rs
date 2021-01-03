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

    use crate::tdcore::common::load_mat;

    pub fn setup(
        commands: &mut Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // Load materials
        let icon_mat_handle = load_mat(&asset_server, &mut materials, "tex/icon.png");
        let spinner_mat_handle = load_mat(&asset_server, &mut materials, "tex/spinner.png");
        let fira_bold_fnt_handle = asset_server.load("fnt/FiraSans-Bold.ttf");

        // Spawn entities
        spawn_main_sprite(commands, icon_mat_handle);
        spawn_progress_spinner(commands, spinner_mat_handle);
        spawn_loading_text(commands, fira_bold_fnt_handle);
    }

    /// Spawns an ent w/ a sprite component in the center of the screen
    fn spawn_main_sprite(commands: &mut Commands, mat: Handle<ColorMaterial>) {
        // Create transform matrix
        let trans_mat = Mat4::from_scale_rotation_translation(
            Vec3::one(),
            Quat::identity(),
            Vec3::zero(),
        );

        // Spawn sprite using provided texture
        commands.spawn(SpriteBundle {
            material: mat,
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        });
    }

    /// Spawns progress spinner ent
    fn spawn_progress_spinner(commands: &mut Commands, mat: Handle<ColorMaterial>) {
        // Create transform matrix
        let trans_mat = Mat4::from_scale_rotation_translation(
            (0.1, 0.1, 1.0).into(),
            Quat::identity(),
            (0.0, -250.0, 0.0).into(),
        );

        // Spawn sprite using provided texture
        commands.spawn(SpriteBundle {
            material: mat,
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        });
    }

    /// Spawns loading text
    fn spawn_loading_text(commands: &mut Commands, fnt_handle: Handle<Font>) {
        // Create transform matrix
        let trans_mat= Mat4::from_scale_rotation_translation(
            (1.0, 1.0, 1.0).into(),
            Quat::identity(),
            (0.0, -250.0, 0.0).into(),
        );

        commands.spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Now Loading ...".to_string(),
                font: fnt_handle,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            },
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        });
    }
}
