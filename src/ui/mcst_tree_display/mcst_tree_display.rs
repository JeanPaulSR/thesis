use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_egui::{egui, EguiContexts};
use crate::npcs::agent::Agent;
use crate::system::mcst_tree::mcst_tree::MCTSTree;
use crate::ui::setup_ui::AgentActionButton;
use crate::{SelectedNPC, SimulationTree};
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::npc_components::npc_type::NPCType;

// Marker component for nodes
#[derive(Component)]
pub struct NodeMarker;

// Resource to manage the tree window state
#[derive(Resource, Default)]
pub struct DisplayTreeWindowState {
    pub is_open: bool,
    pub tree: Option<MCTSTree>,
}

// Add a system to toggle the visibility of the button based on the selected NPC
pub fn update_agent_action_button_visibility(
    selected_npc: Res<SelectedNPC>,
    npc_query: Query<&NPCBase>,
    mut button_query: Query<&mut Style, With<AgentActionButton>>,
) {
    if let Ok(mut button_style) = button_query.get_single_mut() {
        if let Some(selected_entity) = selected_npc.0 {
            if let Ok(npc_base) = npc_query.get(selected_entity) {
                // Show the button only if the selected NPC is an agent
                if matches!(npc_base.npc_type, NPCType::Agent) {
                    button_style.display = Display::Flex; // Show the button
                } else {
                    button_style.display = Display::None; // Hide the button
                }
            }
        } else {
            button_style.display = Display::None; // Hide the button if no NPC is selected
        }
    }
}

// Add a system to handle button interaction (e.g., open a window with "Test")
pub fn agent_action_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AgentActionButton>)>,
    mut window_state: ResMut<DisplayTreeWindowState>,
    simulation_tree: Res<SimulationTree>,
    selected_npc: Res<SelectedNPC>,
    npc_query: Query<(&Agent, &NPCBase)>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!("Button clicked!"); // Debug log to confirm the button is clicked

            if let Some(selected_entity) = selected_npc.0 {
                if let Ok((agent, npc_base)) = npc_query.get(selected_entity) {
                    let agent_id = agent.get_id();
                    println!("Selected agent ID: {}", agent_id); // Debug log for agent ID

                    if let Some(tree) = simulation_tree.get_tree(agent_id as i32) {
                        println!("Tree found for agent ID: {}", agent_id); // Debug log for tree existence

                        // Toggle the tree display window
                        if window_state.is_open {
                            window_state.is_open = false;
                            window_state.tree = None;
                            println!("Tree window closed."); // Debug log for window state
                        } else {
                            window_state.is_open = true;
                            window_state.tree = Some(tree.clone());
                            println!("Tree window opened."); // Debug log for window state
                        }
                    } else {
                        println!("No tree found for agent ID: {}", agent_id); // Debug log for missing tree
                    }
                } else {
                    println!("No agent found for selected entity."); // Debug log for missing agent
                }
            } else {
                println!("No NPC selected."); // Debug log for missing selection
            }
        }
    }
}