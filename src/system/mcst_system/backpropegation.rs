use bevy::prelude::*;
use crate::system::mcst_tree::simulation_tree::SimulationTree;
use crate::npcs::agent::Agent;
use crate::{Backpropogate, MCSTFlag};

/// Checks if the MCST flag is true and performs backpropagation for each agent's tree.
pub fn backpropegate_system(
    mut mcst_flag: ResMut<MCSTFlag>,
    mut backpropegation_flag: ResMut<Backpropogate>,
    mut simulation_tree: ResMut<SimulationTree>,
    query: Query<&Agent>,
) {
    // Check if we are in the MCST phase
    if !mcst_flag.0 {
        return;
    }

    // Check if the backpropagation flag is true
    if !backpropegation_flag.0 {
        return;
    }

    // For each agent
    for agent in query.iter() {
        // Get the agent's ID
        let agent_id = agent.get_id();

        // For each tree in the simulation tree where the agent matches the key
        if let Some(tree) = simulation_tree.get_tree_mut(agent_id) {
            // Get the agent's current reward
            let reward = agent.get_reward();

            // Backpropagate it up the tree starting from the current node
            if let Some(current_node) = tree.get_current_node() {
                let mut current_node_lock = current_node.lock().unwrap();
                current_node_lock.backpropagate(reward);
            }
        }
    }

    backpropegation_flag.0 = false;
    mcst_flag.0 = false;
}
