
use bevy::prelude::*;

/// Resources for project startup state
#[derive(Debug, Clone)]
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
            mat_clr_icon: res_mat_clr.add(asset_srv.load("tex/background.png").into()),
        }
    }
}