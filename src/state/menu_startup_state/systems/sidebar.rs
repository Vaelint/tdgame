//! Project Main Menu sidebar module

use super::super::resources::*;
use crate::state::ButtonMaterials;
use bevy::prelude::*;

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

    // Store handle of sidebar entity
    ents.ent_sidebar = Some(commands.current_entity().unwrap());
}
