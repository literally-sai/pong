use bevy::prelude::*;
mod bundles;
mod components;
mod constants;
mod systems;
use bundles::*;
use components::*;
use systems::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.23, 0.23, 0.23)))
        .add_plugins(DefaultPlugins.set(create_window()))
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                spawn_dotted_line,
                spawn_ball,
                spawn_paddles,
                spawn_camera,
                spawn_scoreboard,
                spawn_boundary,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                move_player1_paddle,
                detect_scoring,
                move_player2_paddle,
                respawn_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(update_score),
                update_entity_positions.after(move_ball),
                move_paddles.after(move_player1_paddle),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}
