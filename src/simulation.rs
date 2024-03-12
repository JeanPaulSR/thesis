//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity for now
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
    mut agent_query: Query<&mut Agent>, 
    mut commands: Commands,
){
    
    if tree.is_empty() {
        
        for mut agent in agent_query.iter_mut() {
            let mut new_tree = mcst::MCTSTree::new_empty();
            new_tree.initialize_tree(agent.clone());
            tree.insert_tree(new_tree, agent.get_id());
            println!("Finished setup for agent {}", agent.get_id());
        }
    } else {

        //Need to do tree
        //Simulation
        //Back Propgation
        
        
        let mut agent_backup = save_agents_to_vector(&mut agent_query);
        let mut world_copy = world.clone();
        
        //Set correct end condition
        //For loop for the MCTS process
        for i in 0..10{
            let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> = tree.get_forest();
            let mut npc_actions: Vec<(u32, Vec<mcst::NpcAction>)> = Vec::new();

            //Selection phase for each agent, expansion is included in this phase. Max tree height is 255 currently
            for agent in agent_query.iter_mut() {
                let mut forest_lock = forest_guard.lock().unwrap();
                let index = (agent.get_id() - 1) as usize;
                if let Some((_, mcst_tree)) = forest_lock.get_mut(index) {
                    let result = mcst_tree.selection_phase();
                    npc_actions.push((agent.get_id(), result));
                } else {
                    println!("It's empty inside here for {} for selection", agent.get_id());
                }
            }

            //TESTING ONLY
            if !agent_messages.is_empty(){
                panic!("An error occurred!");
            }
            if npc_actions.is_empty(){
                panic!("An error occurred! npc_actions length is {}", npc_actions.len());
            }

            //Simulation phase using all agents (Currently in progress)
            simulation(&mut world, &mut agent_messages, &mut monster_messages, &mut treasure_messages,
                        &mut npc_actions);
            //Backpropegation phase

            //Restore system for next selection phase
            restore_agents_from_vector(&mut commands, &mut agent_query, &mut agent_backup);
            *world = world_copy.clone();
        }

            //Select action, and prune other branches
            //Set the actions
    }

    // End Condition
    *iteration_counter += 1;
    // Check if it's time to end the simulation
    if *iteration_counter >= 3 {
        // Trigger AppExit event to end the program
        app_exit_events.send(AppExit);
    }
}


pub fn simulation(
    world: &mut ResMut<World>,
    agent_messages: &mut ResMut<AgentMessages>,
    monster_messages: &mut ResMut<MonsterMessages>,
    treasure_messages: &mut ResMut<TreasureMessages>,
    npc_actions: &mut Vec<(u32, Vec<mcst::NpcAction>)>,
) {
    println!("NEW LOOP");
    let npc_actions_clone = npc_actions.clone(); // Clone npc_actions

    let mut int = 0;
    let mut int2 = 0;

    while !is_finished(npc_actions) {
        println!("NEW ITERATION");
        for index in 0..npc_actions.len() {
            let (id, actions) = &mut npc_actions[index];
            if let Some(action) = actions.pop() {
                //let compare: usize = (*id).try_into().unwrap();
                //if index + 1 != compare {
                //    println!("Vector current length is {}", npc_actions_clone.len());
                //    panic!("An error occurred! With id: {}", id);
                //}
                // Optionally, you can handle the popped element here if needed
                // For example, you can store it in a variable or print it
                // println!("Popped element: {:?}", popped_element);
                println!("Vector {} current length is {}", id, actions.len());
            } else {
                // Handle the case when the vector is empty
                if *id == 2 {
                    println!(
                        "Vector is not empty! {} , {}",
                        id,
                        { let tmp = int2; int2 += 1; tmp }
                    );
                }
                if *id == 1 {
                    println!(
                        "Vector is empty! {} , {}",
                        id,
                        { let tmp = int; int += 1; tmp }
                    );
                }
            }
        }
    }
}

fn is_finished(npc_actions:  &mut Vec<(u32, Vec<mcst::NpcAction>)>) -> bool{
    let finished_flag = false;
    for (key, value) in npc_actions{
        if !value.is_empty(){
            return false
        }
    }
    true
}

// Function to save the state of agents from a query into a vector
fn save_agents_to_vector(query: &mut Query<&mut Agent>) -> Vec<Agent> {
    let mut agent_backup = Vec::new();
    for mut agent in query.iter_mut() {
        agent_backup.push(agent.clone());
    }
    agent_backup
}

fn restore_agents_from_vector(commands: &mut Commands, query: &mut Query<&mut Agent>, agent_backup: &mut Vec<Agent>) {
    let mut backup_copy = agent_backup.clone();

    for mut agent_entity in query.iter_mut() {
        let agent_id = agent_entity.get_id();

        if let Some(backup_index) = backup_copy.iter().position(|backup_agent| backup_agent.get_id() == agent_id) {
            let backup_agent = backup_copy.remove(backup_index);
            *agent_entity = backup_agent;
        } else {
            commands.entity(agent_entity.get_entity()).despawn();
        }
    }

    for agent in backup_copy {
        commands.spawn().insert(agent).id();
    }
}