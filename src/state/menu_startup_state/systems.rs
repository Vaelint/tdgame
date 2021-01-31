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
    sty_ui_button: Res<'_, StateUiResources>,
    res: Res<'_, StateMenuStartupResources>,
) {
    commands
        .spawn(ButtonBundle {
            style: sty_ui_button.style_std.clone(),
            material: mat_button.normal.clone(),
            ..Default::default()
        })
        .with(MenuStartupButtons::NewGame)
        // TODO refactor to combine these two function calls
        .with_children(create_child_txt_builder(
            "New Game".to_string(),
            res.fnt_bold_fira.clone(),
        ));

    // Store handle of sprite entity
    ents.ent_button_game_new = Some(commands.current_entity().unwrap());
}

/// Spawns an ent w/ a button component that has a Text component as it's child
pub fn spawn_but_game_load(
    commands: &mut Commands,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
    mat_button: Res<'_, ButtonMaterials>,
    sty_ui_button: Res<'_, StateUiResources>,
    res: Res<'_, StateMenuStartupResources>,
) {
    commands
        .spawn(ButtonBundle {
            style: sty_ui_button.style_std.clone(),
            material: mat_button.normal.clone(),
            ..Default::default()
        })
        .with(MenuStartupButtons::LoadGame)
        .with_children(create_child_txt_builder(
            "Load Game".to_string(),
            res.fnt_bold_fira.clone(),
        ));

    // Store handle of sprite entity
    ents.ent_button_game_new = Some(commands.current_entity().unwrap());
}

/// Spawns an ent w/ a button component that has a Text component as it's child
pub fn spawn_but_game_continue(
    commands: &mut Commands,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
    sty_ui_button: Res<'_, StateUiResources>,
    mat_button: Res<'_, ButtonMaterials>,

    res: Res<'_, StateMenuStartupResources>,
) {
    commands
        .spawn(ButtonBundle {
            style: sty_ui_button.style_std.clone(),
            material: mat_button.normal.clone(),
            ..Default::default()
        })
        .with(MenuStartupButtons::Continue)
        .with_children(create_child_txt_builder(
            "Continue".to_string(),
            res.fnt_bold_fira.clone(),
        ));

    // Store handle of sprite entity
    ents.ent_button_game_new = Some(commands.current_entity().unwrap());
}

/// Spawns an ent w/ a button component that has a Text component as it's child
pub fn spawn_but_game_exit(
    commands: &mut Commands,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
    mat_button: Res<'_, ButtonMaterials>,
    sty_ui_button: Res<'_, StateUiResources>,
    res: Res<'_, StateMenuStartupResources>,
) {
    commands
        .spawn(ButtonBundle {
            style: sty_ui_button.style_std.clone(),
            material: mat_button.normal.clone(),
            ..Default::default()
        })
        .with(MenuStartupButtons::Exit)
        .with_children(create_child_txt_builder(
            "Exit".to_string(),
            res.fnt_bold_fira.clone(),
        ));

    // Store handle of sprite entity
    ents.ent_button_game_new = Some(commands.current_entity().unwrap());
}

/// Spawns an ent w/ a button component that has a Text component as it's child
pub fn spawn_but_options(
    commands: &mut Commands,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
    mat_button: Res<'_, ButtonMaterials>,
    sty_ui_button: Res<'_, StateUiResources>,
    res: Res<'_, StateMenuStartupResources>,
) {
    commands
        .spawn(ButtonBundle {
            style: sty_ui_button.style_std.clone(),
            material: mat_button.normal.clone(),
            ..Default::default()
        })
        .with(MenuStartupButtons::Options)
        .with_children(create_child_txt_builder(
            "Options".to_string(),
            res.fnt_bold_fira.clone(),
        ));

    // Store handle of sprite entity
    ents.ent_button_game_new = Some(commands.current_entity().unwrap());
}

/// Updates button state and dispatches events
pub fn button_system(
    button_materials: Res<'_, ButtonMaterials>,
    mut interaction_query: Query<
        '_,
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (
            Mutated<Interaction>,
            (With<Button>, With<MenuStartupButtons>),
        ),
    >,
    mut _text_query: Query<'_, &Text>,
) {
    // TODO make align with project conventions
    // TODO Add button match to branch on button type
    for (interaction, mut material, _children) in interaction_query.iter_mut() {
        //let mut text = text_query.get_mut(children[0]).unwrap();

        // Change button material based on hover state
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
