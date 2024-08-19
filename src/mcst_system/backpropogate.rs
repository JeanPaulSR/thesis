use bevy::prelude::*;
use crate::{entities::agent::{Agent, Status}, Backpropogate, MCSTCurrent, MCSTFlag, NpcActions, NpcActionsCopy, RunningFlag, ScoreTracker, WorldSim};
use super::{mcst::{NpcAction, SimulationTree}, mcst_tree::mcst_tree::MCTSTree};



pub fn check_simulation_finish(
    mut simulation_tree: ResMut<SimulationTree>,
    mut mcst_flag: ResMut<MCSTFlag>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_res: ResMut<NpcActions>,
    mut backpropogate_flag: ResMut<Backpropogate>,
    mut agent_query: Query<&mut Agent>, 
    mut mcstcurrent: ResMut<MCSTCurrent>,
){
    if mcst_flag.0 {
        let score_tracker = &mut score_tracker_res.0;
        let npc_actions = &mut npc_actions_res.0;
        let mut finished = true;

        for (score_id, score) in score_tracker.iter_mut() {
            if score < &mut 0{
                let agent = agent_query.iter_mut().find(|agent| agent.get_id() == *score_id);
                match agent {
                    Some(mut agent) => {
                        if agent.get_status() != Status::Idle{
                            finished = false;
                            break
                        }
                    },
                    None => {
                        println!("Agent with score_id {} not found.", *score_id);
                    }
                }
            }
        }

        if finished {
            backpropogate_flag.0 = true;
            mcst_flag.0 = false;
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
    mut running_flag: ResMut<RunningFlag>,
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
        running_flag.0 = true;
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