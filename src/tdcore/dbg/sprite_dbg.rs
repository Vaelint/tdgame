use bevy::prelude::*;

use crate::tdcore::prelude::*;

/// Bevy plugin for debugging sprites
pub struct SpriteDbgPlug;

impl Plugin for SpriteDbgPlug {
    fn build(&self, app: &mut AppBuilder) {
        // Verbose debugging systems
        app.add_startup_system(log_sprite_materials.system());
    }
    fn name(&self) -> &str {
        "Sprite Debugging Plugin"
    }
}

/// Prints debug for all entities with a sprite component
fn log_sprite_materials(query: Query<(&SpriteBundle, &DebugSwitch)>) {
    for (sprite, dbgswitch) in query.iter() {
        if dbgswitch.0 == true {
            info!("{:?}", sprite.material);
        } else {
            info!("No sprites w/ dbg switch loaded");
        }
    }
}
