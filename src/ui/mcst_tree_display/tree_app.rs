use bevy::ecs::system::Res;
use bevy_egui::EguiContexts;
use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::system::mcst_tree::mcst_tree::MCTSTree;
use crate::system::mcst_tree::mcst_node::Node;

use super::mcst_tree_display::DisplayTreeWindowState;

pub fn run_tree_display(tree: MCTSTree) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tree Display",
        options,
        Box::new(|_cc| Box::new(TreeApp { tree })),
    )
}

struct TreeApp {
    tree: MCTSTree,
}

impl eframe::App for TreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(root) = &self.tree.get_root() {
                render_node(ui, root, 400.0, 50.0, 200.0);
            }
        });
    }
}

fn render_node(
    ui: &mut egui::Ui,
    node: &Arc<Mutex<Node>>,
    x: f32,
    y: f32,
    x_spacing: f32,
) {
    let node_lock = node.lock().unwrap();

    // Draw the node as a circle
    ui.painter().circle_filled(
        egui::pos2(x, y),
        20.0,
        egui::Color32::WHITE,
    );
    ui.painter().text(
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
        ui.painter().line_segment(
            [egui::pos2(x, y + 20.0), egui::pos2(child_x, child_y - 20.0)],
            egui::Stroke::new(2.0, egui::Color32::BLACK),
        );

        // Recursively render the child node
        render_node(ui, child, child_x, child_y, x_spacing / 2.0);
    }
}

pub fn display_tree_window_system(
    mut egui_contexts: EguiContexts,
    window_state: Res<DisplayTreeWindowState>,
) {
    if window_state.is_open {
        egui::Window::new("Tree Display")
            .resizable(true)
            .show(egui_contexts.ctx_mut(), |ui| {
                if let Some(tree) = &window_state.tree {
                    if let Some(root) = &tree.root {
                        render_node(ui, root, 400.0, 50.0, 200.0);
                    }
                } else {
                    ui.label("No tree to display.");
                }
            });
    }
}