use bevy::prelude::*;
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::input::ElementState;
use bevy::render::camera::Camera;

#[allow(unused_variables)]
#[derive(Default)]
pub struct CameraDragging {
    pub is_dragging: bool,
    pub previous_mouse_position: Option<Vec2>,
}


pub fn camera_drag_system(
    mut ev_mouse: EventReader<MouseButtonInput>,
    mut camera_dragging: ResMut<CameraDragging>,
    windows: Res<Windows>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    for event in ev_mouse.iter() {
        if event.button == MouseButton::Right {
            if event.state == ElementState::Pressed {
                camera_dragging.is_dragging = true;
                camera_dragging.previous_mouse_position = Some(window.cursor_position().unwrap());
            } else if event.state == ElementState::Released {
                camera_dragging.is_dragging = false;
            }
        }
    }
    if camera_dragging.is_dragging {
        if let Some(current_mouse_position) = window.cursor_position() {
            if let Some(prev_mouse_position) = camera_dragging.previous_mouse_position {
                let delta = current_mouse_position - prev_mouse_position;
                for (_, mut transform) in query.iter_mut() {
                    transform.translation.x -= delta.x;
                    transform.translation.y -= delta.y;
                }
            }
            camera_dragging.previous_mouse_position = Some(current_mouse_position);
        }
    }
}