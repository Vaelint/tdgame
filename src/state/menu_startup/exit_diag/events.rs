//! Exit confirmation event module

use bevy::prelude::*;

use crate::state::{
    menu_startup::universal::{StartupMenuRes, StateUiResources},
    ButtonMaterials, DiagEvents,
};

/// Exit conformation dialog event
pub struct ExitConfirmDiagEvent(pub DiagEvents);
pub struct ExitConfirmDiagEnts {
    ent_root_node: Option<Entity>,
}

impl FromResources for ExitConfirmDiagEnts {
    fn from_resources(_resources: &Resources) -> Self {
        Self {
            ent_root_node: None,
        }
    }
}

/// Exit conformation spawning system
pub fn event_exitconf_sys(
    mut events: EventReader<'_, ExitConfirmDiagEvent>,
    commands: &mut Commands,
    res_ui: Res<'_, StateUiResources>,
    res_butt: Res<'_, ButtonMaterials>,
    res_state: Res<'_, StartupMenuRes>,
    mut res_ents: ResMut<'_, ExitConfirmDiagEnts>,
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
                                "Are you sure you want to exit?",
                                TextStyle {
                                    font: res_state.fnt_standard.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    })
                    .with_children(|parent| {
                        parent.spawn(ButtonBundle {
                            style: res_ui.style_std.clone(),
                            material: res_butt.normal.clone(),
                            ..Default::default()
                        });
                    });

                // Store entity handle for despawning
                res_ents.ent_root_node = Some(commands.current_entity().unwrap());
            }
            // Close Dialog
            DiagEvents::Close => {
                // Despawn spawned ents
                // TODO recover if Close event without ent handle to despawn
                commands.despawn_recursive(
                    res_ents
                        .ent_root_node
                        .expect("Tried to despawn popup root node but it doesn't exist"),
                );
            }
        }
    }
}
