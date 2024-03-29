use bevy::prelude::*;
use crate::{entities::agent::{Agent, Status}, Backpropogate, MCSTCurrent, NpcActions, NpcActionsCopy, RunningFlag, ScoreTracker, SimulationFlag};
use super::{mcst::SimulationTree, mcst_tree::mcst_tree::MCTSTree};


pub fn check_finish(
    mut simulation_tree: ResMut<SimulationTree>,
    mut simulation_flag: ResMut<SimulationFlag>,
    mut running_flag: ResMut<RunningFlag>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_res: ResMut<NpcActions>,
    mut backpropogate_flag: ResMut<Backpropogate>,
    mut agent_query: Query<&mut Agent>, 
    mut mcstcurrent: ResMut<MCSTCurrent>,
    
){
    if simulation_flag.0 {
        let score_tracker = &mut score_tracker_res.0;
        let npc_actions = &mut npc_actions_res.0;
        let mut finished = true;

        for mut agent in agent_query.iter_mut(){
            for (action_id, action) in npc_actions.iter_mut(){
                if *action_id == agent.get_id(){
                    for (score_id, score) in score_tracker.iter_mut() {
                        if *score_id == agent.get_id(){
                            if !(*score < 0){
                                if agent.get_status() == Status::Idle {
                                    if action.is_empty() {
                                        *score = 0 - (agent.get_reward() as i32);
                                    } else {
                                        agent.set_action(action.pop_front().unwrap());
                                    }
                                }
                                finished = false;
                            }
                            break;
                        }
                    }
                    break;
                }
            }
        }
        //for (score_id, score) in score_tracker.iter_mut() {
            
        //    if *score != 0 {
        //println!("Current score: {}", score);
        //        for (action_id, action) in npc_actions.iter_mut() {
        //            if *action_id == *score_id {
        //                if action.is_empty() {
        //                    for agent in agent_query.iter_mut() {
        //                        if agent.get_id() == *score_id {
        //                            if agent.get_status() == Status::Idle {
        //                                *score = agent.get_reward();
        //                            } else {
        //                                finished = false;
        //                            }
        //                            break;
        //                        }
        //                    }
        //                } else {
        //                    finished = false;
        //                }
        //                break;
        //            }
        //        }
        //    }
        //}

        if finished {
            backpropogate_flag.0 = true;
            simulation_flag.0 = false;
            mcstcurrent.0 = mcstcurrent.0 + 1;
        }
    }
    if running_flag.0 {
        let mut min_size = u16::MAX;
        let mut max_size = 0;
        let mut less_than_100_height = false;
        let forest_guard = simulation_tree.get_forest();
       for (_, tree) in forest_guard.lock().unwrap().iter_mut(){
           let tree_height = tree.get_height() as u16;
           min_size = tree_height.min(min_size);
           max_size = tree_height.max(max_size);
           if tree_height <= 100{
               less_than_100_height = true;
           }
       }
        //If a tree has a difference of more than 10 actions to another 
        //tree or a tree has less than 100 actions in the tree left, restart mcst
        if less_than_100_height || (max_size - min_size) >= 10{
            println!("Entered into end of running phase");
            running_flag.0 = false;
        }
    }
}

pub fn backpropgate(
    mut backpropogate_flag: ResMut<Backpropogate>,
    mut tree: ResMut<SimulationTree>,
    mut agent_query: Query<&mut Agent>, 
    mut agent_copy: ResMut<Vec::<Agent>>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_copy_res: ResMut<NpcActionsCopy>,
    mut commands: Commands,
){
    if backpropogate_flag.0 {
        let npc_actions_copy = &mut npc_actions_copy_res.0;
        let score_tracker = &mut score_tracker_res.0;
        let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> = tree.get_forest();

        for agent in agent_query.iter_mut() {
            let agent_id = agent.get_id();
            if let Some((_tree_id, mcst_tree)) = forest_guard.lock().unwrap().iter_mut().find(|(tree_id, _)| *tree_id == agent_id) {
                for (score_id, score) in score_tracker.iter() {
                    if *score_id == agent_id {
                        for (action_id, action) in &mut *npc_actions_copy{
                            if *action_id == agent_id{
                                mcst_tree.backpropegate(action.clone(), (-1 * *score) as u32);
                            }
                        }
                        break;
                    }
                }
            } else {
                println!("No matching MCTSTree found for agent_id: {}", agent_id);
            }
        }

        restore_agents_from_vector(&mut commands, &mut agent_query, &mut agent_copy);
        backpropogate_flag.0 = false;
        *npc_actions_copy_res = NpcActionsCopy(Vec::new());
    }
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