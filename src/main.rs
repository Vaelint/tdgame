use bevy::prelude::*;

mod ecs;

use ecs::plugins;

fn main() {
    App::build().add_plugins(plugins::DbgPlugs).run();
}
