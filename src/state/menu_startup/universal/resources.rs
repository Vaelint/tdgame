use bevy::prelude::*;

/// Resources for project startup state
#[derive(Debug, Clone)]
pub struct StartupMenuRes {
    pub fnt_standard: Handle<Font>,
    pub mat_background: Handle<ColorMaterial>,
}

impl FromResources for StartupMenuRes {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let asset_srv = resources.get_mut::<AssetServer>().unwrap();
        let mut res_mat_clr = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            fnt_standard: asset_srv.load("fnt/FiraSans-Bold.ttf"),
            mat_background: res_mat_clr.add(asset_srv.load("tex/background.png").into()),
        }
    }
}
