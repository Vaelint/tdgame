use bevy::prelude::*;

/// Holds the entity ids of entities spawned by this state
///
/// NOTE: only add entities that need to be cleaned upon exiting the state
#[derive(Debug, Default)]
pub struct StateMenuStartupEnts {
    pub ent_sidebar: Option<Entity>,
    pub ent_background: Option<Entity>,
    pub ent_cam_main: Option<Vec<Entity>>,
    pub ent_cam_ui: Option<Vec<Entity>>,
}
