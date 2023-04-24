use bevy::prelude::*;

pub fn setup_camera(
    commands: &mut Commands,
    half_grid_width: f32,
    half_grid_height: f32,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(half_grid_width, half_grid_height, 1000.0));
}