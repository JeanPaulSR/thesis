use std::collections::HashMap;

//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity for now
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::entities::agent::Agent;
use crate::entities::agent::{Status, Target};
use crate::errors::MyError;
use crate::mcst_system::mcst::NpcAction;
use crate::{
    AgentMessages, Backpropogate, FinishedSelectingActions, GameWorld, MCSTCurrent, MCSTFlag,
    MonsterMessages, NpcActions, RunningFlag, ScoreTracker, TreasureMessages, WorldSim,
};

use super::mcst::SimulationTree;
use super::systems::{send_agent_message, MessageType};

pub fn set_simulation_actions(
    mut simulation_tree: ResMut<SimulationTree>,
    mut world_sim: ResMut<WorldSim>,
    mut mcst_flag: ResMut<MCSTFlag>,
    mut running_flag: ResMut<RunningFlag>,
    mut finished_selecting: ResMut<FinishedSelectingActions>,
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

        let world = &world_sim.0;
        let closest_agent_vec = world.find_closest_agents();
        let closest_monster_vec = world.find_closest_monsters();
        let closest_treasure_vec = world.find_closest_treasures();
        let closest_village_vec = world.find_closest_villages();

        let mut agent_ids = Vec::new();

        for agent in agent_query.iter_mut() {
            agent_ids.push(agent.get_id());
        }

        for mut agent in agent_query.iter_mut() {
            for (action_id, action) in npc_actions.iter_mut() {
                if *action_id == agent.get_id() {
                    for (score_id, score) in score_tracker.iter_mut() {
                        if *score_id == agent.get_id() {
                            if !(*score < 0) {
                                if agent.get_status() == Status::Idle {
                                    let current_action = action.pop_front().unwrap();
                                    if action.is_empty() || current_action == NpcAction::None {
                                        *score = 0 - (agent.get_reward() as i32);
                                        action.push_front(current_action);
                                    } else {
                                        match current_action {
                                            NpcAction::AttackAgent => {
                                                // let target_id = agent.calculate_best_agent(NpcAction::AttackAgent, &agent_ids);
                                                // agent.set_agent_target_id(target_id);
                                            }
                                            NpcAction::AttackMonster => {
                                                // if let Some(&(_agent_id, target_id)) = closest_monster_vec.iter().find(|&&(id, _)| id == agent.get_id()) {
                                                //     agent.set_monster_target_id(target_id);
                                                // }
                                            }
                                            NpcAction::Steal => {
                                                // let target_id = agent.calculate_best_agent(NpcAction::Steal, &agent_ids);
                                                // agent.set_agent_target_id(target_id);
                                            }
                                            NpcAction::TreasureHunt => {
                                                // if let Some(&(_agent_id, target_id)) = closest_treasure_vec.iter().find(|&&(id, _)| id == agent.get_id()) {
                                                //     agent.set_treasure_target_id(target_id);
                                                // }
                                            }
                                            NpcAction::Talk => {
                                                // let target_id = agent.calculate_best_agent(NpcAction::Talk, &agent_ids);
                                                // agent.set_agent_target_id(target_id);
                                            }
                                            NpcAction::Rest => {
                                                // if let Some(&(_agent_id, target_id)) = closest_village_vec.iter().find(|&&(id, _)| id == agent.get_id()) {
                                                //     agent.set_tile_target(Some(target_id));
                                                // }
                                            }
                                            _ => {}
                                        }
                                        agent.set_action(current_action);
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
        if finished {
            finished_selecting.0 = true;
        }
    }
}

pub fn run_actual(
    mut running_flag: ResMut<RunningFlag>,
    mut agent_query: Query<&mut Agent>,
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
) {
    if running_flag.0 {
        for mut agent in agent_query.iter_mut() {
            agent.set_status(Status::Idle);
            let mut rng = thread_rng();
            let reward = rng.gen_range(10..=100);
            agent.add_reward(reward);
        }
        running_flag.0 = false;
    }
}

pub fn run_simulation(
    mcst_flag: ResMut<MCSTFlag>,
    mut agent_query: Query<&mut Agent>,
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
) {
    if mcst_flag.0 {
        let similar_agents_map = find_agents_with_same_coordinates(&mut agent_query);

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
        for mut agent in agent_query.iter_mut() {
            if agent.is_follower() {
                let agent_id = agent.get_id();
                let agent_action = agent.get_action();
                let agent_target_id = agent.get_agent_target_id();

                for &(follower_id, leader_action, leader_agent_target_id) in &follower_agents {
                    if agent_id == follower_id {
                        if agent_action == leader_action {
                            //Group this if else into a more succient conditional
                            if let NpcAction::AttackAgent = agent_action {
                                if agent_target_id == leader_agent_target_id {
                                    //Set to no longer follower
                                    //Message leader no longer follower
                                }
                            } else {
                                //Set to no longer follower
                                //Message leader no longer follower
                            }
                        }
                    }
                }
            }
        }
        //Movement
        for mut agent in agent_query.iter_mut() {
            if agent.get_status() == Status::Idle {
                //Check if the agent is already moving, or if it is currently fighting
                if agent.is_leader() {
                    let agent_action = agent.get_action();
                    match agent_action {
                        NpcAction::AttackAgent => {
                            // let target = agent.get_tile_target().unwrap();
                            // if agent.get_position() == target{
                            //     send_agent_message(agent.get_id(), agent.get_agent_target_id(), MessageType::Attack(5), &mut agent_messages)
                            // } else {
                            //     if let Some(mut path) = agent.get_path() {
                            //         if let Some(tile_target) = agent.get_tile_target() {
                            //             // Convert the tile target from (u32, u32) to (i32, i32)
                            //             let tile_target_i32 = (tile_target.0 as i32, tile_target.1 as i32);

                            //             // Check if the last position in the path equals the tile target
                            //             if let Some(&last_position) = path.last() {
                            //                 if last_position == tile_target_i32 {calculate_best_agent
                            //                     // Send the move message to the next node in the path
                            //                     if let Some((first, second)) = path.pop() {
                            //                         send_agent_message(
                            //                             agent.get_id(),
                            //                             agent.get_agent_target_id(),
                            //                             MessageType::Move((first as u32, second as u32)),
                            //                             &mut agent_messages,
                            //                         );
                            //                     }
                            //                 } else {
                            //                     // If the target does not match, recalculate the path
                            //                     if let Ok(agent_position) = world.get_agent_position(agent.get_id()) {
                            //                         let start_pos = (agent_position.0 as i32, agent_position.1 as i32);

                            //                         // Function to extract tiles from the grid
                            //                         fn extract_tiles(grid: &Vec<Vec<Arc<Mutex<Tile>>>>) -> Vec<Vec<Tile>> {
                            //                             grid.iter()
                            //                                 .map(|row| row.iter().map(|tile| tile.lock().unwrap().clone()).collect())
                            //                                 .collect()
                            //                         }

                            //                         // Extract tiles and find a new path
                            //                         let tile_grid = extract_tiles(&world.grid);
                            //                         if let Some(new_path) = find_path(tile_grid, start_pos, tile_target_i32) {
                            //                             agent.set_path(new_path);
                            //                         } else {
                            //                             // Handle the case where no path is found
                            //                             eprintln!("Failed to find a path for agent {}", agent.get_id());
                            //                             // You might want to set an empty path, log an error, or take other action here
                            //                             agent.set_path(Vec::new());
                            //                         }
                            //                     }
                            //                 }
                            //             }
                            //         }
                            //     }
                            // }
                            //Check if at position
                            //If so, send attack message
                            //Else, move to position
                        }
                        NpcAction::AttackMonster => {
                            //Check if at position
                            //If so, send attack message
                            //Else, move to position
                        }
                        NpcAction::Steal => {
                            //Check if at position
                            //If so, send attack message
                            //Else, move to position
                        }
                        NpcAction::TreasureHunt => {
                            //Check if at position
                            //If so, send attack message
                            //Else, move to position
                        }
                        NpcAction::Rest => todo!(),
                        NpcAction::Talk => todo!(),
                        NpcAction::None => todo!(),
                    }

                    //Debug for assigning all agents
                    agent.set_status(Status::Idle);
                    let mut rng = thread_rng();
                    let reward = rng.gen_range(10..=100);
                    agent.add_reward(reward);
                }
            }
        }
    }

    pub fn update_targets(agent: &mut Agent, world: &GameWorld) -> Result<(), MyError> {
        match agent.get_target() {
            Target::Agent => {
                let target_id = agent.get_agent_target_id();
                match world.get_agent_position(target_id) {
                    Ok(position) => {
                        agent.set_tile_target(Some((position.0 as u32, position.1 as u32)));
                    }
                    Err(err) => return Err(err),
                }
            }
            Target::Monster => {
                let target_id = agent.get_monster_target_id();
                match world.get_monster_position(target_id) {
                    Ok(position) => {
                        agent.set_tile_target(Some((position.0 as u32, position.1 as u32)));
                    }
                    Err(err) => return Err(err),
                }
            }
            Target::Treasure => {
                let target_id = agent.get_treasure_target_id();
                match world.get_treasure_position(target_id) {
                    Ok(position) => {
                        agent.set_tile_target(Some((position.0 as u32, position.1 as u32)));
                    }
                    Err(err) => return Err(err),
                }
            }
            Target::Tile | Target::None => {
                // If the target is None or Tile, clear the tile_target
                agent.set_tile_target(None);
            }
        }
        Ok(())
    }

    // If the agent is not at the target position, initiate travel
    //            match agent.travel(world.get_grid(), &mut commands) {
    //                Ok(_) => {
    //                    if agent.is_leader() && agent.get_followers().len() > 0{
    //                        for agent_id in agent.get_followers(){
    //                            send_agent_message(
    //                                agent.get_id(),
    //                                agent_id,
    //                                MessageType::Move(agent.get_position()),
    //                                &mut agent_messages,
    //                            );
    //                        }
    //                    }
    //                },
    //                Err(_) => println!("Invalid Target in system::perform_action line {}",573),
    //            };
    //            agent.set_status(Status::Moving);
    //            // Call the move_between_tiles function to move the agent to the next position in the path

    //            match world.move_agent(agent.get_id(), x as usize, y as usize){
    //                Ok(it) => it,
    //                Err(_) => println!("Invalid Move"),
    //            }

    // for mut agent in query.iter_mut() {
    //    if !(agent.is_leader() && !agent.get_followers().is_empty()) {
    //        let current_target = agent.get_target();
    //        match current_target {
    //            Target::Agent => {
    //                match world.get_agent_position(agent.get_agent_target_id()) {
    //                    Ok(agent_position) => {
    //                        let (x, y) = agent_position;
    //                        agent.set_tile_target(Some((x as u32, y as u32)));
    //                    }
    //                    Err(MyError::AgentNotFound) => {
    //                        println!("Agent not found in system::perform_action line {} from agent {}",284, agent.get_id());
    //                    }
    //                    _ => {} // Handle other errors if needed
    //                }
    //            }
    //            Target::Monster => {
    //                match world.get_monster_position(agent.get_monster_target_id()) {
    //                    Ok(monster_position) => {
    //                        let (x, y) = monster_position;
    //                        agent.set_tile_target(Some((x as u32, y as u32)));
    //                    }
    //                    Err(MyError::MonsterNotFound) => {
    //                        println!("Monster not found in system::perform_action line {}",296);
    //                    }
    //                    _ => {} // Handle other errors if needed
    //                }
    //            }
    //            Target::Treasure => {
    //                match world.get_treasure_position(agent.get_treasure_target_id()) {
    //                    Ok(treasure_position) => {
    //                        let (x, y) = treasure_position;
    //                        agent.set_tile_target(Some((x as u32, y as u32)));
    //                    }
    //                    Err(MyError::TreasureNotFound) => {
    //                        println!("Treasure not found.");
    //                    }
    //                    _ => {} // Handle other errors if needed
    //                }
    //            }
    //            Target::None => {
    //                println!("Invalid Target in system::perform_action line {} from agent {}",296, agent.get_id());
    //            }
    //            Target::Tile => {
    //                todo!()
    //            }
    //        }

    //        // Check if the agent's current position is equal to the tile target
    //        let (x, y) = agent.get_position();
    //        if (x, y) == agent.get_tile_target().unwrap_or_default() {
    //        //     // Continue with action logic
    //            let action = agent.get_action();
    //            //Match the type of action
    //            match action {
    //                NpcAction::Attack => {
    //                    //Match the current target for the Attack action
    //                    match current_target{
    //                        //For the target Agent of the Attack action
    //                        Target::Agent => {
    //                            let id = agent.get_agent_target_id();
    //                            let message = MessageType::Attack(10);
    //                            follower_actions(
    //                                agent.clone(),
    //                                message,
    //                                id,
    //                                &mut agent_messages,
    //                            );
    //                            send_agent_message(
    //                                agent.get_id(),
    //                                id,
    //                                MessageType::Attack(10),
    //                                &mut agent_messages,
    //                            );
    //                            agent.remove_energy(5);
    //                            agent.set_status(Status::Working);
    //                        },
    //                        Target::Monster => {
    //                            let id = agent.get_monster_target_id();
    //                            send_monster_message(
    //                                agent.get_id(),
    //                                id,
    //                                MessageType::Attack(10),
    //                                &mut monster_messages,
    //                            );
    //                            agent.remove_energy(5);
    //                            if agent.is_leader() && agent.get_followers().len() > 0{
    //                                for agent_id in agent.get_followers(){
    //                                    send_agent_message(
    //                                        agent.get_id(),
    //                                        id,
    //                                        MessageType::Energy(false, 5),
    //                                        &mut agent_messages,
    //                                    );
    //                                }
    //                            }
    //                            agent.set_status(Status::Working);
    //                        },
    //                        _ => println!("Invalid Target in system::perform_action line {}",506),
    //                    }
    //                }
    //                NpcAction::Steal => {
    //                    //Match the current target for the Attack action
    //                    match current_target{
    //                        //For the target Agent of the Attack action
    //                        Target::Agent => {

    //                            let id = agent.get_agent_target_id();
    //                            send_agent_message(
    //                                agent.get_id(),
    //                                id,
    //                                MessageType::Steal(10),
    //                                &mut agent_messages,
    //                            );
    //                            agent.remove_energy(20);
    //                        },
    //                        Target::Treasure => {
    //                            let id = agent.get_agent_target_id();
    //                            send_treasure_message(
    //                                agent.get_id(),
    //                                id,
    //                                MessageType::Steal(10),
    //                                &mut treasure_messages,
    //                            );
    //                            let _treasure = world.remove_treasure(id);
    //                            agent.remove_energy(5);
    //                        },
    //                        _ => println!("Invalid Target in system::perform_action line {}",536),
    //                    }
    //                }
    //                NpcAction::Rest => {
    //                    //Match the current target for the rest action
    //                    match current_target{
    //                        Target::Tile => {
    //                            agent.add_energy(10);
    //                            if agent.get_energy() == agent.get_max_energy() {
    //                                agent.set_status(Status::Idle);
    //                            }
    //                        },
    //                        _ => println!("Invalid Target in system::perform_action line {}",548)
    //                    }
    //                }
    //                NpcAction::Talk => {
    //                    // Logic for moving to a monster
    //                }
    //                NpcAction::None => {
    //                    // Logic for moving to a monster
    //                }
    //            }
    //        } else {

    //            // If the agent is not at the target position, initiate travel
    //            match agent.travel(world.get_grid(), &mut commands) {
    //                Ok(_) => {
    //                    if agent.is_leader() && agent.get_followers().len() > 0{
    //                        for agent_id in agent.get_followers(){
    //                            send_agent_message(
    //                                agent.get_id(),
    //                                agent_id,
    //                                MessageType::Move(agent.get_position()),
    //                                &mut agent_messages,
    //                            );
    //                        }
    //                    }
    //                },
    //                Err(_) => println!("Invalid Target in system::perform_action line {}",573),
    //            };
    //            agent.set_status(Status::Moving);
    //            // Call the move_between_tiles function to move the agent to the next position in the path

    //            match world.move_agent(agent.get_id(), x as usize, y as usize){
    //                Ok(it) => it,
    //                Err(_) => println!("Invalid Move"),
    //            }
    //        }
    //    }
    // }
}

fn follower_actions(
    agent: Agent,
    message: MessageType,
    target_id: u32,
    agent_messages: &mut AgentMessages,
) {
    if agent.is_leader() && agent.get_followers().len() > 0 {
        for agent_id in agent.get_followers() {
            send_agent_message(agent_id, target_id, message.copy(), agent_messages);
            send_agent_message(
                agent_id,
                agent_id,
                match message {
                    MessageType::Attack(_) => MessageType::Energy(false, 5),
                    MessageType::MonsterAttack(_) => todo!(),
                    MessageType::Reward(_) => todo!(),
                    MessageType::Steal(_) => todo!(),
                    MessageType::Cooperate(_) => todo!(),
                    MessageType::StopCooperating => todo!(),
                    MessageType::Move(_) => todo!(),
                    MessageType::Energy(_, _) => todo!(),
                    MessageType::GroupDamage(_) => todo!(),
                    MessageType::GroupReward(_) => todo!(),
                    MessageType::Inherit(_, _) => todo!(),
                    MessageType::BecomeFollower => todo!(),
                },
                agent_messages,
            );
        }
    }
}

fn set_current_target(agent: &mut Mut<Agent>, world: &mut GameWorld) {
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
        Target::Monster => {
            match world.get_monster_position(agent.get_monster_target_id()) {
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
                _ => {} // Handle other errors if needed
            }
        }
        Target::Treasure => {
            match world.get_treasure_position(agent.get_treasure_target_id()) {
                Ok(treasure_position) => {
                    let (x, y) = treasure_position;
                    agent.set_tile_target(Some((x as u32, y as u32)));
                }
                Err(MyError::TreasureNotFound) => {
                    println!("Treasure not found.");
                }
                _ => {} // Handle other errors if needed
            }
        }
        Target::None => {
            //println!("Invalid Target in system::perform_action line {} from agent {}",296, agent.get_id());
        }
        Target::Tile => {}
    }
}

fn find_agents_with_same_coordinates(query: &mut Query<&mut Agent>) -> HashMap<u32, Vec<u32>> {
    let mut result = HashMap::new();

    // Collect positions of all agents first
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

/*

Create a list of flags that mark if it has reached the end
Check if it is currently
If not

*/
// pub fn simulation(
//     _world: &mut ResMut<GameWorld>,
//     _agent_messages: &mut ResMut<AgentMessages>,
//     _monster_messages: &mut ResMut<MonsterMessages>,
//     _treasure_messages: &mut ResMut<TreasureMessages>,
//     npc_actions: &mut Vec<(u32, Vec<NpcAction>)>,
//     query: &mut Query<&mut Agent>,
//     _commands: &mut Commands,
// ) {
//     println!("NEW LOOP");
//     //let npc_actions_clone = npc_actions.clone();
//     //Set to false if not finished yet
//     let mut finished_flags: Vec<bool> = npc_actions.iter().map(|_| false).collect();

//     while !is_finished(&mut finished_flags) {
//         //println!("NEW ITERATION");
//         for index in 0..npc_actions.len() {
//             let (action_id, actions) = &mut npc_actions[index];
//             for mut agent in query.iter_mut() {
//                 if action_id == &mut agent.get_id(){
//                     if agent.get_status() == agent::Status::Idle{
//                         if let Some(action) = actions.pop() {
//                             println!("Reached here");
//                             agent.set_action(action);
//                             if actions.is_empty(){
//                                 if let Some(flag) = finished_flags.get_mut(index) {
//                                     *flag = true;
//                                 } else {
//                                     panic!("Index out of bounds.");
//                                 }
//                             }
//                         } else {
//                             //agent.set_random_action();
//                         }
//                         //agent.calculate_target();
//                     }
//                 }

//             }

//         }
//         //perform action
//         //systems::agent_message_system(agent_messages, monster_messages, world, query, commands);
//         //messages
//     }
// }

fn is_finished(flags: &mut Vec<bool>) -> bool {
    for &flag in flags.iter() {
        if !flag {
            return false;
        }
    }
    true
}
