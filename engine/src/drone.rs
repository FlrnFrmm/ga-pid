use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

use super::physics::{Acceleration, Velocity};

#[derive(Debug, Component)]
pub struct Drone;

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let center = Vec3::new(window.width() / 2.0, window.height() / 2.0, 10.0);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle::from(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
            transform: Transform::from_translation(center),
            ..default()
        },
        Velocity::new(0.0, 0.0, 0.0),
        Acceleration::new(0.0, 9.81, 0.0),
        Drone,
    ));
}
