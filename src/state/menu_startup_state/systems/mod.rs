use crate::state::ButtonMaterials;

use super::resources::*;
/// Project startup menu ecs systems module
use bevy::{app::AppExit, prelude::*};

mod sidebar;

pub use sidebar::*;

/// Spawns an ent w/ a sprite component in the center of the screen
pub fn spawn_sprite_main(
    commands: &mut Commands,
    res: Res<'_, StateMenuStartupResources>,
    mut ents: ResMut<'_, StateMenuStartupEnts>,
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

/// Updates button state and dispatches events
pub fn button_system(
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
    mut app_exit_events: ResMut<'_, Events<AppExit>>,
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
                    MenuStartupButtons::Exit => app_exit_events.send(AppExit {}),
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
