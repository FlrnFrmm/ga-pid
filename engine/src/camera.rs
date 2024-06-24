use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let center = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(center),
        ..Default::default()
    });
}
