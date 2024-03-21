//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity for now
use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::AppExit;
use crate::mcst::NpcAction;
use crate::mcst_system::mcst_tree::mcst_tree::MCTSTree;
use crate::MCSTCurrent;
use crate::MCSTTotal;
use crate::RunningFlag;
use crate::SimulationFlag;
use crate::SimulationTotal;
use crate::World;
use crate::AgentMessages;
use crate::MonsterMessages;
use crate::TreasureMessages;
use crate::mcst;
use crate::entities::agent;




use crate::entities::agent::Agent;
use crate::WorldSim;



pub fn setup_simulation(
    world: ResMut<World>,
    world_sim: ResMut<WorldSim>,
    mut agent_copy: ResMut<Vec::<Agent>>,
    mut tree: ResMut<mcst::SimulationTree>,
    _agent_messages: ResMut<AgentMessages>,
    _monster_messages: ResMut<MonsterMessages>,
    _treasure_messages: ResMut<TreasureMessages>,
    iteration_total: ResMut<SimulationTotal>,
    mut iteration_counter: Local<i32>,
    simulation_flag: ResMut<SimulationFlag>,
    running_flag: ResMut<RunningFlag>,
    _mcst_current: ResMut<MCSTCurrent>,
    _mcst_total: ResMut<MCSTTotal>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut agent_query: Query<&mut Agent>, 
    _commands: Commands,
){
    //If tree is empty, it is the first iteration
    if tree.is_empty() {
        
        for agent in agent_query.iter_mut() {
            let mut new_tree = MCTSTree::new_empty();
            new_tree.initialize_tree(agent.clone());
            tree.insert_tree(new_tree, agent.get_id());
            println!("Finished setup for agent {}", agent.get_id());
        }
    }

    //Need to do tree
    //Simulation
    //Back Propgation
    //MCTS PHASE -> Selection -> Simulation (Skips setup during this time) -> Execution Stage
    //If the simulation is not running, setup for simulation
    //If Simulating
        //Do nothing
    //If not simulating
        //Check if currently running program
            //If not running program, Check if in mcst phase (check if mcst iteration is equal to total)
                //If in mcst phase, check if end condition of mcst phase met
                    //If not end of mcst phase, new selection phase
                //If not in mcst phase, check if program finished
                    //If program finished, check if end condition met
                        //If end condition met, end program
    if !simulation_flag.0 && !running_flag.0 {
        println!("Current iteration: {}", *iteration_counter);
        // End Condition
        *iteration_counter += 1;
        // Check if it's time to end the simulation
        if *iteration_counter >= iteration_total.0 + 1 {
            // Trigger AppExit event to end the program
            app_exit_events.send(AppExit);
        }

        *agent_copy = save_agents_to_vector(&mut agent_query);
        world_sim.copy_world(&world);
        
        //Set correct end condition
        //For loop for the MCTS process
        
    }
    let _world_copy = world_sim.get_world();

    //MCTS Counter and Current Counter
    //Selection phase
    //Expansion phase
    //Simulation phase
    //Backpropogation


    //True simulation



    //Set 
    //for i in 0..10{
    //    let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> = tree.get_forest();
    //    let mut npc_actions: Vec<(u32, Vec<mcst::NpcAction>)> = Vec::new();

    //    //Selection phase for each agent, expansion is included in this phase. Max tree height is 255 currently
    //    for agent in agent_query.iter_mut() {
    //        let mut forest_lock = forest_guard.lock().unwrap();
    //        let index = (agent.get_id() - 1) as usize;
    //        if let Some((_, mcst_tree)) = forest_lock.get_mut(index) {
    //            let result = mcst_tree.selection_phase();
    //            npc_actions.push((agent.get_id(), result));
    //        } else {
    //            println!("It's empty inside here for {} for selection", agent.get_id());
    //        }
    //    }

    //    //TESTING ONLY
    //    if !agent_messages.is_empty(){
    //        panic!("An error occurred!");
    //    }
    //    if npc_actions.is_empty(){
    //        panic!("An error occurred! npc_actions length is {}", npc_actions.len());
    //    }

    //    //Simulation phase using all agents (Currently in progress)
    //    simulation(&mut world, &mut agent_messages, &mut monster_messages, &mut treasure_messages,
    //                &mut npc_actions,&mut  agent_query, &mut commands);
    //    //Backpropegation phase

    //    //Restore system for next selection phase
    //    restore_agents_from_vector(&mut commands, &mut agent_query, &mut agent_copy);
    //    *world = world_copy.clone();
    //}

        //Select action, and prune other branches
        //Set the actions
}


pub fn run_simulation(
    _world_sim: ResMut<WorldSim>,
    _agent_copy: ResMut<Vec::<Agent>>,
    _tree: ResMut<mcst::SimulationTree>,
    _agent_messages: ResMut<AgentMessages>,
    _monster_messages: ResMut<MonsterMessages>,
    _treasure_messages: ResMut<TreasureMessages>,
    _mcst_total: ResMut<MCSTTotal>,
    _agent_query: Query<&mut Agent>, 
){

}
/*

Create a list of flags that mark if it has reached the end
Check if it is currently 
If not

*/
pub fn simulation(
    _world: &mut ResMut<World>,
    _agent_messages: &mut ResMut<AgentMessages>,
    _monster_messages: &mut ResMut<MonsterMessages>,
    _treasure_messages: &mut ResMut<TreasureMessages>,
    npc_actions: &mut Vec<(u32, Vec<NpcAction>)>,
    query: &mut Query<&mut Agent>,
    _commands: &mut Commands,
) {
    //println!("NEW LOOP");
    //let npc_actions_clone = npc_actions.clone();
    //Set to false if not finished yet
    let mut finished_flags: Vec<bool> = npc_actions.iter().map(|_| false).collect();

    let _int = 0;
    let _int2 = 0;

    while !is_finished(&mut finished_flags) {
        //println!("NEW ITERATION");
        for index in 0..npc_actions.len() {
            let (action_id, actions) = &mut npc_actions[index];
            for mut agent in query.iter_mut() {
                if action_id == &mut agent.get_id(){
                    if agent.get_status() == agent::Status::Idle{
                        if let Some(action) = actions.pop() {
                            agent.set_action(action);
                            if actions.is_empty(){
                                if let Some(flag) = finished_flags.get_mut(index) {
                                    *flag = true;
                                } else {
                                    panic!("Index out of bounds.");
                                }
                            }
                        } else {
                            //agent.set_random_action();
                        }
                        //agent.calculate_target();
                    }
                }

            }
            
        }
        //perform action
        //systems::agent_message_system(agent_messages, monster_messages, world, query, commands);
        //messages
    }
}

fn is_finished(flags: &mut Vec<bool>) -> bool {
    for &flag in flags.iter() {
        if !flag {
            return false;
        }
    }
    true
}

// Function to save the state of agents from a query into a vector
fn save_agents_to_vector(query: &mut Query<&mut Agent>) -> Vec<Agent> {
    let mut agent_backup = Vec::new();
    for agent in query.iter_mut() {
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