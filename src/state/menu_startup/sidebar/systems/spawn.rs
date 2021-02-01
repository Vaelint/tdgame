//! Project Main Menu sidebar module

use crate::state::menu_startup::{
    assets::StartupMenuRes,
    sidebar::SidebarEnts,
    styles::StateUiResources,
    txtbuilder::{create_child_txt_builder, MenuStartupButtons},
};
use crate::state::ButtonMaterials;
use bevy::prelude::*;

/// Spawns buttons for main menu
pub fn spawn_sidebar(
    commands: &mut Commands,
    mut ents: ResMut<'_, SidebarEnts>,
    mat_button: Res<'_, ButtonMaterials>,
    sty_ui_button: Res<'_, StateUiResources>,
    res: Res<'_, StartupMenuRes>,
) {
    // Spawn sidebar ents
    commands
        // Root node
        .spawn(NodeBundle {
            style: sty_ui_button.style_node_root.clone(),
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            ..Default::default()
        })
        // Game Text
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    // Setup Margin
                    size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
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
        });

    // Spawn buttons as children
    spawn_buttons(
        commands,
        mat_button.normal.clone(),
        sty_ui_button.style_std.clone(),
        res.fnt_bold_fira.clone(),
    );

    // Store handle of sidebar entity
    ents.ent_sidebar = Some(commands.current_entity().unwrap());
}

/// Data needed for spawning sidebar buttons recursively
const BUTTON_DATA: [(&str, MenuStartupButtons); 5] = [
    ("Continue Game", MenuStartupButtons::Continue),
    ("New Game", MenuStartupButtons::NewGame),
    ("Load Game", MenuStartupButtons::LoadGame),
    ("Options Menu", MenuStartupButtons::Options),
    ("Exit Game", MenuStartupButtons::Exit),
];

/// Spawns the start menu buttons as a child of parent
fn spawn_buttons(
    commands: &mut Commands,
    mat: Handle<ColorMaterial>,
    sty: Style,
    fnt: Handle<Font>,
) {
    // Spawn buttons
    // Uses data from static memory
    for button in BUTTON_DATA.iter() {
        spawn_button_as_child(
            commands,
            mat.clone(),
            sty.clone(),
            fnt.clone(),
            String::from(button.0),
            button.1.clone(),
        );
    }
}

/// Spawns a ButtonBundle as a child of current ent?
fn spawn_button_as_child<'com>(
    commands: &'com mut Commands,
    mat: Handle<ColorMaterial>,
    sty: Style,
    fnt: Handle<Font>,
    text: String,
    butt: MenuStartupButtons,
) -> &'com mut Commands {
    commands.with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                style: sty,
                material: mat,
                ..Default::default()
            })
            .with(butt)
            .with_children(create_child_txt_builder(text, fnt));
    })
}
