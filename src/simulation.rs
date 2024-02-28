//For loop that takes all the agents

//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity
use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::AppExit;
use crate::mcst::MCTSNode;
use crate::World;
use crate::AgentMessages;
use crate::MonsterMessages;
use crate::TreasureMessages;
use crate::mcst;
use crate::entities;
use crate::mcst::MCTSTree;
use crate::perform_action;


use crate::entities::agent::Agent;



pub fn run_simulation(
    mut world: ResMut<World>,
    mut tree: ResMut<mcst::SimulationTree>,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
    mut iteration_counter: Local<i32>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut query: Query<&mut Agent>, 
){
    if tree.is_empty() {
        
        for mut agent in query.iter_mut() {
            let mut new_tree = mcst::MCTSTree::new_empty();
            new_tree.initialize_tree(agent.clone());
            tree.insert_tree(new_tree, agent.get_id());
            println!("Finished setup for agent {}", agent.get_id());
        }
    }

    //Need to do tree
    //Simulation
    //Back Propgation
    let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> = tree.get_forest();
    let mut npc_actions: Vec<(u32, Vec<mcst::NpcAction>)> = Vec::new();
    for agent in query.iter_mut() {
        let mut forest_lock = forest_guard.lock().unwrap();
        let index = (agent.get_id() - 1) as usize;
        if let Some((_, mcst_tree)) = forest_lock.get_mut(index) {
            let result = mcst_tree.selection_phase();
            npc_actions.push((agent.get_id(), result));
        } else {
            println!("It's empty inside here for {} for selection", agent.get_id());
        }
    }
    
    let mut world_copy = world.copy();


    // End Condition
    *iteration_counter += 1;
    // Check if it's time to end the simulation
    if *iteration_counter >= 10 {
        // Trigger AppExit event to end the program
        app_exit_events.send(AppExit);
    }
}
