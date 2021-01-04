/// Loading screen module.
use bevy::prelude::*;
use bevy::ui::{PositionType, Val};

use crate::ecs::*;

/// Bevy plugin for handling editor initialization
pub struct LoadScreenPlug;

impl Plugin for LoadScreenPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .init_resource::<LoadScreenResources>();
    }
    fn name(&self) -> &str {
        "EditorInitPlugin"
    }
}

pub struct LoadScreenResources {
    icon_mat: Handle<ColorMaterial>,
    spinner_mat: Handle<ColorMaterial>,
    fira_bold_fnt: Handle<Font>,
}

impl FromResources for LoadScreenResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let mut mats = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_serv = resources.get_mut::<AssetServer>().unwrap();

        // Load assets
        Self {
            icon_mat: mats.add(asset_serv.load("tex/icon.png").into()),
            spinner_mat: mats.add(asset_serv.load("tex/spinner.png").into()),
            fira_bold_fnt: asset_serv.load("fnt/FiraSans-Bold.ttf"),
        }
    }
}

/// Creates the loading screen scene
pub fn setup(commands: &mut Commands, lscrn_res: Res<LoadScreenResources>) {
    // Spawn entities
    spawn_main_sprite(commands, lscrn_res.icon_mat.clone());
    spawn_progress_spinner(commands, lscrn_res.spinner_mat.clone());
    spawn_loading_text(
        commands,
        lscrn_res.fira_bold_fnt.clone(),
        format!("CatGame: version {}", env!("CARGO_PKG_VERSION")),
    );
}

/// Spawns an ent w/ a sprite component in the center of the screen
fn spawn_main_sprite(commands: &mut Commands, mat: Handle<ColorMaterial>) {
    // Create transform matrix
    let trans_mat =
        Mat4::from_scale_rotation_translation(Vec3::one(), Quat::identity(), Vec3::zero());

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

    // Spawn spinner
    commands
        .spawn(SpriteBundle {
            material: mat,
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        })
        .with(Rotating(-4.0));
}

/// Spawns loading text entity
fn spawn_loading_text(commands: &mut Commands, fnt_handle: Handle<Font>, text: String) {
    commands.spawn(TextBundle {
        style: Style {
            // Setup Margin
            size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(20.0),
                top: Val::Percent(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            value: text.to_string(),
            font: fnt_handle,
            style: TextStyle {
                font_size: 60.0,
                color: Color::BLACK,
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
        },
        ..Default::default()
    });
}