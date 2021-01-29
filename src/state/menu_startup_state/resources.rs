use bevy::prelude::*;

/// Holds the entity ids of entities spawned by this state
///
/// NOTE: only add entities that need to be cleaned upon exiting the state
#[derive(Debug, Default)]
pub struct StateMenuStartupEnts {
    pub ent_txt_menu_main_title: Option<Entity>,
    pub ent_sprite_icon: Option<Entity>,
    pub ent_cam_main: Option<Vec<Entity>>,
    pub ent_cam_ui: Option<Vec<Entity>>,
    pub ent_button_game_new: Option<Entity>,
}

/// Resources for project startup state
#[derive(Debug)]
pub struct StateMenuStartupResources {
    pub fnt_bold_fira: Handle<Font>,
    pub mat_clr_icon: Handle<ColorMaterial>,
}

impl FromResources for StateMenuStartupResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let asset_srv = resources.get_mut::<AssetServer>().unwrap();
        let mut res_mat_clr = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            fnt_bold_fira: asset_srv.load("fnt/FiraSans-Bold.ttf"),
            mat_clr_icon: res_mat_clr.add(asset_srv.load("tex/icon.png").into()),
        }
    }
}
