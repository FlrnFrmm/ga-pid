mod camera;
mod coordinates;
mod drone;
mod physics;

use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, coordinates::CoordinateSystem))
        .add_systems(Startup, (camera::setup, drone::setup))
        .add_systems(
            Update,
            (
                physics::apply_acceleration,
                physics::apply_velocity,
                physics::confine_movement,
            )
                .chain(),
        )
        .run();
}
