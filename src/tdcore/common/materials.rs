/// Common material code
use bevy::prelude::*;

/// Loads a material into resources and returns its handle
pub fn load_mat<'func>(
    asset_server: &'func Res<AssetServer>,
    mut mats: &'func mut ResMut<Assets<ColorMaterial>>,
    asset_path: &str,
) -> Handle<ColorMaterial> {
    mats.add(asset_server.load(asset_path).into())
}
