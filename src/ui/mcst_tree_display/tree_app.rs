use bevy::ecs::system::ResMut;
use bevy_egui::EguiContexts;
use eframe::egui;
use egui::Vec2;
use std::sync::{Arc, Mutex};
use crate::system::mcst_tree::mcst_tree::MCTSTree;
use crate::system::mcst_tree::mcst_node::Node;

use super::mcst_tree_display::DisplayTreeWindowState;

#[derive(Default)]
pub struct TreeCamera {
    pub zoom: f32,
    pub offset: Vec2,
    pub is_dragging: bool,
    pub previous_mouse_position: Option<Vec2>,
}

pub fn run_tree_display(tree: MCTSTree) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        decorated: true,
        resizable: true,
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Tree Display",
        options,
        Box::new(|_cc| Box::new(TreeApp {
            tree,
            camera: TreeCamera::default(),
        })),
    )
}

struct TreeApp {
    tree: MCTSTree,
    camera: TreeCamera,
}

impl eframe::App for TreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(root) = &self.tree.root {
                // Allocate the entire window's space for rendering
                let available_rect = ui.allocate_rect(
                    egui::Rect::from_min_size(ui.min_rect().min, ui.available_size()),
                    egui::Sense::hover(),
                );
                let painter = ui.painter();

                // Check if the mouse is inside the tree rendering area
                let is_mouse_in_window = ui.rect_contains_pointer(available_rect.rect);

                // Handle zooming with the mouse wheel
                if is_mouse_in_window {
                    let scroll_delta = ui.input(|i| i.scroll_delta.y);
                    let zoom_factor = 1.1;
                    if scroll_delta > 0.0 {
                        self.camera.zoom /= zoom_factor;
                    } else if scroll_delta < 0.0 {
                        self.camera.zoom *= zoom_factor;
                    }
                    self.camera.zoom = self.camera.zoom.clamp(0.1, 5.0); // Clamp zoom level
                }

                // Handle panning with right-click drag
                if is_mouse_in_window {
                    if ui.input(|i| i.pointer.secondary_down()) {
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let mouse_pos_vec2 = mouse_pos.to_vec2(); // Convert Pos2 to Vec2
                            if let Some(prev_pos) = self.camera.previous_mouse_position {
                                let delta = mouse_pos_vec2 - prev_pos;
                                self.camera.offset += delta / self.camera.zoom;
                            }
                            self.camera.previous_mouse_position = Some(mouse_pos_vec2);
                        }
                    } else {
                        self.camera.previous_mouse_position = None;
                    }
                }

                // Consume mouse scroll events if the mouse is in the window
                if is_mouse_in_window {
                    ui.input_mut(|i| {
                        i.scroll_delta = Vec2::ZERO; 
                    });
                }

                // Draw a background for the tree rendering area
                painter.rect_filled(
                    available_rect.rect,
                    0.0, // No corner rounding
                    egui::Color32::from_gray(30), // Dark gray background
                );

                // Render the tree relative to the allocated area
                let center = available_rect.rect.center();
                render_node(
                    painter,
                    root,
                    center.x + self.camera.offset.x * self.camera.zoom,
                    center.y + self.camera.offset.y * self.camera.zoom,
                    200.0 * self.camera.zoom,
                );
            }
        });
    }
}

pub fn display_tree_window_system(
    mut egui_contexts: EguiContexts,
    mut window_state: ResMut<DisplayTreeWindowState>,
) {
    if window_state.is_open {
        let mut is_open = window_state.is_open; // Clone the `is_open` field
        let tree = window_state.tree.clone();   // Clone the `tree` field

        egui::Window::new("Tree Display")
            .resizable(true)
            .collapsible(false) // Prevent collapsing the window
            .open(&mut is_open) // Allow closing the window
            .show(egui_contexts.ctx_mut(), |ui| {
                if let Some(tree) = &tree {
                    if let Some(root) = &tree.root {
                        // Allocate the entire window's space for rendering
                        let available_rect = ui.allocate_rect(
                            egui::Rect::from_min_size(ui.min_rect().min, ui.available_size()),
                            egui::Sense::click_and_drag(),
                        );
                        let painter = ui.painter();

                        // Draw a background for the tree rendering area
                        painter.rect_filled(
                            available_rect.rect,
                            0.0, // No corner rounding
                            egui::Color32::from_gray(30), // Dark gray background
                        );

                        // Handle zooming with the mouse wheel
                        if ui.rect_contains_pointer(available_rect.rect) {
                            let scroll_delta = ui.input(|i| i.scroll_delta.y);
                            let zoom_factor = 1.1;
                            if scroll_delta > 0.0 {
                                window_state.camera.zoom /= zoom_factor;
                            } else if scroll_delta < 0.0 {
                                window_state.camera.zoom *= zoom_factor;
                            }
                            window_state.camera.zoom = window_state.camera.zoom.clamp(0.1, 5.0); // Clamp zoom level
                        }

                        // Handle panning with right-click drag
                        if ui.rect_contains_pointer(available_rect.rect) {
                            if ui.input(|i| i.pointer.secondary_down()) {
                                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                    let mouse_pos_vec2 = mouse_pos.to_vec2(); // Convert Pos2 to Vec2
                                    if let Some(prev_pos) = window_state.camera.previous_mouse_position {
                                        let delta = mouse_pos_vec2 - prev_pos;
                                        let camera = &mut window_state.camera;
                                        camera.offset += delta / camera.zoom;
                                    }
                                    window_state.camera.previous_mouse_position = Some(mouse_pos_vec2);
                                }
                            } else {
                                window_state.camera.previous_mouse_position = None;
                            }
                        }

                        // Render the tree relative to the allocated area
                        let center = available_rect.rect.center();
                        render_node(
                            painter,
                            root,
                            center.x + window_state.camera.offset.x * window_state.camera.zoom,
                            center.y + window_state.camera.offset.y * window_state.camera.zoom,
                            200.0 * window_state.camera.zoom,
                        );
                    } else {
                        ui.label("No root node found in the tree.");
                    }
                } else {
                    ui.label("No tree to display.");
                }
            });

        // Update the `is_open` field after the window is shown
        window_state.is_open = is_open;
    }
}

fn render_node(
    painter: &egui::Painter,
    node: &Arc<Mutex<Node>>,
    x: f32,
    y: f32,
    x_spacing: f32,
) {
    let node_lock = node.lock().unwrap();

    // Draw the node as a circle
    painter.circle_filled(
        egui::pos2(x, y),
        20.0,
        egui::Color32::WHITE,
    );
    painter.text(
        egui::pos2(x, y),
        egui::Align2::CENTER_CENTER,
        format!("{:?}", node_lock.get_node_type().to_string()),
        egui::FontId::default(),
        egui::Color32::BLACK,
    );

    // Draw edges and child nodes
    let num_children = node_lock.children.len();
    for (i, child) in node_lock.children.iter().enumerate() {
        let child_x = x - x_spacing / 2.0 + i as f32 * (x_spacing / num_children as f32);
        let child_y = y + 100.0;

        // Draw edge
        painter.line_segment(
            [egui::pos2(x, y + 20.0), egui::pos2(child_x, child_y - 20.0)],
            egui::Stroke::new(2.0, egui::Color32::BLACK),
        );

        // Recursively render the child node
        render_node(painter, child, child_x, child_y, x_spacing / 2.0);
    }
}