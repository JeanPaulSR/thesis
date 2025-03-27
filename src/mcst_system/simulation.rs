use std::collections::HashMap;

//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity for now
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::entities::agent::Agent;
use crate::entities::agent::{Status, Target};
use crate::entities::monster::Monster;
use crate::errors::MyError;
use crate::mcst_system::mcst::NpcAction;
use crate::movement::find_path;
use crate::{
    AgentMessages, FinishedSelectingActions, GameWorld, MCSTFlag, MonsterMessages, NpcActions, RunningFlag, ScoreTracker, TreasureMessages, WorldSim,
};

use super::mcst::SimulationTree;
use super::systems::{
    send_agent_message, send_treasure_message, AgentMessage, MessageType,
};

//Missing what to do if target is gone
pub fn set_mcst_actions(
    world_sim: ResMut<WorldSim>,
    mcst_flag: ResMut<MCSTFlag>,
    mut finished_selecting: ResMut<FinishedSelectingActions>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    mut npc_actions_res: ResMut<NpcActions>,
    mut agent_query: Query<&mut Agent>,
    mut monster_query: Query<&mut Monster>,
) {
    if mcst_flag.0 {
        let score_tracker = &mut score_tracker_res.0;
        let npc_actions = &mut npc_actions_res.0;
        let mut finished = true;

        let world = &world_sim.0;
        let closest_monster_vec = world.find_closest_monsters();
        let closest_treasure_vec = world.find_closest_treasures();
        let closest_village_vec = world.find_closest_villages();

        let mut agent_ids = Vec::new();
        let mut agent_positions = Vec::new();
        let mut agent_status_map: HashMap<u32, Status> = HashMap::new();
        let mut agent_attack_id_map: HashMap<u32, u32> = HashMap::new();
        for agent in agent_query.iter_mut() {
            agent_ids.push(agent.get_id());
            agent_positions.push((agent.get_id(), agent.get_position()));
            agent_status_map.insert(agent.get_id(), agent.get_status());
            if agent.get_target() == Target::Agent {
                agent_attack_id_map.insert(agent.get_id(), agent.get_agent_target_id());
            } else {
                agent_attack_id_map.insert(agent.get_id(), agent.get_monster_target_id());
            }
        }
        let mut monster_ids = Vec::new();
        let mut monster_status_map: HashMap<u32, Status> = HashMap::new();
        let mut monster_attack_id_map: HashMap<u32, u32> = HashMap::new();
        for monster in monster_query.iter_mut() {
            monster_ids.push(monster.get_id());
            monster_status_map.insert(monster.get_id(), monster.get_status());
            monster_attack_id_map.insert(monster.get_id(), monster.get_target_id());
        }

        'agents: for mut agent in agent_query.iter_mut() {
            let agent_npc_action = agent.get_action();
            if agent.get_status() == Status::Idle {
                for (score_id, score) in score_tracker.iter_mut() {
                    if agent.get_id() == *score_id {
                        if *score < 0 {
                            continue 'agents;
                        } else {
                            for (actions_id, agent_actions) in npc_actions.iter_mut() {
                                if agent.get_id() == *actions_id {
                                    if agent_actions.is_empty() {
                                        *score = 0 - (agent.get_reward() as i32);
                                        agent.set_status(Status::Finished);
                                    } else {
                                        finished = false;
                                        let current_action = agent_actions.pop_front().unwrap();
                                        agent.set_action(current_action);
                                        set_mcst_agent_actions(
                                            &mut agent,
                                            current_action,
                                            world,
                                            agent_ids.clone(),
                                            closest_monster_vec.clone(),
                                            closest_treasure_vec.clone(),
                                            closest_village_vec.clone(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            } else if agent.get_status() == Status::Retaliating {
                let target_id = agent.get_retaliation_target_id();
                let is_in_vector;
                let target_status;
                let attack_target_id;
                if agent.get_retaliation_target() == Target::Agent {
                    is_in_vector = agent_ids.contains(&target_id);
                    target_status = agent_status_map.get(&target_id).unwrap();
                    attack_target_id = agent_attack_id_map.get(&target_id).unwrap();
                } else {
                    is_in_vector = monster_ids.contains(&target_id);
                    target_status = monster_status_map.get(&target_id).unwrap();
                    attack_target_id = monster_attack_id_map.get(&target_id).unwrap();
                }
                if is_in_vector
                    && *target_status == Status::Attacking
                    && *attack_target_id == agent.get_id()
                {
                    let status = agent.flight_or_fight(target_id, agent.get_retaliation_target() == Target::Monster);
                    if status == Status::Fleeing {
                        agent.set_status(Status::Fleeing);
                    }
                } else {
                    set_current_target(&mut agent, world);
                    set_mcst_agent_actions(
                        &mut agent,
                        agent_npc_action,
                        world,
                        agent_ids.clone(),
                        closest_monster_vec.clone(),
                        closest_treasure_vec.clone(),
                        closest_village_vec.clone(),
                    );
                }
            } else if agent.get_status() == Status::Fleeing {
                let tile_target = agent.get_tile_target().unwrap();
                if agent.get_position() == tile_target {
                    agent.set_status(Status::Recovering);
                }
            } else if agent.get_status() == Status::Recovering {
                if agent.get_energy() == agent.get_max_energy() {
                    set_current_target(&mut agent, world);
                    agent.set_status(Status::RequiresInstruction);
                }
            } else if agent.get_status() == Status::RequiresInstruction {
                set_mcst_agent_actions(
                    &mut agent,
                    agent_npc_action,
                    world,
                    agent_ids.clone(),
                    closest_monster_vec.clone(),
                    closest_treasure_vec.clone(),
                    closest_village_vec.clone(),
                );
            } else {
                finished = false;
                println!("Error assigning actions to agents in simulation");
            }
        }
        if finished {
            finished_selecting.0 = true;
        }
    }
}

fn set_mcst_agent_actions(
    agent: &mut Mut<Agent>,
    npc_action: NpcAction,
    world: &GameWorld,
    agent_ids: Vec<u32>,
    closest_monster_vec: Vec<(u32, u32, (u32, u32))>,
    closest_treasure_vec: Vec<(u32, u32, (u32, u32))>,
    closest_village_vec: Vec<(u32, (u32, u32))>,
) {
    match npc_action {
        NpcAction::AttackAgent => {
            let target_id = agent.calculate_best_agent(NpcAction::AttackAgent, &agent_ids);
            agent.set_agent_target_id(target_id);
            agent.set_target(Target::Agent);
            set_current_target(agent, world);
        }
        NpcAction::AttackMonster => {
            if let Some(&(_agent_id, target_id, position)) = closest_monster_vec
                .iter()
                .find(|&&(id, _, _)| id == agent.get_id())
            {
                agent.set_monster_target_id(target_id);
                agent.set_tile_target(Some(position));
                agent.set_target(Target::Monster);
            }
        }
        NpcAction::Steal => {
            let target_id = agent.calculate_best_agent(NpcAction::Steal, &agent_ids);
            agent.set_agent_target_id(target_id);
            agent.set_target(Target::Agent);
            set_current_target(agent, world);
        }
        NpcAction::TreasureHunt => {
            if let Some(&(_agent_id, target_id, position)) = closest_treasure_vec
                .iter()
                .find(|&&(id, _, _)| id == agent.get_id())
            {
                agent.set_treasure_target_id(target_id);
                agent.set_target(Target::Treasure);
                agent.set_tile_target(Some(position));
            }
        }
        NpcAction::Talk => {
            let target_id = agent.calculate_best_agent(NpcAction::Talk, &agent_ids);
            agent.set_agent_target_id(target_id);
            agent.set_target(Target::Agent);
            set_current_target(agent, world);
        }
        NpcAction::Rest => {
            if let Some(&(_agent_id, target_pos)) = closest_village_vec
                .iter()
                .find(|&&(id, _)| id == agent.get_id())
            {
                agent.set_tile_target(Some(target_pos));
                agent.set_target(Target::Tile);
            }
        }
        _ => {}
    }
}

pub fn set_actual_simulation_actions(
    mut game_world: ResMut<GameWorld>,
    running_flag: ResMut<RunningFlag>,
    mut tree: ResMut<SimulationTree>,
    finished_selecting: ResMut<FinishedSelectingActions>,
    score_tracker_res: ResMut<ScoreTracker>,
    mut agent_query: Query<&mut Agent>,
) {
    if running_flag.0 {
        let world = game_world.as_mut();

        let closest_monster_vec = world.find_closest_monsters();
        let closest_treasure_vec = world.find_closest_treasures();
        let closest_village_vec = world.find_closest_villages();

        let mut agent_ids = Vec::new();
        let mut agent_positions = Vec::new();
        for agent in agent_query.iter_mut() {
            agent_ids.push(agent.get_id());
            agent_positions.push((agent.get_id(), agent.get_position()));
        }

        let mut agent_status_map: HashMap<u32, Status> = HashMap::new();
        for agent in agent_query.iter() {
            agent_status_map.insert(agent.get_id(), agent.get_status());
        }
        for mut agent in agent_query.iter_mut() {
            let agent_id = agent.get_id();

            if agent.get_status() == Status::Idle {
                let forest_guard = tree.get_forest();
                if let Some((_, mcst_tree)) = forest_guard
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .find(|(tree_id, _)| *tree_id == agent_id)
                {
                    //println!("{}",mcst_tree.get_height());
                    let selected_action = mcst_tree.choose_action();
                    // Set the selected child node as the new root and prune others
                    mcst_tree.set_selected_node_as_root(selected_action);

                    match selected_action {
                        NpcAction::AttackAgent => {
                            let target_id =
                                agent.calculate_best_agent(NpcAction::AttackAgent, &agent_ids);
                            agent.set_agent_target_id(target_id);
                            agent.set_target(Target::Agent);
                            set_current_target(&mut agent, world);
                        }
                        NpcAction::AttackMonster => {
                            if let Some(&(_agent_id, target_id, position)) = closest_monster_vec
                                .iter()
                                .find(|&&(id, _, _)| id == agent.get_id())
                            {
                                agent.set_monster_target_id(target_id);
                                agent.set_tile_target(Some(position));
                                agent.set_target(Target::Monster);
                            }
                        }
                        NpcAction::Steal => {
                            let target_id =
                                agent.calculate_best_agent(NpcAction::Steal, &agent_ids);
                            agent.set_agent_target_id(target_id);
                            agent.set_target(Target::Agent);
                            set_current_target(&mut agent, world);
                        }
                        NpcAction::TreasureHunt => {
                            if let Some(&(_agent_id, target_id, position)) = closest_treasure_vec
                                .iter()
                                .find(|&&(id, _, _)| id == agent.get_id())
                            {
                                agent.set_treasure_target_id(target_id);
                                agent.set_target(Target::Treasure);
                                agent.set_tile_target(Some(position));
                            }
                        }
                        NpcAction::Talk => {
                            let target_id = agent.calculate_best_agent(NpcAction::Talk, &agent_ids);
                            agent.set_agent_target_id(target_id);
                            agent.set_target(Target::Agent);
                            set_current_target(&mut agent, world);
                        }
                        NpcAction::Rest => {
                            if let Some(&(_agent_id, target_pos)) = closest_village_vec
                                .iter()
                                .find(|&&(id, _)| id == agent.get_id())
                            {
                                agent.set_tile_target(Some(target_pos));
                                agent.set_target(Target::Tile);
                            }
                        }
                        _ => {}
                    }
                } else {
                    println!("No matching MCTSTree found for agent_id: {}", agent_id);
                }
            } else if agent.get_status() == Status::Retaliating {
                let mut monster = true;
                if agent.get_retaliation_target() == Target::Agent {
                    monster = false;
                }
                let target_id = &agent.get_retaliation_target_id();
                let is_in_vector = agent_ids.contains(target_id);

                let target_status = agent_status_map.get(&target_id).unwrap();
                if is_in_vector && *target_status == Status::Attacking {
                    let status = agent.flight_or_fight(*target_id, monster);
                    if status == Status::Fleeing {
                        agent.set_status(Status::Fleeing);
                    }
                } else {
                    let tile_target = agent.get_tile_target().unwrap();
                    if agent.get_position() == tile_target {
                        agent.set_status(Status::Working);
                    } else {
                        agent.set_status(Status::Moving);
                    }
                }
            } else if agent.get_status() == Status::Fleeing {
                let tile_target = agent.get_tile_target().unwrap();
                if agent.get_position() == tile_target {
                    agent.set_status(Status::Recovering);
                }
            } else if agent.get_status() == Status::Recovering {
                if agent.get_energy() == agent.get_max_energy() {
                    //GET POSITION OF TARGET AND REASSIGN
                    let tile_target = agent.get_tile_target().unwrap();
                    if agent.get_position() == tile_target {
                        if agent.get_action() == NpcAction::AttackAgent
                            || agent.get_action() == NpcAction::AttackMonster
                        {
                            agent.set_status(Status::Attacking);
                        } else {
                            agent.set_status(Status::Working);
                        }
                    } else {
                        agent.set_status(Status::Moving);
                    }
                }
            }
        }
    }
}

pub fn run_actual(
    running_flag: ResMut<RunningFlag>,
    mcst_flag: ResMut<MCSTFlag>,
    mut agent_query: Query<&mut Agent>,
    world_sim: ResMut<WorldSim>,
    world_actual: ResMut<GameWorld>,
    commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
) {
    let world;
    if mcst_flag.0 {
        world = &world_sim.0;
    } else if running_flag.0 {
        world = &world_actual;
    } else {
        return;
    }

    if mcst_flag.0 {
        for agent in agent_query.iter_mut() {
            match agent.get_status() {
                Status::Working => match agent.get_action() {
                    NpcAction::Steal => {
                        let mut steal_amount = 5;
                        let current_followers = agent.get_followers();
                        if !current_followers.is_empty() {
                            steal_amount = 5 * (current_followers.iter().count() as u32);
                        }
                        send_agent_message(
                            agent.get_id(),
                            agent.get_agent_target_id(),
                            MessageType::Steal(steal_amount),
                            &mut agent_messages,
                        );
                    }
                    NpcAction::TreasureHunt => {
                        let mut steal_amount = 5;
                        let current_followers = agent.get_followers();
                        if !current_followers.is_empty() {
                            steal_amount = 5 * (current_followers.iter().count() as u32);
                        }
                        send_treasure_message(
                            agent.get_id(),
                            agent.get_treasure_target_id(),
                            MessageType::Steal(steal_amount),
                            &mut treasure_messages,
                        );
                    }
                    NpcAction::Talk => {
                        send_agent_message(
                            agent.get_id(),
                            agent.get_agent_target_id(),
                            MessageType::Talk,
                            &mut agent_messages,
                        );
                    }
                    _ => {
                        println!("Error incorrect action for status in simulation::run_action on line {}", line!())
                    }
                },
                Status::Attacking => {
                    let mut attack_amount = 5;
                    let current_followers = agent.get_followers();
                    if !current_followers.is_empty() {
                        attack_amount = 5 * (current_followers.iter().count() as u32);
                    }
                    send_agent_message(
                        agent.get_id(),
                        agent.get_agent_target_id(),
                        MessageType::Attack(attack_amount.try_into().unwrap()),
                        &mut agent_messages,
                    );
                }
                _ => {
                    //Nothing to do
                }
            }
        }
    } else if running_flag.0 {
        for mut agent in agent_query.iter_mut() {
            agent.set_status(Status::Idle);
            let mut rng = thread_rng();
            let reward = rng.gen_range(10..=100);
            agent.add_reward(reward);
        }
    }
}

pub fn check_actual_finish(mut running_flag: ResMut<RunningFlag>) {
    if running_flag.0 {
        running_flag.0 = false;
    }
}

pub fn check_cooperation(
    mcst_flag: ResMut<MCSTFlag>,
    running_flag: ResMut<RunningFlag>,
    mut agent_query: Query<&mut Agent>,
    mut agent_messages: ResMut<AgentMessages>,
) {
    if mcst_flag.0 || running_flag.0 {
        let similar_agents_map = find_agents_with_same_coordinates(&mut agent_query);
        //Find the action each leader is taking, then link with (Follower ID, Leader Action, Agent_Target_ID (For Attack Agent))
        let mut follower_agents: Vec<(u32, NpcAction, u32)> = Vec::new();
        for agent in agent_query.iter_mut() {
            if agent.is_leader() {
                let leader_action = agent.get_action();
                let agent_target_id = agent.get_agent_target_id();

                for &follower_id in agent.get_followers().iter() {
                    follower_agents.push((follower_id, leader_action, agent_target_id));
                }
            }
        }

        //For each agent, if the actions no longer match, begin the process of removing the follower
        for mut agent in agent_query.iter_mut() {
            if !agent.is_follower() {
                continue;
            }

            let agent_id = agent.get_id();
            let agent_action = agent.get_action();
            let agent_target_id = agent.get_agent_target_id();

            for &(follower_id, leader_action, leader_agent_target_id) in &follower_agents {
                if agent_id != follower_id || agent_action != leader_action {
                    if NpcAction::AttackAgent == agent_action {
                        if agent_target_id != leader_agent_target_id {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                agent.set_is_follower(false);
                let message = AgentMessage::new(
                    agent_id,
                    agent.get_leader_id(),
                    MessageType::StopCooperating,
                );
                agent_messages.add_message(message);
            }
        }

        for (agent_id, similar_agents) in similar_agents_map.iter() {
            // Iterate over the agent query to find the current agent
            for agent in agent_query.iter() {
                if *agent_id == agent.get_id() {
                    // Now, iterate over similar agent IDs
                    for similar_agent_id in similar_agents.iter() {
                        // Find the agent that matches similar_agent_id
                        if let Some(similar_agent) =
                            agent_query.iter().find(|a| a.get_id() == *similar_agent_id)
                        {
                            // Compare actions of the current agent and the similar agent
                            if agent.get_action() == similar_agent.get_action() {
                                match agent.get_action() {
                                    NpcAction::AttackAgent => {
                                        if agent.get_agent_target_id()
                                            == similar_agent.get_agent_target_id()
                                        {
                                            let message = AgentMessage::new(
                                                agent.get_id(),
                                                similar_agent.get_id(),
                                                MessageType::CheckStartCooperate,
                                            );
                                            agent_messages.add_message(message);
                                        }
                                    }
                                    NpcAction::Steal => {
                                        if agent.get_agent_target_id()
                                            == similar_agent.get_agent_target_id()
                                        {
                                            let message = AgentMessage::new(
                                                agent.get_id(),
                                                similar_agent.get_id(),
                                                MessageType::CheckStartCooperate,
                                            );
                                            agent_messages.add_message(message);
                                        }
                                    }
                                    NpcAction::Rest => {
                                        continue;
                                    }
                                    NpcAction::None => {
                                        continue;
                                    }
                                    _ => {
                                        let message = AgentMessage::new(
                                            agent.get_id(),
                                            similar_agent.get_id(),
                                            MessageType::CheckStartCooperate,
                                        );
                                        agent_messages.add_message(message);
                                    }
                                };
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}
// pub enum NpcAction {
//     AttackAgent,
//     AttackMonster,
//     Steal,
//     TreasureHunt,
//     Rest,
//     Talk,
//     None,
// }

//NEeds to be renamed simulation desicions ystem

pub fn handle_current_agent_status(
    mcst_flag: ResMut<MCSTFlag>,
    running_flag: ResMut<RunningFlag>,
    mut agent_query: Query<&mut Agent>,
    agent_monster: Query<&mut Monster>,
    world_sim: ResMut<WorldSim>,
    world_actual: ResMut<GameWorld>,
    mut agent_messages: ResMut<AgentMessages>,
) {
    let world;
    if mcst_flag.0 {
        world = &world_sim.0;
    } else if running_flag.0 {
        world = &world_actual;
    } else {
        return;
    }
    for mut agent in agent_query.iter_mut() {
        if agent.is_leader() {
            continue;
        }
        let agent_action = agent.get_action();
        let (x, y) = agent.get_position();
        let agent_position = (x as usize, y as usize);
        if agent.get_status() == Status::Idle {
            match agent_action {
                NpcAction::AttackAgent => {
                    let is_next_to_target = world.is_next_to(
                        agent_position,
                        Target::Agent,
                        agent.get_agent_target_id(),
                    );
                    if is_next_to_target {
                        agent.set_status(Status::Attacking);
                        continue;
                    }
                }
                NpcAction::AttackMonster => {
                    let is_next_to_target = world.is_next_to(
                        agent_position,
                        Target::Monster,
                        agent.get_agent_target_id(),
                    );
                    if is_next_to_target {
                        agent.set_status(Status::Attacking);
                        continue;
                    }
                }
                NpcAction::Steal => {
                    let is_next_to_target = world.is_next_to(
                        agent_position,
                        Target::Agent,
                        agent.get_agent_target_id(),
                    );
                    if is_next_to_target {
                        agent.set_status(Status::Working);
                        continue;
                    }
                }
                NpcAction::TreasureHunt => {
                    let is_next_to_target = world.is_next_to(
                        agent_position,
                        Target::Treasure,
                        agent.get_agent_target_id(),
                    );
                    if is_next_to_target {
                        let (x, y) = world
                            .get_treasure_position(agent.get_treasure_target_id())
                            .unwrap();
                        agent.set_tile_target(Some((x as u32, y as u32)));
                        agent.set_status(Status::Working);
                        continue;
                    }
                }
                NpcAction::Rest => {
                    let closest_village =
                        world.find_closest_tiletype(agent_position, crate::gameworld::tile_types::TileType::Village);
                    match closest_village {
                        Some((v1, v2)) => {
                            if agent.get_position() == (v1 as u32, v2 as u32) {
                                agent.set_status(Status::Recovering);
                                agent.set_tile_target(Some((v1 as u32, v2 as u32)));
                                continue;
                            }
                        }
                        None => {
                            println!(
                                "Tile village not found in system::handle_current_agent_status line {} from agent {}",
                                line!(),
                                agent.get_id()
                            );
                        }
                    }
                }
                NpcAction::Talk => {
                    let is_next_to_target = world.is_next_to(
                        agent_position,
                        Target::Agent,
                        agent.get_agent_target_id(),
                    );
                    if is_next_to_target {
                        agent.set_status(Status::Working);
                        continue;
                    }
                }
                NpcAction::None => {}
            }

            pathfind(&mut agent, world, &mut agent_messages);
            agent.set_status(Status::Moving);
        //  Functionality to move or take an action. If wants to
        // change to move and take an action this can go into message handling
        } else if agent.get_status() == Status::Moving {
            if agent_action == NpcAction::AttackAgent
                || agent_action == NpcAction::Steal
                || agent_action == NpcAction::Talk
            {
                let is_next_to_target =
                    world.is_next_to(agent_position, Target::Agent, agent.get_agent_target_id());
                if is_next_to_target {
                    match agent_action {
                        NpcAction::AttackAgent => agent.set_status(Status::Attacking),
                        _ => agent.set_status(Status::Working),
                    }
                    continue;
                }
            } else if agent_action == NpcAction::AttackMonster {
                let is_next_to_target =
                    world.is_next_to(agent_position, Target::Monster, agent.get_agent_target_id());
                if is_next_to_target {
                    agent.set_status(Status::Attacking);
                    continue;
                }
            } else if agent_action == NpcAction::TreasureHunt {
                let is_next_to_target = world.is_next_to(
                    agent_position,
                    Target::Treasure,
                    agent.get_agent_target_id(),
                );
                if is_next_to_target {
                    agent.set_status(Status::Working);
                    continue;
                }
            } else if agent_action == NpcAction::Talk {
                let is_next_to_target = world.is_next_to(
                    agent_position,
                    Target::Treasure,
                    agent.get_agent_target_id(),
                );
                if is_next_to_target {
                    agent.set_status(Status::Working);
                    continue;
                }
            } else if agent_action == NpcAction::Rest {
                let closest_village =
                    world.find_closest_tiletype(agent_position, crate::gameworld::tile_types::TileType::Village);
                match closest_village {
                    Some((v1, v2)) => {
                        if agent.get_position() == (v1 as u32, v2 as u32) {
                            agent.set_status(Status::Recovering);
                            continue;
                        }
                    }
                    None => {
                        println!(
                        "Tile village not found in system::handle_current_agent_status line {} from agent {}",
                        line!(),
                        agent.get_id()
                    );
                    }
                }
            }

            pathfind(&mut agent, world, &mut agent_messages);
        } else if agent.get_status() == Status::Working {
            match agent_action {
                NpcAction::Steal => {
                    //Should be handled in messages
                }
                NpcAction::TreasureHunt => {
                    //Should be handled in messages
                }
                NpcAction::Talk => {
                    //Should be handled in messages
                }
                _ => {}
            }
        } else if agent.get_status() == Status::Retaliating {
            let is_next_to_target =
                world.is_next_to(agent_position, Target::Agent, agent.get_agent_target_id());
            if is_next_to_target {
                //Check if dead
                //Continues to retaliate
                //Flight of fight

                let fight_or_flight;
                if agent.get_retaliation_target() == Target::Agent {
                    fight_or_flight = agent.flight_or_fight(agent.get_agent_target_id(), false);
                } else {
                    fight_or_flight = agent.flight_or_fight(agent.get_agent_target_id(), true);
                }
                if fight_or_flight == Status::Fleeing {
                    let closest_village =
                        world.find_closest_tiletype(agent_position, crate::gameworld::tile_types::TileType::Village);
                    match closest_village {
                        Some((v1, v2)) => {
                            agent.set_status(Status::Fleeing);
                            agent.set_tile_target(Some((v1 as u32, v2 as u32)));
                            continue;
                        }
                        None => {
                            println!(
                                "Tile village not found in system::handle_current_agent_status line {} from agent {}",
                                line!(),
                                agent.get_id()
                            );
                        }
                    }
                }
            } else {
                match agent_action {
                    NpcAction::AttackAgent => {
                        let is_next_to_target = world.is_next_to(
                            agent_position,
                            Target::Agent,
                            agent.get_agent_target_id(),
                        );
                        if is_next_to_target {
                            agent.set_status(Status::Attacking);
                            continue;
                        }
                    }
                    NpcAction::AttackMonster => {
                        let is_next_to_target = world.is_next_to(
                            agent_position,
                            Target::Monster,
                            agent.get_agent_target_id(),
                        );
                        if is_next_to_target {
                            agent.set_status(Status::Attacking);
                            continue;
                        }
                    }
                    NpcAction::Steal => {
                        let is_next_to_target = world.is_next_to(
                            agent_position,
                            Target::Agent,
                            agent.get_agent_target_id(),
                        );
                        if is_next_to_target {
                            agent.set_status(Status::Working);
                            continue;
                        }
                    }
                    NpcAction::TreasureHunt => {
                        let is_next_to_target = world.is_next_to(
                            agent_position,
                            Target::Treasure,
                            agent.get_agent_target_id(),
                        );
                        if is_next_to_target {
                            let (x, y) = world
                                .get_treasure_position(agent.get_treasure_target_id())
                                .unwrap();
                            agent.set_tile_target(Some((x as u32, y as u32)));
                            agent.set_status(Status::Working);
                            continue;
                        }
                    }
                    NpcAction::Rest => {
                        let closest_village = world
                            .find_closest_tiletype(agent_position, crate::gameworld::tile_types::TileType::Village);
                        match closest_village {
                            Some((v1, v2)) => {
                                if agent.get_position() == (v1 as u32, v2 as u32) {
                                    agent.set_status(Status::Recovering);
                                    agent.set_tile_target(Some((v1 as u32, v2 as u32)));
                                    continue;
                                }
                            }
                            None => {
                                println!(
                                    "Tile village not found in system::handle_current_agent_status line {} from agent {}",
                                    line!(),
                                    agent.get_id()
                                );
                            }
                        }
                    }
                    NpcAction::Talk => {
                        let is_next_to_target = world.is_next_to(
                            agent_position,
                            Target::Agent,
                            agent.get_agent_target_id(),
                        );
                        if is_next_to_target {
                            agent.set_status(Status::Working);
                            continue;
                        }
                    }
                    NpcAction::None => {}
                }
            }
        } else if agent.get_status() == Status::Fleeing {
        } else if agent.get_status() == Status::Recovering {
        }

        // else if agent.get_status() == Status::Retaliating {
        //     let (x, y);
        //     if agent.get_retaliation_target() == Target::Agent {
        //         (x, y) = world.get_agent_position(agent.get_agent_target_id());
        //     } else if agent.get_retaliation_target() == Target::Monster {
        //         (x, y) = world.get_monster_position(agent.get_monster_target_id());
        //     }
        //     let target_position_unwrapped = (x as u32, y as u32);
        //     match target_position {
        //         Ok(target_position_unwrapped) => {
        //             if target_position_unwrapped == (agent.get_position()) {
        //                 if agent.get_retaliation_target() == Target::Agent {
        //                     send_agent_message(
        //                         agent.get_id(),
        //                         agent.get_agent_target_id(),
        //                         MessageType::Attack(5),
        //                         &mut agent_messages,
        //                     );
        //                 } else if agent.get_retaliation_target() == Target::Monster {
        //                     set_current_target(&mut agent, world);
        //                     let target = agent.get_tile_target().unwrap();
        //                     if agent.get_position() == target {
        //                         send_monster_message(
        //                             agent.get_id(),
        //                             agent.get_monster_target_id(),
        //                             MessageType::Attack(5),
        //                             &mut monster_messages,
        //                         );
        //                         agent.set_status(Status::Attacking);
        //                         continue;
        //                     }
        //                 }
        //             }
        //         }
        //         Err(_) => {
        //             println!(
        //                 "Position not found in system::handle_current_agent_status line {} from agent {}",
        //                 line!(),
        //                 agent.get_id()
        //             );
        //         }
        //     }
        //Get status of attacker
        //If agent or monster
        //If anything but attacking or retaliating, reset action
        //If retaliating
        //Check if agent is in same space
        //If so mark error
        //Otherwise attack
        // }
    }
}

//Might be situation with pop() vs remove(0)
fn pathfind(agent: &mut Agent, world: &GameWorld, mut agent_messages: &mut AgentMessages) {
    let (x1, y1) = agent.get_position();
    let current_pos_i32 = (x1 as i32, y1 as i32);
    let (x2, y2) = agent.get_tile_target().unwrap();
    let tile_target_i32 = (x2 as i32, y2 as i32);
    let mut path_option = agent.get_path();
    let tile_grid = world.get_grid();

    if path_option.is_none() {
        path_option = find_path(tile_grid.clone(), current_pos_i32, tile_target_i32);
    }
    let mut path = path_option.unwrap();

    if let Some(last_position) = path.last().cloned() {
        if last_position == tile_target_i32 {
            let (first, second) = path.remove(0);
            send_agent_message(
                agent.get_id(),
                agent.get_id(),
                MessageType::Move((first as u32, second as u32)),
                &mut agent_messages,
            );
        } else {
            path_option = find_path(tile_grid, current_pos_i32, tile_target_i32);
            path = path_option.unwrap();

            if let Some(last_position) = path.last().cloned() {
                if last_position == tile_target_i32 {
                    let (first, second) = path.remove(0);
                    send_agent_message(
                        agent.get_id(),
                        agent.get_id(),
                        MessageType::Move((first as u32, second as u32)),
                        &mut agent_messages,
                    );
                }
            }
        }
    }
}

fn set_current_target(agent: &mut Mut<Agent>, world: &GameWorld) {
    let current_target = agent.get_target();
    match current_target {
        Target::Agent => match world.get_agent_position(agent.get_agent_target_id()) {
            Ok(agent_position) => {
                let (x, y) = agent_position;
                agent.set_tile_target(Some((x as u32, y as u32)));
            }
            Err(MyError::AgentNotFound) => {
                println!(
                    "Agent not found in system::perform_action line {} from agent {}",
                    line!(),
                    agent.get_id()
                );
            }
            _ => {}
        },
        Target::Monster => match world.get_monster_position(agent.get_monster_target_id()) {
            Ok(monster_position) => {
                let (x, y) = monster_position;
                agent.set_tile_target(Some((x as u32, y as u32)));
            }
            Err(MyError::MonsterNotFound) => {
                println!(
                    "Monster not found in system::perform_action line {} from agent {}",
                    line!(),
                    agent.get_id()
                );
            }
            _ => {}
        },
        Target::Treasure => match world.get_treasure_position(agent.get_treasure_target_id()) {
            Ok(treasure_position) => {
                let (x, y) = treasure_position;
                agent.set_tile_target(Some((x as u32, y as u32)));
            }
            Err(MyError::TreasureNotFound) => {
                println!("Treasure not found.");
            }
            _ => {}
        },
        Target::None => {
            //Waiting to be assigned an action
            //println!("Invalid Target in simulations::set_current_target line {} from agent {}",line!(), agent.get_id());
        }
        Target::Tile => {} //Should already be handled
    }
}

fn find_agents_with_same_coordinates(query: &mut Query<&mut Agent>) -> HashMap<u32, Vec<u32>> {
    let mut result = HashMap::new();

    // Collect positions of all agents first, excluding followers
    let agent_positions: HashMap<u32, (u32, u32)> = query
        .iter_mut()
        .filter(|agent| !agent.is_follower())
        .map(|agent| (agent.get_id(), agent.get_position()))
        .collect();

    // Iterate over the agent positions to find matching coordinates
    for (agent_id, (x, y)) in &agent_positions {
        let mut similar_agents = Vec::new();

        for (other_id, (other_x, other_y)) in &agent_positions {
            if agent_id != other_id && x == other_x && y == other_y {
                if let Some(agent) = query
                    .iter_mut()
                    .find(|agent| agent.get_id() == *other_id && !agent.is_follower())
                {
                    similar_agents.push(agent.get_id());
                }
            }
        }

        result.insert(*agent_id, similar_agents);
    }

    result
}

fn is_finished(flags: &mut Vec<bool>) -> bool {
    for &flag in flags.iter() {
        if !flag {
            return false;
        }
    }
    true
}
