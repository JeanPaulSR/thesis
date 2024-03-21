

use bevy::prelude::*;

use crate::{entities::agent::{Agent, Status}, Backpropogate, MCSTCurrent, NpcActions, NpcActionsCopy, ScoreTracker, SimulationFlag};

use super::{mcst::SimulationTree, mcst_tree::mcst_tree::MCTSTree};


pub fn check_finish(
    mut simulation_flag: ResMut<SimulationFlag>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_res: ResMut<NpcActions>,
    mut backpropogate_flag: ResMut<Backpropogate>,
    mut agent_query: Query<&mut Agent>, 
    mut mcstcurrent: ResMut<MCSTCurrent>,
    
){
    if !simulation_flag.0 {
        let score_tracker = &mut score_tracker_res.0;
        let npc_actions = &mut npc_actions_res.0;
        let mut finished = true;

        for (score_id, score) in score_tracker.iter_mut() {
            if *score != 0 {
                for (action_id, action) in npc_actions.iter_mut() {
                    if *action_id == *score_id {
                        if action.is_empty() {
                            for agent in agent_query.iter_mut() {
                                if agent.get_id() == *score_id {
                                    if agent.get_status() == Status::Idle {
                                        *score = agent.get_reward();
                                    } else {
                                        finished = false;
                                    }
                                    break;
                                }
                            }
                        } else {
                            finished = false;
                        }
                        break;
                    }
                }
            }
        }

        if finished {
            backpropogate_flag.0 = true;
            simulation_flag.0 = false;
            mcstcurrent.0 = mcstcurrent.0 + 1;
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
                        for (_action_id, action) in &mut *npc_actions_copy{
                            mcst_tree.backpropegate(action.clone(), *score);
                        }
                    } else {
                        println!("No matching Score found for agent_id: {}", agent_id);
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