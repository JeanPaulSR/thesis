use bevy::ecs::system::{Query, Res, ResMut};

use crate::{
    system::{mcst_tree::{mcst_node::NodeType, simulation_tree::SimulationTree}, mcst_system::selection::select_best_action_using_uct},
    npcs::{agent::Agent, npc_components::npc_action::NpcAction},
    MCSTFlag, // Import the helper function
};

pub fn expansion_system(
    mcst_flag: Res<MCSTFlag>,
    mut simulation_tree: ResMut<SimulationTree>,
    mut agents: Query<&mut Agent>,
) {
    //If not in MCST phase, skip this step
    if !mcst_flag.0 {
        return;
    }
    
    for (agent_id, tree) in simulation_tree.trees.iter_mut() {
        // Find the corresponding agent for the current tree.
        if let Some(mut agent) = agents.iter_mut().find(|a| a.get_id() == *agent_id) {
            // Check if the tree is ready for expansion.
            if tree.is_ready_for_expansion() {
                if let Some(current_node) = tree.get_current_node().cloned() {
                    let mut current_node_lock = current_node.lock().unwrap();

                    // Expand the current node by adding all possible actions as children.
                    for action in NpcAction::iter() {
                        if !current_node_lock.has_child(&NodeType::ActionNode { action }) {
                            current_node_lock.expand(action);
                        }
                    }

                    // Drop the lock on the current node before performing UCT selection.
                    drop(current_node_lock);

                    // Perform UCT selection for the next action using the helper function.
                    if let Some(best_action_node) =
                        select_best_action_using_uct(&mut agent, &current_node.lock().unwrap(), *agent_id)
                    {
                        tree.set_current_node(best_action_node);
                    } else {
                        eprintln!(
                            "Agent {}: Failed to select the best action during expansion.",
                            agent_id
                        );
                    }

                    // Mark the tree as no longer ready for expansion.
                    tree.set_ready_for_expansion(false);
                }
            }
        }
    }
}