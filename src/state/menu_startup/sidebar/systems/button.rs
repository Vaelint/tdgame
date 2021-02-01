use bevy::prelude::*;

use crate::state::menu_startup::MenuStartupButtons;
use crate::state::ButtonMaterials;
use crate::state::{menu_startup::exit_diag::ExitConfirmDiagEvent, DiagEvents};

/// Updates button state and dispatches events
pub fn startup_butt_sys(
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
    mut exitconf_events: ResMut<'_, Events<ExitConfirmDiagEvent>>,
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
                    MenuStartupButtons::Exit => {
                        exitconf_events.send(ExitConfirmDiagEvent(DiagEvents::Open))
                    }
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