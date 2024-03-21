use bevy::prelude::*;

use crate::{entities::agent::Agent, NpcActions, NpcActionsCopy, RunningFlag, SimulationFlag};

use super::{mcst::SimulationTree, mcst_tree::mcst_tree::MCTSTree};


pub fn select_phase(
    _agent_copy: ResMut<Vec::<Agent>>,
    mut tree: ResMut<SimulationTree>,
    simulation_flag: ResMut<SimulationFlag>,
    running_flag: ResMut<RunningFlag>,
    mut agent_query: Query<&mut Agent>, 
    mut npc_actions_res: ResMut<NpcActions>,
    mut npc_actions_copy_res: ResMut<NpcActionsCopy>,
    _commands: Commands,
){
    if !simulation_flag.0 && !running_flag.0 {
        let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> = tree.get_forest();
        //let mut npc_actions: Vec<(u32, Vec<mcst::NpcAction>)> = Vec::new();
        let npc_actions = &mut npc_actions_res.0;
        let npc_actions_copy = &mut npc_actions_copy_res.0;

        //Selection phase for each agent, expansion is included in this phase. Max tree height is 255 currently
        for agent in agent_query.iter_mut() {
            let agent_id = agent.get_id();
            if let Some(tree_tuple) = forest_guard.lock().unwrap().iter_mut().find(|(tree_id, _)| *tree_id == agent_id) {
                let (_, mcst_tree) = tree_tuple;
                let result = mcst_tree.selection_phase();
                npc_actions.push((agent_id, result.clone()));
                npc_actions_copy.push((agent_id, result));
            } else {
                println!("No matching MCTSTree found for agent_id: {}", agent_id);
            }
        }
    }
}