use bevy::prelude::*;

use crate::{entities::agent::Agent, FinishedRunningFlag, NpcActions, NpcActionsCopy, RunningFlag, SimulationFlag};

use super::{mcst::SimulationTree, mcst_tree::mcst_tree::MCTSTree};


pub fn select_phase(
    mut simulation_tree: ResMut<SimulationTree>,
    mut simulation_flag: ResMut<SimulationFlag>,
    mut running_flag: ResMut<RunningFlag>,
    finished_running_flag: ResMut<FinishedRunningFlag>,
    mut agent_query: Query<&mut Agent>, 
    mut npc_actions_res: ResMut<NpcActions>,
    mut npc_actions_copy_res: ResMut<NpcActionsCopy>,
    mut iteration_counter2: Local<i32>,
){
    *iteration_counter2 += 1;
    if !simulation_flag.0 && !running_flag.0 {
        let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> = simulation_tree.get_forest();
        let npc_actions = &mut npc_actions_res.0;
        let npc_actions_copy = &mut npc_actions_copy_res.0;

        if finished_running_flag.0 {
            running_flag.0 = true;
            //Select actions from tree for each agent
            //TODO
        } else {
            simulation_flag.0 = true;
            //Selection phase for each agent, expansion is included in this phase. Max tree height is 255 currently
            for agent in agent_query.iter_mut() {
                let agent_id = agent.get_id();
                if let Some(tree_tuple) = forest_guard.lock().unwrap().iter_mut().find(|(simulation_tree, _)| *simulation_tree == agent_id) {
                    let (_, mcst_tree) = tree_tuple;
                    let result = mcst_tree.selection_phase();
                    npc_actions.push((agent_id, result.clone()));
                    npc_actions_copy.push((agent_id, result));
                    mcst_tree.calculate_height();
                } else {
                    println!("No matching MCTSTree found for agent_id: {}", agent_id);
                }
            }
        }
    }
}