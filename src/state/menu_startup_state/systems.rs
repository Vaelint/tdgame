use crate::state::ButtonMaterials;

use super::resources::*;
/// Project startup menu ecs systems module
use bevy::{app::AppExit, prelude::*};

/// Spawns an ent w/ a sprite component in the center of the screen
pub fn spawn_sprite_main(
    commands: &mut Commands,
    res: Res<'_, StateMenuStartupResources>,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
) {
    // TODO Look into borrowing just needed data

    // Create transform matrix
    let trans_mat = Mat4::from_scale_rotation_translation(
        (3.0, 3.0, 1.0).into(),
        Quat::identity(),
        Vec3::zero(),
    );

    // Spawn sprite using texture from LoadStateRes
    commands.spawn(SpriteBundle {
        material: res.mat_clr_icon.clone(),
        transform: Transform::from_matrix(trans_mat),
        ..Default::default()
    });

    // Register sprite entity
    ents.ent_sprite_icon = Some(commands.current_entity().unwrap());
}

/// Spawns buttons for main menu
pub fn spawn_sidebar(
    commands: &mut Commands,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
    mat_button: Res<'_, ButtonMaterials>,
    sty_ui_button: Res<'_, StateUiResources>,
    res: Res<'_, StateMenuStartupResources>,
) {
    // Spawn root node
    commands
        .spawn(NodeBundle {
            style: sty_ui_button.style_node_root.clone(),
            // FIXME TODO
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    // Setup Margin
                    size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::FlexStart,
                    flex_direction: FlexDirection::Column,
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
        })
        // Spawn continue game button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: sty_ui_button.style_std.clone(),
                    material: mat_button.normal.clone(),
                    ..Default::default()
                })
                .with(MenuStartupButtons::Continue)
                .with_children(create_child_txt_builder(
                    "Continue Game".to_string(),
                    res.fnt_bold_fira.clone(),
                ));
        })
        // Spawn new game button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: sty_ui_button.style_std.clone(),
                    material: mat_button.normal.clone(),
                    ..Default::default()
                })
                .with(MenuStartupButtons::NewGame)
                .with_children(create_child_txt_builder(
                    "New Game".to_string(),
                    res.fnt_bold_fira.clone(),
                ));
        })
        // Spawn Load Game button
        .with_children(|parent| {
            parent
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
        })
        // Spawn Options button
        .with_children(|parent| {
            parent
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
        })
        // Spawn Exit Game button
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: sty_ui_button.style_std.clone(),
                    material: mat_button.normal.clone(),
                    ..Default::default()
                })
                .with(MenuStartupButtons::Exit)
                .with_children(create_child_txt_builder(
                    "Exit Game".to_string(),
                    res.fnt_bold_fira.clone(),
                ));
        });
}

/// Updates button state and dispatches events
pub fn button_system(
    button_materials: Res<'_, ButtonMaterials>,
    mut interaction_query: Query<
        '_,
        (
            &Interaction,
            &mut Handle<ColorMaterial>,
            &Children,
            &MenuStartupButtons,
        ),
        (
            Mutated<Interaction>,
            (With<Button>, With<MenuStartupButtons>),
        ),
    >,
    mut _text_query: Query<'_, &Text>,
    mut app_exit_events: ResMut<'_, Events<AppExit>>,
) {
    // TODO make align with project conventions
    // TODO Add button match to branch on button type
    for (interaction, mut material, _children, button) in interaction_query.iter_mut() {
        //let mut text = text_query.get_mut(children[0]).unwrap();

        // Change button material based on hover state
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match button {
                    MenuStartupButtons::Continue => {}
                    MenuStartupButtons::NewGame => {}
                    MenuStartupButtons::LoadGame => {}
                    MenuStartupButtons::Options => {}
                    // TODO confirm exit before exiting
                    MenuStartupButtons::Exit => app_exit_events.send(AppExit {}),
                }
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
