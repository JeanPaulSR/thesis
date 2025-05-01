use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::window::PrimaryWindow;

/// Sets up a 2D camera and centers it on the specified grid dimensions.
pub fn setup_camera(commands: &mut Commands, half_grid_width: f32, half_grid_height: f32) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(half_grid_width, half_grid_height, 1000.0),
        ..Default::default()
    });
}

#[allow(unused_variables)]
#[derive(Default, Resource)]
pub struct CameraDragging {
    pub is_dragging: bool,
    pub previous_mouse_position: Option<Vec2>,
}

pub fn camera_drag_system(
    mut ev_mouse: EventReader<MouseButtonInput>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera_dragging: ResMut<CameraDragging>,
    mut query: Query<(&Camera, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Access the primary window using the query
    let window = window_query.get_single().expect("Primary window not found");

    // Handle mouse button inputs for dragging
    for event in ev_mouse.iter() {
        if event.button == MouseButton::Right {
            if event.state == ButtonState::Pressed {
                camera_dragging.is_dragging = true;
                camera_dragging.previous_mouse_position = window.cursor_position();
            } else if event.state == ButtonState::Released {
                camera_dragging.is_dragging = false;
            }
        }
    }

    // Handle camera dragging
    if camera_dragging.is_dragging {
        if let Some(current_mouse_position) = window.cursor_position() {
            if let Some(prev_mouse_position) = camera_dragging.previous_mouse_position {
                let delta = current_mouse_position - prev_mouse_position;
                for (_, mut transform) in query.iter_mut() {
                    // Adjust movement speed based on zoom level
                    let zoom_adjustment = transform.scale.x;
                    transform.translation.x -= delta.x * zoom_adjustment;
                    transform.translation.y += delta.y * zoom_adjustment;
                }
            }
            camera_dragging.previous_mouse_position = Some(current_mouse_position);
        }
    }

    // Handle zooming with the mouse wheel
    for event in ev_scroll.iter() {
        for (_, mut transform) in query.iter_mut() {
            let zoom_factor = 1.1;
            let min_scale = 0.1;
            let max_scale = 5.0;

            if event.y > 0.0 {
                transform.scale /= zoom_factor;
            } else if event.y < 0.0 {
                transform.scale *= zoom_factor;
            }

            // Clamp the scale to prevent zooming too far in or out
            transform.scale = transform.scale.clamp(Vec3::splat(min_scale), Vec3::splat(max_scale));

            // Ensure the camera doesn't move out of the view bounds
            transform.translation.z = transform.scale.x.max(0.1);
        }
    }
}