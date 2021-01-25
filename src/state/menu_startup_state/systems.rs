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
        text: Text {
            value: "Project Name".to_string(),
            font: res.fnt_bold_fira.clone(),
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
