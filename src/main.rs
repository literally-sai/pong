use bevy::prelude::*;

mod systems;
mod constants;
mod components;
mod bundles;

use systems::*;
use bundles::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(create_window()))
        .add_systems(Startup, (spawn_dotted_lines, spawn_ball, spawn_paddles, spawn_camera))
        .add_systems(Update, (move_ball, update_entity_position.after(move_ball)))
        .run();
}
