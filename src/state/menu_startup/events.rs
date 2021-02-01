//! Exit confirmation event module

use std::num::ParseIntError;

use bevy::prelude::*;

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
pub fn event_exitconf_sys(mut events: EventReader<'_, ExitConfirmDiagEvent>) {
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
                // TODO spawn root node/children
            }
            // Close Dialog
            DiagEvents::Close => {}
        }
    }
}
