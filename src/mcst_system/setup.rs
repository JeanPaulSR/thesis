use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::AppExit;
use crate::entities::agent::Agent;
use crate::mcst::NpcAction;
use crate::mcst_system::mcst_tree::mcst_tree::MCTSTree;
use crate::FinishedRunningFlag;
use crate::WorldSim;
use crate::{MCSTCurrent, MCSTTotal, RunningFlag, SimulationFlag, SimulationTotal, World,
            AgentMessages, MonsterMessages, TreasureMessages, mcst};
use crate::entities::agent;

pub fn setup_simulation(
    world: ResMut<World>,
    world_sim: ResMut<WorldSim>,
    mut agent_copy: ResMut<Vec::<Agent>>,
    mut tree: ResMut<mcst::SimulationTree>,
    iteration_total: ResMut<SimulationTotal>,
    mut iteration_counter: Local<i32>,
    mut mcst_current: ResMut<MCSTCurrent>,
    mcst_total: ResMut<MCSTTotal>,
    simulation_flag: ResMut<SimulationFlag>,
    running_flag: ResMut<RunningFlag>,
    mut finished_running_flag: ResMut<FinishedRunningFlag>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut agent_query: Query<&mut Agent>, 
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
    if !simulation_flag.0 && !running_flag.0 {
        *agent_copy = save_agents_to_vector(&mut agent_query);
        world_sim.copy_world(&world);
        if mcst_current.0 == mcst_total.0 {
            if finished_running_flag.0 {
                println!("Current iteration: {}", *iteration_counter);
                // End Condition
                *iteration_counter += 1;
                // Check if it's time to end the simulation
                if *iteration_counter >= iteration_total.0 + 1 {
                        // Trigger AppExit event to end the program
                    app_exit_events.send(AppExit);
                }
                finished_running_flag.0 = false;
                mcst_current.0 = 0;
            } else {
                finished_running_flag.0 = true;
            }

        }        
    }
}

// Function to save the state of agents from a query into a vector
fn save_agents_to_vector(
    query: &mut Query<&mut Agent>
) -> Vec<Agent> {
    let mut agent_backup = Vec::new();
    for agent in query.iter_mut() {
        agent_backup.push(agent.clone());
    }
    agent_backup
}