use crate::state::ButtonMaterials;

use super::resources::*;
/// Project startup menu ecs systems module
use bevy::prelude::*;

/// Spawns loading text entity
pub fn spawn_txt_menu_main_title(commands: &mut Commands, res: Res<'_, StateMenuStartupResources>) {
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
        text: Text::with_section(
            "Project Name",
            TextStyle {
                font: res.fnt_bold_fira.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    });
}

/// Spawns an ent w/ a sprite component in the center of the screen
pub fn spawn_sprite_main(
    commands: &mut Commands,
    res: Res<'_, StateMenuStartupResources>,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
) {
    // TODO Look into borrowing just needed data

    // Create transform matrix
    let trans_mat =
        Mat4::from_scale_rotation_translation(Vec3::one(), Quat::identity(), Vec3::zero());

    // Spawn sprite using texture from LoadStateRes
    commands.spawn(SpriteBundle {
        material: res.mat_clr_icon.clone(),
        transform: Transform::from_matrix(trans_mat),
        ..Default::default()
    });

    // Register sprite entity
    ents.ent_sprite_icon = Some(commands.current_entity().unwrap());
}

/// Spawns an ent w/ a button component that has a Text component as it's child
pub fn spawn_but_game_new(
    commands: &mut Commands,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
    mat_button: Res<'_, ButtonMaterials>,
    res: Res<'_, StateMenuStartupResources>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: mat_button.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::with_section(
                    "New Game",
                    TextStyle {
                        font: res.fnt_bold_fira.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    // Store handle of sprite entity
    ents.ent_button_game_new = Some(commands.current_entity().unwrap());
}
