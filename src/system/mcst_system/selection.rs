use bevy::ecs::system::{Query, Res, ResMut};

use crate::{
    system::mcst_tree::{mcst_node::{Node, NodeType}, simulation_tree::SimulationTree},
    npcs::{agent::Agent, npc_components::npc_status::Status},
    MCSTFlag,
};

/*To run this program we have two phases
    We start off with the MCST phase, which will attempt to create MCST action desicion trees for every agent
    Then, we will enter into the action taking phase, which will choose the best action by those provided by the MCST trees.
    The action phase runs until we reach a node that has a low exploration number. And then start mcst again until that number is high enough
    When we are in the action phase and we select a move, we trim the tree by selecting that action node and setting it as the new root of the tree


    MCST Explanation
        MCST takes the core sections of the MCST algorithm.
        Selection
            Firstly, the selection phase will happen during the MCST phase. Instead of finding the leaf node before Mcst
            we will be finding it throughout the mcst phase. Since this is Open Loop MCST then the simulation will be happening at the same time.
            How this will work:
                1. We will start at the root node of the tree
                2. We will select an action using Node's select_action()
                3. We will run the simulation, until the action is finished
                    3.a While running the simulation, we will add nodes to the tree called InformationNode when an action from another npc affects the current one
                4. We will continue this process until we reach a leaf node, and then do expansion

        Expansion
            Expansion phase adds a new node, based on normal MCST, but after this we do not save any new nodes. Instead we save only the result at the end of the simulation
            and backpropegation

        Simulation
            We use the actions we have been given by selection until we reach the expansion phase. After that we take random actions until the end condition is met

        ackpropegation
            We will backpropegate the result of the simulation to the root node, and then we will go to the next agent and repeat the process
            The backpropegation will be done by going through all the nodes in the tree, and adding the result to each node's total reward and visits. This is done by using a queue, which is a vector of tuples (node, result)
            The result is added to each node's total reward and visits, and then we go to the next node in the queue. This is done until we reach the root node, and then we stop.
*/
/*
    Check if the agent is currently idle. If its not, continue
    Check if the npc action is currently none, send an error if it isnt because it should never be idle with npc_action not none
    Check if the flag for the end of the selection phase is ended
        If it is, then we select an action at random
    In the current tree tied to the agent, we get the current node
        1. Check for if a global end condition is met. For now if depth is 256 for the current node we stop the selection phase
        2. We will select an action using the select_action() method
        3. We check if the current node is a leaf node. If it is, we mark the mcst tree with the flag that simulation phase has ended

    We assume that other systems will handle when the agent is idle, when the total simulation will end, etc
   */
pub fn selection_system(
    mcst_flag: Res<MCSTFlag>,
    mut simulation_tree: ResMut<SimulationTree>,
    mut agents: Query<&mut Agent>,
) {
    //If not in MCST phase, skip this step
    if !mcst_flag.0 {
        return;
    }

    for mut agent in agents.iter_mut() {
        let agent_id = agent.get_id();

        // 1. Check if the agent is currently idle. If not, continue.
        if agent.get_status() != Status::Idle {
            continue;
        }

        // 2. Get the MCTS tree for the current agent.
        if let Some(tree) = simulation_tree.get_tree_mut(agent_id) {
            let mut set_ready_for_expansion = false;
            let mut set_in_selection_phase = false;
            let mut new_current_node = None;

            if let Some(current_node) = tree.get_current_node() {
                let current_node_lock = current_node.lock().unwrap();

                // 3.a. Check for a global end condition (e.g., depth of 256).
                if current_node_lock.depth >= 256 {
                    set_in_selection_phase = false;
                } else if current_node_lock.children.is_empty() {
                    // 3.b. If the current node has no children, mark it for expansion.
                    set_ready_for_expansion = true;
                    set_in_selection_phase = false;
                } else {
                    // 3.c. Otherwise, select the best action using UCT.
                    new_current_node = select_best_action_using_uct(&mut agent, &current_node_lock, agent_id);
                }
            }

            // Apply changes to the tree outside the scope of current_node_lock.
            if set_ready_for_expansion {
                tree.set_ready_for_expansion(true);
            }
            if !set_in_selection_phase {
                tree.set_in_selection_phase(false);
            }
            if let Some(new_node) = new_current_node {
                tree.set_current_node(new_node);
            }
        }
    }
}

/// Helper function to select the best action using UCT.
/// Returns the selected action node if successful.
pub fn select_best_action_using_uct(
    agent: &mut Agent,
    current_node_lock: &std::sync::MutexGuard<Node>,
    agent_id: i32,
) -> Option<std::sync::Arc<std::sync::Mutex<Node>>> {
    if let Some(best_action_node) = current_node_lock.select_action(1.414) {
        if let Ok(best_action_node_lock) = best_action_node.lock() {
            if let NodeType::ActionNode { action } = best_action_node_lock.node_type {
                agent.set_action(action);
            }
        }
        Some(best_action_node)
    } else {
        eprintln!(
            "Agent {}: Failed to select an action using UCT.",
            agent_id
        );
        None
    }
}