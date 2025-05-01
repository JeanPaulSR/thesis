use std::collections::{HashMap, HashSet};
use std::fmt;
use crate::npcs::npc_components::gene_type::GeneType;
use crate::npcs::npc_components::npc_status::Status;
use crate::npcs::agent::Agent;
use crate::npcs::npc_components::target::Target;
use crate::npcs::monster::Monster;
use crate::{MCSTFlag, RunningFlag, WorldSim};
use bevy::prelude::*;
use rand::Rng;

use crate::gameworld::world::GameWorld;

use super::mcst::NpcAction;

#[derive(Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MessageType {
    Attack(u8),
    MonsterAttack(u8),
    GroupDamage(u8),
    GroupReward(u32),
    Reward(u32),
    Steal(u32),
    Cooperate(Vec<u32>),
    CheckStartCooperate,
    BecomeFollower,
    SetLeader(u32),
    StopCooperating,
    Move((u32, u32)),
    Rest,
    Talk,
    //True is add, false is remove
    Energy(bool, u8),
    Inherit(Vec<u32>, u32),
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::Attack(value) => write!(f, "Attack({})", value),
            MessageType::MonsterAttack(value) => write!(f, "MonsterAttack({})", value),
            MessageType::GroupDamage(value) => write!(f, "GroupDamage({})", value),
            MessageType::GroupReward(value) => write!(f, "GroupReward({})", value),
            MessageType::Reward(value) => write!(f, "Reward({})", value),
            MessageType::Steal(value) => write!(f, "Steal({})", value),
            MessageType::Cooperate(vec) => write!(f, "Cooperate({:?})", vec),
            MessageType::CheckStartCooperate => write!(f, "CheckStartCooperate"),
            MessageType::BecomeFollower => write!(f, "BecomeFollower"),
            MessageType::SetLeader(value) => write!(f, "SetLeader({})", value),
            MessageType::StopCooperating => write!(f, "StopCooperating"),
            MessageType::Move((x, y)) => write!(f, "Move({}, {})", x, y),
            MessageType::Rest => write!(f, "Rest"),
            MessageType::Talk => write!(f, "Talk"),
            MessageType::Energy(add, value) => write!(f, "Energy({}, {})", add, value),
            MessageType::Inherit(vec, value) => write!(f, "Inherit({:?}, {})", vec, value),
        }
    }
}

impl PartialOrd for MessageType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MessageType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Define the order of the variants
        use MessageType::*;

        let self_order = match self {
            Attack(_) => 0,
            MonsterAttack(_) => 1,
            GroupDamage(_) => 2,
            GroupReward(_) => 3,
            Reward(_) => 4,
            Steal(_) => 5,
            Cooperate(_) => 6,
            CheckStartCooperate => 7,
            BecomeFollower => 8,
            SetLeader(_) => 9,
            StopCooperating => 10,
            Move(_) => 11,
            Energy(_, _) => 12,
            Inherit(_, _) => 13,
            Rest => 14,
            Talk => 15,
        };

        let other_order = match other {
            Attack(_) => 0,
            MonsterAttack(_) => 1,
            GroupDamage(_) => 2,
            GroupReward(_) => 3,
            Reward(_) => 4,
            Steal(_) => 5,
            Cooperate(_) => 6,
            CheckStartCooperate => 7,
            BecomeFollower => 8,
            SetLeader(_) => 9,
            StopCooperating => 10,
            Move(_) => 11,
            Energy(_, _) => 12,
            Inherit(_, _) => 13,
            Rest => 14,
            Talk => 15,
        };

        self_order.cmp(&other_order)
    }
}

impl MessageType {
    // Manually implement the copy method
    pub fn copy(&self) -> Self {
        match self {
            MessageType::Attack(val) => MessageType::Attack(*val),
            MessageType::GroupDamage(val) => MessageType::GroupDamage(*val),
            MessageType::MonsterAttack(val) => MessageType::MonsterAttack(*val),
            MessageType::Reward(val) => MessageType::Reward(*val),
            MessageType::Steal(val) => MessageType::Steal(*val),
            MessageType::Cooperate(vec) => MessageType::Cooperate(vec.clone()),
            MessageType::CheckStartCooperate => MessageType::CheckStartCooperate,
            MessageType::StopCooperating => MessageType::StopCooperating,
            MessageType::BecomeFollower => MessageType::BecomeFollower,
            MessageType::SetLeader(val) => MessageType::SetLeader(*val),
            MessageType::Move((val1, val2)) => MessageType::Move((*val1, *val2)),
            MessageType::Energy(flag, amount) => MessageType::Energy(*flag, *amount),
            MessageType::GroupReward(reward) => MessageType::GroupReward(*reward),
            MessageType::Inherit(agents, reward) => MessageType::Inherit(agents.clone(), *reward),
            MessageType::Rest => MessageType::Rest,
            MessageType::Talk => MessageType::Talk,
        }
    }
}
//     _
//     /\                   | |
//    /  \   __ _  ___ _ __ | |_
//   / /\ \ / _` |/ _ \ '_ \| __|
//  / ____ \ (_| |  __/ | | | |_
// /_/    \_\__, |\___|_| |_|\__|
//           __/ |
//          |___/

#[allow(dead_code)]
#[derive(Clone)]
pub struct AgentMessage {
    sender_id: u32,
    receiver_id: u32,
    message_type: MessageType,
}

impl AgentMessage {
    pub fn new(sender_id: u32, receiver_id: u32, message_type: MessageType) -> Self {
        AgentMessage {
            sender_id: sender_id,
            receiver_id: receiver_id,
            message_type: message_type,
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            message_type: self.message_type.copy(),
        }
    }
}

#[derive(Resource)]
pub struct AgentMessages {
    pub(crate) messages: Vec<AgentMessage>,
}

impl AgentMessages {
    pub fn new() -> Self {
        AgentMessages {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: AgentMessage) {
        self.messages.push(message);
    }

    pub fn is_empty(&self) -> bool {
        return self.messages.is_empty();
    }
}

//Missing talk action
pub fn agent_message_system_social(
    mut agent_messages: ResMut<AgentMessages>,
    mut agent_query: Query<&mut Agent>,
) {
    while !agent_messages.messages.is_empty() {
        let mut cooperation_list: Vec<(u32, Vec<u32>)> = Vec::new();

        // 1. Remove all messages from 'agent_messages' and store them in a vector
        let mut messages = std::mem::take(&mut agent_messages.messages);

        // Sort the messages first by `receiver_id`, then by `sender_id`, and finally by `MessageType`
        messages.sort_by(|a, b| {
            a.receiver_id
                .cmp(&b.receiver_id)
                .then_with(|| a.sender_id.cmp(&b.sender_id))
                .then_with(|| a.message_type.cmp(&b.message_type)) // Compare by MessageType as the final criterion
        });

        // 3. Organize agents by `agent.get_id()`
        let mut agents: Vec<_> = agent_query.iter_mut().collect();
        agents.sort_by_key(|agent| agent.get_id());

        // 4. Initialize the message counter
        let mut message_counter = 0;

        // 5. Process each message
        while let Some(message) = messages.pop() {
            let AgentMessage {
                sender_id,
                receiver_id,
                message_type,
            } = message;

            // Iterate through the agents to find the matching receiver
            while message_counter < agents.len() {
                // Get the agent that should process this message
                let agent = &mut agents[message_counter];

                if agent.get_id() == receiver_id {
                    //Handle based on message type
                    match message_type {
                        //To enter here, the sender must first set itself to follower
                        //Never Called
                        // MessageType::Cooperate(followers_to_handle) => {
                        //     if agent.is_follower(){
                        //         send_agent_message(
                        //             sender_id,
                        //             agent.get_leader_id(),
                        //             MessageType::Cooperate(followers_to_handle),
                        //             &mut agent_messages,
                        //         )
                        //     } else {
                        //         for agent_id in followers_to_handle {
                        //             if agent.calculate_cooperation_acceptance(agent_id){

                        //                 send_agent_message(
                        //                     agent.get_leader_id(),
                        //                     agent_id,
                        //                     MessageType::SetLeader(agent.get_id()),
                        //                     &mut agent_messages,
                        //                 )
                        //             } else {
                        //                 send_agent_message(
                        //                     agent.get_leader_id(),
                        //                     agent_id,
                        //                     MessageType::StopCooperating,
                        //                     &mut agent_messages,
                        //                 )

                        //             }
                        //         }
                        //     }
                        // }

                        //If a follower, pass on message to leader
                        MessageType::CheckStartCooperate => {
                            //If follower, already someones follower
                            //If leader, ask to become follower
                            //If not leader but calculate cooperation acceptance fails, don't become follower
                            if agent.is_follower() {
                                if agent.calculate_cooperation_acceptance(sender_id) {
                                    send_agent_message(
                                        sender_id,
                                        agent.get_leader_id(),
                                        MessageType::BecomeFollower,
                                        &mut agent_messages,
                                    )
                                }
                            } else if agent.is_leader() {
                                if agent.calculate_cooperation_acceptance(sender_id) {
                                    send_agent_message(
                                        agent.get_id(),
                                        sender_id,
                                        MessageType::BecomeFollower,
                                        &mut agent_messages,
                                    );
                                }
                            } else {
                                println!("Impossible statement on line {}", line!());
                            }
                        }
                        MessageType::BecomeFollower => {
                            //Check if agent is in the list already, if it is just push onto those interested in follow
                            if let Some((_, follower_list)) = cooperation_list
                                .iter_mut()
                                .find(|(id, _)| *id == agent.get_id())
                            {
                                follower_list.push(sender_id);
                            } else {
                                //Otherwise add a new set of interested followers
                                cooperation_list.push((agent.get_id(), vec![sender_id]));
                            }
                        }
                        MessageType::SetLeader(leader_id) => {
                            // Set agent as follower
                            agent.set_leader_id(leader_id);
                            agent.set_is_leader(false);
                            agent.set_is_follower(true);
                            agent.set_status(Status::Following);

                            //If the agent has followers, give them to new leader
                            if agent.has_followers() {
                                // Separate mutable borrow for leader agent
                                let followers = agent.get_followers();
                                if let Some(leader_agent) =
                                    agents.iter_mut().find(|agent| agent.get_id() == leader_id)
                                {
                                    leader_agent.add_follower(followers);
                                } else {
                                    println!("Error: Leader agent not found at line {}", line!());
                                }
                            }
                        }
                        MessageType::StopCooperating => {
                            agent.remove_follower(sender_id);
                            agent.set_status(Status::Idle);
                        }
                        _ => println!("Invalid Message type {} at line {}", message_type, line!()),
                    }
                    break;
                } else {
                    message_counter += 1; // Move to the next agent
                }
            }
        }
        process_cooperation_list(cooperation_list, &mut agent_query);
    }
}

//Move follower agents as well
pub fn agent_movement_system(
    mut agent_messages: ResMut<AgentMessages>,
    mut agent_query: Query<&mut Agent>,
    world_sim: ResMut<WorldSim>,
    world_actual: ResMut<GameWorld>,
    mcst_flag: ResMut<MCSTFlag>,
    running_flag: ResMut<RunningFlag>,
    mut commands: Commands,
) {
    let world;
    if mcst_flag.0 {
        world = &world_sim.0;
    } else if running_flag.0 {
        world = &world_actual;
    } else {
        return;
    }

    // 1. Remove all messages from `agent_messages` and store them in a vector
    let messages = std::mem::take(&mut agent_messages.messages);

    for message in messages {
        let AgentMessage {
            sender_id,
            receiver_id,
            message_type,
        } = message;
        if !matches!(message_type, MessageType::Move(_)) {
            send_agent_message(sender_id, receiver_id, message_type, &mut agent_messages);
            continue;
        }
        for mut agent in agent_query.iter_mut() {
            if agent.get_id() == message.receiver_id && agent.get_status() == Status::Moving {
                /* Debug */
                //println!("Agent {} entered into agent_movement_system with {}, checking loop", agent.get_id(), message_type.to_string());
                /* Debug */
                match message_type {
                    MessageType::Move((x, y)) => {
                        match world.move_agent(agent.get_id(), x as usize, y as usize) {
                            Ok(_) => {
                                if running_flag.0 {
                                    agent.move_to(x as f32, y as f32, &mut commands);
                                    //commands.entity(agent.get_entity()).insert(Transform::from_translation(Vec3::new(x as f32 * 32.0, y as f32 * 32.0, 1.0)));
                                }
                            }
                            Err(_) => println!(
                                "Error processing Agent {} move at line {} in systems.rs",
                                agent.get_id(),
                                line!()
                            ),
                        }
                    }
                    _ => println!("Invalid Message type {} at line {}", message_type, line!()),
                }
                break; // Exit the loop as we found the correct agent
            }
        }
    }
}

pub fn process_cooperation_list(
    cooperation_list: Vec<(u32, Vec<u32>)>,
    agent_query: &mut Query<&mut Agent>,
) {
    // Step 1: Group all agents into subsets
    let mut agent_groups: Vec<HashSet<u32>> = Vec::new();
    let mut leader_candidates: HashMap<u32, u32> = HashMap::new();

    for (agent_id, followers) in &cooperation_list {
        let mut group = HashSet::new();
        group.insert(*agent_id);
        for &follower_id in followers {
            group.insert(follower_id);
        }

        // Merge groups if they overlap
        let mut merged = false;
        for existing_group in &mut agent_groups {
            if !group.is_disjoint(existing_group) {
                existing_group.extend(&group);
                merged = true;
                break;
            }
        }
        if !merged {
            agent_groups.push(group);
        }

        *leader_candidates.entry(*agent_id).or_insert(0) += followers.len() as u32;
    }

    // Step 2: For each group, select the leader and process cooperation
    for group in agent_groups {
        let leader_id = group
            .iter()
            .max_by_key(|&&id| leader_candidates.get(&id).unwrap_or(&0))
            .unwrap();

        // Step 3: Process each agent in the group, including the leader
        for &agent_id in &group {
            // Fetch the agent (including the leader)
            let mut agent = agent_query
                .iter_mut()
                .find(|agent| agent.get_id() == agent_id)
                .unwrap();

            if agent_id == *leader_id {
                // Leader logic
                agent.set_is_leader(true);
            } else {
                // Follower logic
                // Agent was not originally interested in this leader
                if !cooperation_list
                    .iter()
                    .any(|(id, followers)| *id == *leader_id && followers.contains(&agent_id))
                {
                    if agent.calculate_cooperation_acceptance(*leader_id) {
                        // Agent accepts the cooperation
                        agent.set_is_follower(true);
                        agent.set_is_leader(false);
                        agent.set_leader_id(*leader_id);

                        // Fetch leader agent to add this follower
                        let mut leader_agent = agent_query
                            .iter_mut()
                            .find(|agent| agent.get_id() == *leader_id)
                            .unwrap();
                        leader_agent.add_follower(vec![agent_id]);
                    } else {
                        // Agent did not accept the cooperation
                        agent.set_leader_id(u32::MAX);
                    }
                // Agent was originally interested in this leader
                } else {
                    agent.set_is_follower(true);
                    agent.set_leader_id(*leader_id);
                    agent.set_status(Status::Following);

                    // Fetch leader agent to add this follower
                    let mut leader_agent = agent_query
                        .iter_mut()
                        .find(|agent| agent.get_id() == *leader_id)
                        .unwrap();
                    leader_agent.add_follower(vec![agent_id]);
                }
            }
        }
    }
}

//Don't forget to handle the system for leaving a group
//Don't forget the system for killing the leader of a group (as well as their next replacement)
//Don't forget the case where an agent gets a reward after it is dead (create treasure in its place)?
//Add error handling if the target is gone/dead
//Don't forget to handle the system for leaving a group
//Don't forget to set status to following
//Fix performing an invalid task until it dies
//Move removing the energy into the message handling
//Fix the movement issue of randomly moving to 0, as well as using th path isntead of the current position
//Opinion fix for multiple talkers
pub fn agent_message_system(
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut agent_query: Query<&mut Agent>,
    mcst_flag: ResMut<MCSTFlag>,
    running_flag: ResMut<RunningFlag>,
    world_sim: ResMut<WorldSim>,
    world_actual: ResMut<GameWorld>,
) {
    let world;
    if mcst_flag.0 {
        world = &world_sim.0;
    } else if running_flag.0 {
        world = &world_actual;
    } else {
        return;
    }
    let closest_village_vec = world.find_closest_villages();
    // 1. Remove all messages from `agent_messages` and store them in a vector
    let mut messages = std::mem::take(&mut agent_messages.messages);

    // Sort the messages first by `receiver_id`, then by `sender_id`, and finally by `MessageType`
    messages.sort_by(|a, b| {
        a.receiver_id
            .cmp(&b.receiver_id)
            .then_with(|| a.sender_id.cmp(&b.sender_id))
            .then_with(|| a.message_type.cmp(&b.message_type)) // Compare by MessageType as the final criterion
    });
    // 3. Organize agents by `agent.get_id()`
    let mut agents: Vec<_> = agent_query.iter_mut().collect();
    agents.sort_by_key(|agent| agent.get_id());

    /* DEBUG */
    // for message in messages.clone(){
    //     let AgentMessage { sender_id, receiver_id, message_type } = message;
    //     println!("Sender ID: {:?}, Reciever ID: {:?}, Message: {:?}", sender_id, receiver_id, message_type.to_string());
    // }
    // for agent in agents.iter() {
    //     println!("Agent ID: {:?}", agent.get_id());
    // }
    /* DEBUG */

    // Process each message
    while let Some(message) = messages.pop() {
        let AgentMessage {
            sender_id,
            receiver_id,
            message_type,
        } = message;

        // Match on the message_type first
        match message_type {
            MessageType::Attack(attack_value) => {
                // Find the receiver agent
                if let Some(agent_index) = agents
                    .iter()
                    .position(|agent| agent.get_id() == receiver_id)
                {
                    // Mutably borrow the agent
                    let agent = &mut agents[agent_index];

                    // Check conditions and process the agent
                    if agent.get_followers().is_empty() && agent.is_leader() {
                        agent.remove_energy(attack_value);
                        if agent.get_status() != Status::Retaliating
                            && agent.get_status() != Status::Fleeing
                            && agent.get_status() != Status::Recovering
                            && agent.get_status() != Status::Attacking
                        {
                            let status = agent.flight_or_fight(agent.get_agent_target_id(), false);
                            agent.set_status(status.clone());

                            if status == Status::Retaliating {
                                agent.set_retaliation_target(Target::Agent);
                                agent.set_retaliation_target_id(sender_id);
                                send_agent_message(
                                    agent.get_id(),
                                    agent.get_retaliation_target_id(),
                                    MessageType::Attack(5),
                                    &mut agent_messages,
                                );
                            } else {
                                if let Some(&(_, target_pos)) = closest_village_vec
                                    .iter()
                                    .find(|&&(id, _)| id == agent.get_id())
                                {
                                    agent.set_tile_target(Some(target_pos));
                                }
                            }
                        }
                    } else if agent.is_leader() {
                        let followers_ids = agent.get_followers().clone();
                        let num_followers = followers_ids.len();
                        let mut total_score = 0.0;

                        // Process the leader
                        let leader_fight_result = agent.flight_or_fight(sender_id, false);
                        if leader_fight_result == Status::Retaliating {
                            total_score += 0.33; // Leader contributes 0.33 to the total score
                        }
                        // Each follower contributes a fraction of the remaining 0.66 to the total score
                        let follower_contribution = if num_followers > 0 {
                            0.66 / num_followers as f32
                        } else {
                            println!(
                                "Error: Follower agent with no agents found at line {}",
                                !line!()
                            );
                            0.0
                        };
                        // Process followers
                        let _ = agent;
                        for follower_id in followers_ids.clone() {
                            if let Some(follower_index) = agents
                                .iter()
                                .position(|agent| agent.get_id() == follower_id)
                            {
                                let follower_agent = &mut agents[follower_index];
                                let follower_fight_result =
                                    follower_agent.flight_or_fight(sender_id, false);
                                if follower_fight_result == Status::Retaliating {
                                    total_score += follower_contribution;
                                }
                            } else {
                                println!(
                                    "Error: Follower agent with ID {} not found at line {}",
                                    follower_id,
                                    !line!()
                                );
                            }
                        }
                        let fight = total_score >= 0.5;
                        let agent = &mut agents[agent_index];
                        if fight {
                            agent.set_status(Status::Retaliating);
                            agent.set_retaliation_target(Target::Agent);
                            agent.set_retaliation_target_id(sender_id);
                            send_agent_message(
                                agent.get_id(),
                                agent.get_retaliation_target_id(),
                                MessageType::Attack((3 * (num_followers + 1)) as u8),
                                &mut agent_messages,
                            );
                        } else {
                            agent.set_status(Status::Fleeing);
                            if let Some(&(_, target_pos)) = closest_village_vec
                                .iter()
                                .find(|&&(id, _)| id == agent.get_id())
                            {
                                agent.set_tile_target(Some(target_pos));
                            }
                        }

                        let mut rng = rand::thread_rng();
                        let random_index = rng.gen_range(0..num_followers);
                        let selected_agent_id = if random_index == 0 {
                            agent.get_id()
                        } else {
                            followers_ids[random_index - 1]
                        };
                        if let Some(selected_agent) = agents
                            .iter_mut()
                            .find(|agent| agent.get_id() == selected_agent_id)
                        {
                            selected_agent.remove_energy(attack_value);
                        } else {
                            println!(
                                "Error: Selected agent with ID {} not found",
                                selected_agent_id
                            );
                        }
                    } else {
                        send_agent_message(
                            sender_id,
                            agent.get_leader_id(),
                            MessageType::Attack(attack_value),
                            &mut agent_messages,
                        );
                    }
                } else {
                    println!("Error: Agent with ID {} not found", receiver_id);
                }
            }
            MessageType::MonsterAttack(attack_value) => {
                // Find the receiver agent
                if let Some(agent_index) = agents
                    .iter()
                    .position(|agent| agent.get_id() == receiver_id)
                {
                    // Mutably borrow the agent
                    let agent = &mut agents[agent_index];

                    // Check conditions and process the agent
                    if agent.get_followers().is_empty() && agent.is_leader() {
                        agent.remove_energy(attack_value);
                        if agent.get_status() != Status::Retaliating
                            && agent.get_status() != Status::Fleeing
                            && agent.get_status() != Status::Recovering
                            && agent.get_status() != Status::Attacking
                        {
                            let status = agent.flight_or_fight(sender_id, true);
                            agent.set_status(status.clone());

                            if status == Status::Retaliating {
                                agent.set_retaliation_target(Target::Monster);
                                agent.set_retaliation_target_id(sender_id);
                                send_monster_message(
                                    agent.get_id(),
                                    agent.get_retaliation_target_id(),
                                    MessageType::Attack(5),
                                    &mut monster_messages,
                                );
                            } else {
                                if let Some(&(_, target_pos)) = closest_village_vec
                                    .iter()
                                    .find(|&&(id, _)| id == agent.get_id())
                                {
                                    agent.set_tile_target(Some(target_pos));
                                }
                            }
                        }
                    } else if agent.is_leader() {
                        let followers_ids = agent.get_followers().clone();
                        let num_followers = followers_ids.len();
                        let mut total_score = 0.0;

                        // Process the leader
                        let leader_fight_result = agent.flight_or_fight(sender_id, true);
                        if leader_fight_result == Status::Retaliating {
                            total_score += 0.33; // Leader contributes 0.33 to the total score
                        }
                        // Each follower contributes a fraction of the remaining 0.66 to the total score
                        let follower_contribution = if num_followers > 0 {
                            0.66 / num_followers as f32
                        } else {
                            println!(
                                "Error: Follower agent with no agents found at line {}",
                                !line!()
                            );
                            0.0
                        };
                        // Process followers
                        let _ = agent;
                        for follower_id in followers_ids.clone() {
                            if let Some(follower_index) = agents
                                .iter()
                                .position(|agent| agent.get_id() == follower_id)
                            {
                                let follower_agent = &mut agents[follower_index];
                                let follower_fight_result =
                                    follower_agent.flight_or_fight(sender_id, false);
                                if follower_fight_result == Status::Retaliating {
                                    total_score += follower_contribution;
                                }
                            } else {
                                println!(
                                    "Error: Follower agent with ID {} not found at line {}",
                                    follower_id,
                                    !line!()
                                );
                            }
                        }
                        let fight = total_score >= 0.5;
                        let agent = &mut agents[agent_index];
                        if fight {
                            agent.set_status(Status::Retaliating);
                            agent.set_retaliation_target(Target::Agent);
                            agent.set_retaliation_target_id(sender_id);
                            send_monster_message(
                                agent.get_id(),
                                agent.get_retaliation_target_id(),
                                MessageType::Attack((3 * (num_followers + 1)) as u8),
                                &mut monster_messages,
                            );
                        } else {
                            agent.set_status(Status::Fleeing);
                            if let Some(&(_, target_pos)) = closest_village_vec
                                .iter()
                                .find(|&&(id, _)| id == agent.get_id())
                            {
                                agent.set_tile_target(Some(target_pos));
                            }
                        }

                        let mut rng = rand::thread_rng();
                        let random_index = rng.gen_range(0..num_followers);
                        let selected_agent_id = if random_index == 0 {
                            agent.get_id()
                        } else {
                            followers_ids[random_index - 1]
                        };
                        if let Some(selected_agent) = agents
                            .iter_mut()
                            .find(|agent| agent.get_id() == selected_agent_id)
                        {
                            selected_agent.remove_energy(attack_value);
                        } else {
                            println!(
                                "Error: Selected agent with ID {} not found",
                                selected_agent_id
                            );
                        }
                    } else {
                        send_agent_message(
                            sender_id,
                            agent.get_leader_id(),
                            MessageType::MonsterAttack(5),
                            &mut agent_messages,
                        );
                    }
                } else {
                    println!("Error: Agent with ID {} not found", receiver_id);
                }
            }
            MessageType::Reward(reward_value) => {
                if let Some(agent_index) = agents
                    .iter()
                    .position(|agent| agent.get_id() == receiver_id)
                {
                    let agent = &mut agents[agent_index];

                    if agent.get_followers().is_empty() && agent.is_leader() {
                        agent.add_reward(reward_value);
                    } else if agent.is_leader() {
                        let followers_ids = agent.get_followers().clone();
                        let new_amount = (reward_value as f32) / 0.66;
                        let rounded_amount = new_amount.ceil() as u32;
                        agent.add_reward(rounded_amount);
                        let _ = agent;
                        for follower_id in followers_ids.clone() {
                            if let Some(follower_index) = agents
                                .iter()
                                .position(|agent| agent.get_id() == follower_id)
                            {
                                let follower_agent = &mut agents[follower_index];
                                follower_agent.add_reward(rounded_amount);
                            } else {
                                println!(
                                    "Error: Follower agent with ID {} not found at line {}",
                                    follower_id,
                                    !line!()
                                );
                            }
                        }
                    } else {
                        agent.add_reward(reward_value);
                    }
                } else {
                    println!("Error: Agent with ID {} not found", receiver_id);
                }
            }
            MessageType::Steal(steal_amount) => {
                let mut stealth_value = 0.0;
                if let Some(agent_index) =
                    agents.iter().position(|agent| agent.get_id() == sender_id)
                {
                    let agent = &mut agents[agent_index];
                    let genes = agent.get_genes();
                    stealth_value =
                        genes.return_type_score(GeneType::Stealth);
                } else {
                    println!("Error: Agent with ID {} not found", sender_id);
                }
                if let Some(agent_index) = agents
                    .iter()
                    .position(|agent| agent.get_id() == receiver_id)
                {
                    let agent = &mut agents[agent_index];

                    if agent.get_followers().is_empty() && agent.is_leader() {
                        let current_amount = agent.get_reward();
                        agent.remove_reward(steal_amount);
                        let new_amount = agent.get_reward();
                        let stolen_amount = current_amount - new_amount;

                        send_agent_message(
                            agent.get_id(),
                            sender_id,
                            MessageType::Reward(stolen_amount),
                            &mut agent_messages,
                        );
                        if agent.notice_steal(stealth_value) {
                            agent.modify_opinion(sender_id, -0.1);
                            if agent.retaliate_steal(sender_id) {
                                if agent.get_status() != Status::Retaliating
                                    && agent.get_status() != Status::Fleeing
                                    && agent.get_status() != Status::Recovering
                                    && agent.get_status() != Status::Attacking
                                {
                                    agent.set_status(Status::Retaliating);
                                    agent.set_retaliation_target(Target::Agent);
                                    agent.set_retaliation_target_id(sender_id);
                                    send_agent_message(
                                        agent.get_id(),
                                        agent.get_retaliation_target_id(),
                                        MessageType::Attack(5),
                                        &mut agent_messages,
                                    );
                                }
                            }
                        }
                    } else if agent.is_leader() {
                        let followers_ids = agent.get_followers().clone();

                        let current_amount = agent.get_reward();
                        agent.remove_reward(steal_amount);
                        let new_amount = agent.get_reward();
                        let stolen_amount = current_amount - new_amount;
                        send_agent_message(
                            agent.get_id(),
                            sender_id,
                            MessageType::Reward(stolen_amount),
                            &mut agent_messages,
                        );
                        if agent.notice_steal(stealth_value) {
                            agent.modify_opinion(sender_id, -0.1);

                            let mut total_score = 0.0;
                            if agent.retaliate_steal(sender_id) {
                                total_score += 0.33;
                            }
                            let num_followers = followers_ids.len();
                            let _ = agent;
                            // Each follower contributes a fraction of the remaining 0.66 to the total score
                            let follower_contribution = if num_followers > 0 {
                                0.66 / num_followers as f32
                            } else {
                                println!(
                                    "Error: Follower agent with no agents found at line {}",
                                    !line!()
                                );
                                0.0
                            };

                            for follower_id in followers_ids.clone() {
                                if let Some(follower_index) = agents
                                    .iter_mut()
                                    .position(|agent| agent.get_id() == follower_id)
                                {
                                    let follower_agent = &mut agents[follower_index];
                                    follower_agent.modify_opinion(sender_id, -0.1);
                                    if follower_agent.retaliate_steal(sender_id) {
                                        total_score += follower_contribution;
                                    }
                                } else {
                                    println!(
                                        "Error: Follower agent with ID {} not found at line {}",
                                        follower_id,
                                        !line!()
                                    );
                                }
                            }
                            let agent = &mut agents[agent_index];
                            if total_score >= 0.5 {
                                if agent.get_status() != Status::Retaliating
                                    && agent.get_status() != Status::Fleeing
                                    && agent.get_status() != Status::Recovering
                                    && agent.get_status() != Status::Attacking
                                {
                                    agent.set_status(Status::Retaliating);
                                    agent.set_retaliation_target(Target::Agent);
                                    agent.set_retaliation_target_id(sender_id);
                                    send_agent_message(
                                        agent.get_id(),
                                        agent.get_retaliation_target_id(),
                                        MessageType::Attack((3 * (num_followers + 1)) as u8),
                                        &mut agent_messages,
                                    );
                                }
                            }
                        }
                    } else {
                        let current_amount = agent.get_reward();
                        agent.remove_reward(steal_amount);
                        let new_amount = agent.get_reward();
                        let stolen_amount = current_amount - new_amount;

                        send_agent_message(
                            agent.get_id(),
                            sender_id,
                            MessageType::Reward(stolen_amount),
                            &mut agent_messages,
                        );
                        send_agent_message(
                            sender_id,
                            agent.get_leader_id(),
                            MessageType::Reward(0),
                            &mut agent_messages,
                        );
                    }
                } else {
                    println!("Error: Agent with ID {} not found", receiver_id);
                }
            }
            MessageType::Rest => {
                if let Some(agent_index) = agents
                    .iter()
                    .position(|agent| agent.get_id() == receiver_id)
                {
                    let agent = &mut agents[agent_index];

                    agent.add_energy(10);
                    if agent.get_energy() == agent.get_max_energy() {
                        agent.set_status(Status::Idle);
                    }
                } else {
                    println!("Error: Agent with ID {} not found", receiver_id);
                }
            }
            MessageType::Talk => {
                let talker_opinions;

                if let Some(agent_index) =
                    agents.iter().position(|agent| agent.get_id() == sender_id)
                {
                    let talker_agent = &mut agents[agent_index];
                    talker_opinions = talker_agent.get_opinions().clone();
                } else {
                    println!("Error: Agent with ID {} not found", sender_id);
                    continue;
                }

                if let Some(agent_index) = agents
                    .iter_mut()
                    .position(|agent| agent.get_id() == receiver_id)
                {
                    let agent = &mut agents[agent_index];
                    agent.influence_opinions(talker_opinions);
                } else {
                    println!("Error: Agent with ID {} not found", receiver_id);
                }
            }
            _ => {
                println!("Unhandled message type at line {}", line!());
            } //
              // MessageType::Cooperate(_) =>todo!(),
              // MessageType::Energy(_, _) => todo!(),
              // MessageType::Inherit(_, _) => todo!(),
        }
    }
}

//  __  __                 _
// |  \/  |               | |
// | \  / | ___  _ __  ___| |_ ___ _ __
// | |\/| |/ _ \| '_ \/ __| __/ _ \ '__|
// | |  | | (_) | | | \__ \ ||  __/ |
// |_|  |_|\___/|_| |_|___/\__\___|_|

#[derive(Resource)]
pub struct MonsterMessages {
    pub(crate) messages: Vec<MonsterMessage>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct MonsterMessage {
    sender_id: u32,
    receiver_id: u32,
    message_type: MessageType,
}

impl MonsterMessage {
    pub fn new(sender_id: u32, receiver_id: u32, message_type: MessageType) -> Self {
        MonsterMessage {
            sender_id: sender_id,
            receiver_id: receiver_id,
            message_type: message_type,
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            message_type: self.message_type.copy(),
        }
    }
}

impl MonsterMessages {
    pub fn new() -> Self {
        MonsterMessages {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: MonsterMessage) {
        self.messages.push(message);
    }
}

pub fn monster_message_system(
    mut _commands: Commands,
    mut monster_messages: ResMut<MonsterMessages>,
    mut query: Query<&mut Monster>,
) {
    // Handle received messages
    for message in &monster_messages.messages {
        let receiver_id = message.receiver_id;

        for mut monster in query.iter_mut() {
            if monster.get_id() == receiver_id {
                match message.message_type {
                    MessageType::Attack(damage) => monster.remove_energy(damage),
                    MessageType::Reward(_) => todo!(),
                    MessageType::Steal(_) => todo!(),
                    MessageType::MonsterAttack(_) => todo!(),
                    MessageType::Cooperate(_) => todo!(),
                    MessageType::StopCooperating => todo!(),
                    MessageType::Move(_) => todo!(),
                    MessageType::Energy(_, _) => todo!(),
                    MessageType::GroupDamage(_) => todo!(),
                    MessageType::GroupReward(_) => todo!(),
                    MessageType::Inherit(_, _) => todo!(),
                    MessageType::BecomeFollower => todo!(),
                    _ => todo!(),
                }
            }
        }
    }

    // Clean up processed messages
    monster_messages.messages.clear();
}
//  _______
// |__   __|
//    | |_ __ ___  __ _ ___ _   _ _ __ ___
//    | | '__/ _ \/ _` / __| | | | '__/ _ \
//    | | | |  __/ (_| \__ \ |_| | | |  __/
//    |_|_|  \___|\__,_|___/\__,_|_|  \___|

#[derive(Resource)]
pub struct TreasureMessages {
    pub(crate) messages: Vec<TreasureMessage>,
}

impl TreasureMessages {
    pub fn new() -> Self {
        TreasureMessages {
            messages: Vec::new(),
        }
    }

    pub fn _add_message(&mut self, message: TreasureMessage) {
        self.messages.push(message);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct TreasureMessage {
    sender_id: u32,
    receiver_id: u32,
    message_type: MessageType,
}

impl TreasureMessage {
    pub fn new(sender_id: u32, receiver_id: u32, message_type: MessageType) -> Self {
        TreasureMessage {
            sender_id: sender_id,
            receiver_id: receiver_id,
            message_type: message_type,
        }
    }

    pub fn copy(&self) -> Self {
        Self {
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            message_type: self.message_type.copy(),
        }
    }
}

pub fn treasure_message_system(
    mut commands: Commands,
    mut treasure_messages: ResMut<TreasureMessages>,
    mut agent_messages: ResMut<AgentMessages>,
    mut query: Query<&mut Monster>,
) {
    // Handle received messages
    for message in &treasure_messages.messages {
        let receiver_id = message.receiver_id;

        for treasure in query.iter_mut() {
            if treasure.get_id() == receiver_id {
                match message.message_type {
                    MessageType::Steal(_) => {
                        let new_message = AgentMessage::new(
                            0,
                            message.sender_id,
                            MessageType::Reward(treasure.get_reward()),
                        );
                        agent_messages.add_message(new_message);

                        commands.entity(treasure.get_entity()).despawn();
                    }
                    MessageType::Attack(_) => todo!(),
                    MessageType::Reward(_) => todo!(),
                    MessageType::MonsterAttack(_) => todo!(),
                    MessageType::Cooperate(_) => todo!(),
                    MessageType::StopCooperating => todo!(),
                    MessageType::Move(_) => todo!(),
                    MessageType::Energy(_, _) => todo!(),
                    MessageType::GroupDamage(_) => todo!(),
                    MessageType::GroupReward(_) => todo!(),
                    MessageType::Inherit(_, _) => todo!(),
                    MessageType::BecomeFollower => todo!(),
                    _ => todo!(),
                }
            }
        }
    }

    // Clean up processed messages
    treasure_messages.messages.clear();
}

//   _____ _       _           _
//  / ____| |     | |         | |
// | |  __| | ___ | |__   __ _| |
// | | |_ | |/ _ \| '_ \ / _` | |
// | |__| | | (_) | |_) | (_| | |
//  \_____|_|\___/|_.__/ \__,_|_|

//Don't forget the case for  removing an agent that is dead that is part of a group
pub fn cleanup_system(
    mut commands: Commands,
    mut query_a: Query<&mut Agent>,
    mut query_m: Query<&mut Monster>,
    mut world: ResMut<GameWorld>,
    mut agent_messages: ResMut<AgentMessages>,
) {
    // Handle received messages
    for mut agent in query_a.iter_mut() {
        if agent.get_energy() == 0 || agent.get_status() == Status::Dead {
            if agent.is_follower() {
                send_agent_message(
                    agent.get_id(),
                    agent.get_leader_id(),
                    MessageType::Reward(agent.get_reward()),
                    &mut agent_messages,
                )
            }
            //If reward is currently greater than 0, create a treasure at that position
            //if agent.get_reward() > 0
            //if agent.get_group_size > 1
            //send an inherit message to that agent
            //create treasure at the current position
            //println!("Dead and burried is agent {}",agent.get_id());
            let _removed_agent = world.remove_agent(agent.get_id());
            commands.entity(agent.get_entity()).despawn();
        }
        if agent.get_followers().len() == 0 {
            agent.set_is_leader(false);
        }
    }
    for monster in query_m.iter_mut() {
        if monster.get_energy() == 0 || monster.get_status() == Status::Dead {
            commands.entity(monster.get_entity()).despawn();
        }
    }
    //If simulation is finished, and flag is set to true,
}

#[allow(unused_variables)]
pub fn perform_action(
    commands: Commands,
    mut treasure_messages: ResMut<TreasureMessages>,
    mcst_flag: ResMut<MCSTFlag>,
    mut agent_query: Query<&mut Agent>,
    world_sim: ResMut<WorldSim>,
    mut agent_messages: ResMut<AgentMessages>,
    monster_messages: ResMut<MonsterMessages>,
) {
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
    }
}

pub fn send_agent_message(
    sender_id: u32,
    receiver_id: u32,
    message_content: MessageType,
    agent_messages: &mut AgentMessages,
) {
    let message = AgentMessage::new(sender_id, receiver_id, message_content);
    agent_messages.messages.push(message);
}

pub fn send_monster_message(
    sender_id: u32,
    receiver_id: u32,
    message_content: MessageType,
    monster_messages: &mut MonsterMessages,
) {
    let message = MonsterMessage::new(sender_id, receiver_id, message_content);
    monster_messages.messages.push(message);
}

pub fn send_treasure_message(
    sender_id: u32,
    receiver_id: u32,
    message_content: MessageType,
    treasure_messages: &mut TreasureMessages,
) {
    let message = TreasureMessage::new(sender_id, receiver_id, message_content);
    treasure_messages.messages.push(message);
}
