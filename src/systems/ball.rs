use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::BallBundle;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0));

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    

    commands.spawn((
            BallBundle::new(1.0, 0.0),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            }
    ));
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) { 
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        positon.0 += velocity.0 * BALL_SPEED;
    }
}

pub fn update_entity_position(&mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in ball.iter_mut() {
        transform.translate = position.0.extend(0.0);
    }
}
