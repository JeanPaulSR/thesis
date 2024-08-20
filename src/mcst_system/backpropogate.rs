use super::{
    mcst::{NpcAction, SimulationTree},
    mcst_tree::mcst_tree::MCTSTree,
};
use crate::{
    entities::agent::{Agent, Status},
    AgentList, Backpropogate, MCSTCurrent, MCSTFlag, NpcActions, NpcActionsCopy, RunningFlag,
    ScoreTracker, WorldSim,
};
use bevy::prelude::*;

pub fn check_simulation_finish(
    mut simulation_tree: ResMut<SimulationTree>,
    mut mcst_flag: ResMut<MCSTFlag>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_res: ResMut<NpcActions>,
    mut backpropogate_flag: ResMut<Backpropogate>,
    mut agent_query: Query<&mut Agent>,
    mut mcstcurrent: ResMut<MCSTCurrent>,
) {
    if mcst_flag.0 {
        let score_tracker = &mut score_tracker_res.0;
        let npc_actions = &mut npc_actions_res.0;
        let mut finished = true;

        for (score_id, score) in score_tracker.iter_mut() {
            if score < &mut 0 {
                let agent = agent_query
                    .iter_mut()
                    .find(|agent| agent.get_id() == *score_id);
                match agent {
                    Some(mut agent) => {
                        if agent.get_status() != Status::Idle {
                            finished = false;
                            break;
                        }
                    }
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

pub fn backpropogate(
    mut backpropogate_flag: ResMut<Backpropogate>,
    mut tree: ResMut<SimulationTree>,
    mut agent_query: Query<(Entity, &mut Agent)>,
    mut agent_copy_res: ResMut<AgentList>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_copy_res: ResMut<NpcActionsCopy>,
    mut running_flag: ResMut<RunningFlag>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    if backpropogate_flag.0 {
        let agent_copy = agent_copy_res.0.clone();
        let npc_actions_copy = &mut npc_actions_copy_res.0;
        let score_tracker = &mut score_tracker_res.0;
        let forest_guard: &mut std::sync::Arc<std::sync::Mutex<Vec<(u32, MCTSTree)>>> =
            tree.get_forest();

        for (entity, mut agent) in agent_query.iter_mut() {
            let agent_id = agent.get_id(); // Call get_id on the `Agent` component

            if let Some((_tree_id, mcst_tree)) = forest_guard
                .lock()
                .unwrap()
                .iter_mut()
                .find(|(tree_id, _)| *tree_id == agent_id)
            {
                for (score_id, score) in score_tracker.iter() {
                    if *score_id == agent_id {
                        for (action_id, action) in &mut *npc_actions_copy {
                            if *action_id == agent_id {
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

        restore_agents_from_vector(
            commands,
            agent_query,
            agent_copy,
            &mut texture_atlases,
            &asset_server,
        );

        backpropogate_flag.0 = false;
        running_flag.0 = true;
        *npc_actions_copy_res = NpcActionsCopy(Vec::new());
    }
}

fn restore_agents_from_vector(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Agent)>,
    agent_backup: Vec<Agent>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
) {
    let mut backup_map = agent_backup
        .into_iter()
        .map(|agent| (agent.get_id(), agent))
        .collect::<std::collections::HashMap<_, _>>();

    // Update existing agents
    for (entity, mut agent) in query.iter_mut() {
        let agent_id = agent.get_id();

        if let Some(backup_agent) = backup_map.remove(&agent_id) {
            *agent = backup_agent;
        } else {
            // Remove entity if not in backup
            commands.entity(entity).despawn();
        }
    }

    // Spawn new entities for agents that were in the backup but not in the current world
    for backup_agent in backup_map.into_values() {
        let (x, y) = backup_agent.get_position();
        let x = x as f32 * 32.0; // Assuming you want to scale by 32.0
        let y = y as f32 * 32.0;

        // Create texture atlas and other resources if needed
        let texture_handle = asset_server.load("textures/agent.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle.clone(),
            Vec2::new(32.0, 32.0),
            1,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // Spawn new entity with the backup data
        let entity = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                ..Default::default()
            })
            .id();

        // Insert the restored agent into the world
        commands.entity(entity).insert(backup_agent);
    }
}
