//! Loading screen background systems module

use crate::state::menu_startup::assets::StartupMenuRes;
use crate::state::menu_startup::sidebar::ents::SidebarEnts;
use bevy::prelude::*;

/// Spawns an ent w/ a sprite component in the center of the screen
pub fn spawn_sprite_main(
    commands: &mut Commands,
    res: Res<'_, StartupMenuRes>,
    mut ents: ResMut<'_, SidebarEnts>,
) {
    // TODO Look into borrowing just needed data

    // Create transform matrix
    let trans_mat = Mat4::from_scale_rotation_translation(
        (2.5, 2.5, 1.0).into(),
        Quat::identity(),
        Vec3::zero(),
    );

    // Spawn sprite using texture from LoadStateRes
    commands.spawn(SpriteBundle {
        material: res.mat_clr_icon.clone(),
        transform: Transform::from_matrix(trans_mat),
        ..Default::default()
    });

    // Register sprite entity
    ents.ent_background = Some(commands.current_entity().unwrap());
}
