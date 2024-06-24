use std::os::unix::raw::off_t;

use bevy::ecs::component::Component;
use bevy::ecs::system::{Query, Res};
use bevy::ecs::world;
use bevy::math::Vec3;
use bevy::prelude::With;
use bevy::time::Time;
use bevy::transform::components::Transform;
use bevy::utils::dbg;
use bevy::window::{PrimaryWindow, Window};

#[derive(Debug, Component)]
pub struct Acceleration(Vec3);

impl Acceleration {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

#[derive(Debug, Component)]
pub struct Velocity(Vec3);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

pub fn apply_acceleration(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

pub fn apply_velocity(
    mut query: Query<(&Velocity, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let factor = (0.9 * window.height()) / 5.0;
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * factor * time.delta_seconds();
    }
}

pub fn confine_movement(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    const RADIUS: f32 = 10.0;
    let window = window_query.get_single().unwrap();
    let offset = Vec3::new(window.width() * 0.05, window.height() * 0.05, 0.0);
    for (mut _velocity, mut transform) in query.iter_mut() {
        if transform.translation.x - RADIUS < offset.x {
            transform.translation.x = RADIUS + offset.x;
        }
        if transform.translation.x + RADIUS > window.width() - offset.x {
            transform.translation.x = window.width() - RADIUS;
        }
        if transform.translation.y - RADIUS < offset.y {
            transform.translation.y = RADIUS + offset.y;
        }
        if transform.translation.y + RADIUS > window.height() - offset.y {
            transform.translation.y = window.height() - RADIUS;
        }
    }
}
