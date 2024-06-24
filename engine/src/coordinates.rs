use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
pub struct CoordinateSystem;

impl Plugin for CoordinateSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_vertical_line, setup_horizontal_line));
    }
}

fn setup_vertical_line(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle::from(meshes.add(Rectangle::new(3.0, window.height() * 0.9))),
        material: materials.add(Color::rgba(0.75, 0.0, 0.0, 0.5)),
        transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
        ..default()
    });
    for i in 1..=10 {
        let y = window.height() * 0.05 + window.height() * 0.09 * i as f32;
        let mut height = 3.0;
        let mut width = window.width() * 0.02;
        if i == 5 || i == 10 {
            height = 5.0;
            width = window.width() * 0.03;
        }
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle::from(meshes.add(Rectangle::new(width, height))),
            material: materials.add(Color::rgba(0.75, 0.0, 0.0, 0.5)),
            transform: Transform::from_xyz(window.width() * 0.5, y, 0.0),
            ..default()
        });
    }
}

fn setup_horizontal_line(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle::from(meshes.add(Rectangle::new(window.width() * 0.9, 3.0))),
        material: materials.add(Color::rgba(0.75, 0.0, 0.0, 0.5)),
        transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.05, 0.0),
        ..default()
    });
}
