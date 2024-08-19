use bevy::prelude::*;

use crate::{entities::agent::Agent, FinishedRunningFlag, FinishedSelectionPhase, NpcActions, NpcActionsCopy, RunningFlag, MCSTFlag};

use super::{mcst::SimulationTree, mcst_tree::mcst_tree::MCTSTree};



pub fn select_expansion(
    mut simulation_tree: ResMut<SimulationTree>,
    mut mcst_flag: ResMut<MCSTFlag>,
    mut running_flag: ResMut<RunningFlag>,
    finished_running_flag: ResMut<FinishedRunningFlag>,
    mut agent_query: Query<&mut Agent>, 
    mut npc_actions_res: ResMut<NpcActions>,
    mut npc_actions_copy_res: ResMut<NpcActionsCopy>,

    mut selection_flag: ResMut<FinishedSelectionPhase>,

){
    if mcst_flag.0 && !selection_flag.0{
        let forest_guard = simulation_tree.get_forest();
        *npc_actions_res = NpcActions(Vec::new());
        let npc_actions = &mut npc_actions_res.0;
        *npc_actions_copy_res = NpcActionsCopy(Vec::new());
        let npc_actions_copy = &mut npc_actions_copy_res.0;
        
        //Selection phase for each agent, expansion is included in this phase. Max tree height is 255 currently
        for agent in agent_query.iter_mut() {
                
            let agent_id = agent.get_id();
            if let Some(tree_tuple) = forest_guard.lock().unwrap().
                    iter_mut().find(|(simulation_tree, _)| *simulation_tree == agent_id) {
                let (_, mcst_tree) = tree_tuple;
                let result = mcst_tree.selection_phase();
                npc_actions.push((agent_id, result.clone()));
                npc_actions_copy.push((agent_id, result));
                mcst_tree.calculate_height();
            } else {
                println!("No matching MCTSTree found for agent_id: {}", agent_id);
            }
        }
        selection_flag.0 = true;
    }

}