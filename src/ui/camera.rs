use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;

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
    mut egui_ctx: EguiContexts,
    mut ev_mouse: EventReader<MouseButtonInput>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera_dragging: ResMut<CameraDragging>,
    mut query: Query<(&Camera, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let ctx = egui_ctx.ctx_mut();

    // 1) If any egui widget wants the pointer or scroll, do nothing.
    if ctx.wants_pointer_input() {
        return;
    }

    // 2) Handle right‐click drag start/stop
    let window = window_query.single();
    for event in ev_mouse.iter() {
        if event.button == MouseButton::Right {
            if event.state == ButtonState::Pressed {
                camera_dragging.is_dragging = true;
                camera_dragging.previous_mouse_position = window.cursor_position();
            } else {
                camera_dragging.is_dragging = false;
            }
        }
    }

    // 3) Perform panning
    if camera_dragging.is_dragging {
        if let (Some(curr), Some(prev)) = (window.cursor_position(), camera_dragging.previous_mouse_position) {
            let delta = curr - prev;
            for (_, mut transform) in &mut query {
                let zoom = transform.scale.x;
                transform.translation.x -= delta.x * zoom;
                transform.translation.y += delta.y * zoom;
            }
            camera_dragging.previous_mouse_position = Some(curr);
        }
    }

    // 4) Handle zoom with scroll
    for event in ev_scroll.iter() {
        for (_, mut transform) in &mut query {
            let factor = 1.1;
            let (min, max) = (0.1, 5.0);
            if event.y > 0.0 { transform.scale /= factor; }
            else if event.y < 0.0 { transform.scale *= factor; }
            transform.scale = transform.scale.clamp(Vec3::splat(min), Vec3::splat(max));
            transform.translation.z = transform.scale.x.max(0.1);
        }
    }
}