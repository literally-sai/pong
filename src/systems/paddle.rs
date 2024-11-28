use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::PaddleBundle;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.0;
        let right_paddle_x = window_width / 2.0 - padding;
        let left_paddle_x = -window_width / 2.0 + padding;
        
        let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        let mesh_handle = meshes.add(mesh);

        commands.spawn((Player1, PaddleBundle::new(right_paddle_x, 0.0),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: materials.add(
                        ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0))
                    ),
                    ..default()
                }));

        commands.spawn((Player1, PaddleBundle::new(left_paddle_x, 0.0),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: materials.add(
                        ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0))
                    ),
                    ..default()
                }));

    }
}
