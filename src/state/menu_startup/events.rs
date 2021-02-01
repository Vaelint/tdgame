//! Exit confirmation event module

use std::num::ParseIntError;

use bevy::prelude::*;

use super::resources::{StateMenuStartupResources, StateUiResources};

pub enum DiagEvents {
    Confirm,
    Deny,
    Open,
    Close,
}

/// Exit conformation dialog event
pub struct ExitConfirmDiagEvent(pub DiagEvents);
pub struct ExitConfirmDiagEnts {
    ent_root_node: Entity,
}

/// Exit conformation spawning system
pub fn event_exitconf_sys(
    mut events: EventReader<'_, ExitConfirmDiagEvent>,
    commands: &mut Commands,
    res_ui: Res<'_, StateUiResources>,
    res_state: Res<'_, StateMenuStartupResources>,
) {
    for event in events.iter() {
        println!("exit confirmation event triggered");
        // TODO Spawn exit conformation dialog

        match event.0 {
            // Dialog Accept
            DiagEvents::Confirm => {}
            // Dialog Reject
            DiagEvents::Deny => {}
            // Open Dialog
            DiagEvents::Open => {
                // Spawn root node/children
                commands
                    .spawn(NodeBundle {
                        style: res_ui.style_popup.clone(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            style: Style {
                                // Set size of object
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                // Object Margins
                                margin: Rect::all(Val::Percent(0.0)),

                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Project Name",
                                TextStyle {
                                    font: res_state.fnt_bold_fira.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            }
            // Close Dialog
            DiagEvents::Close => {}
        }
    }
}
