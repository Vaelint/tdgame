//! Text convenience function module
// TODO Delete this

use bevy::prelude::*;

/// List of buttons in main menu
// TODO Bevy reflection
#[derive(Debug, Clone)]
pub enum MenuStartupButtons {
    Continue,
    NewGame,
    LoadGame,
    Options,
    Exit,
}

/// Returns a closure that spawns a TextBundle from a parent
// TODO Give more control over resulting text
pub fn create_child_txt_builder(
    text: String,
    fnt: Handle<Font>,
) -> impl FnOnce(&mut ChildBuilder<'_>) {
    |parent| {
        parent.spawn(TextBundle {
            text: Text::with_section(
                text,
                TextStyle {
                    font: fnt,
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }
}
