use bevy::prelude::*;
use crate::entities::agent::{Agent, Status, Target};
use crate::entities::monster::Monster;
use crate::errors::MyError;
use crate::mcst::NpcAction;
use crate::World;

#[derive()]
#[derive(Clone)]
#[allow(dead_code)]
pub enum MessageType{
    Attack(u8),
    MonsterAttack(u8),
    GroupDamage(u8),
    GroupReward(u32),
    Reward(u32),
    Steal(u32),
    Cooperate(Vec<u32>),
    BecomeFollower,
    StopCooperating,
    Move((u32,u32)),
    //True is add, false is remove
    Energy(bool, u8),
    Inherit(Vec<u32>, u32)
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
            MessageType::StopCooperating => MessageType::StopCooperating,
            MessageType::BecomeFollower => MessageType::BecomeFollower,
            MessageType::Move((val1,val2)) => MessageType::Move((*val1, *val2)),
            MessageType::Energy(flag, amount) => MessageType::Energy(*flag, *amount),
            MessageType::GroupReward(reward) => MessageType::GroupReward(*reward),
            MessageType::Inherit(agents, reward) => MessageType::Inherit(agents.clone(), *reward),
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

impl AgentMessage{
    pub fn new(
        sender_id: u32,
        receiver_id: u32,
        message_type: MessageType,) -> Self {
            AgentMessage{
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
}

//Don't forget to handle the system for leaving a group
//Don't forget the system for killing the leader of a group (as well as their next replacement)
//Don't forget the case where an agent gets a reward after it is dead (create treasure in its place)?
pub fn agent_message_system(
    //mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    world: ResMut<World>,
    mut query: Query<&mut Agent>,
    mut commands: Commands,
) {
    
    let mut new_messages = AgentMessages::new();
    // Handle received messages
    while !agent_messages.messages.is_empty() {
        for message in &mut agent_messages.messages {
            let receiver_id = message.receiver_id;

            for mut agent in query.iter_mut() {
                if agent.get_status() != Status::Dead{
                    if agent.get_id() == receiver_id {
                        match message.message_type{

                            //Agent is attacked
                            //If agent is part of a group, send a GroupDamage
                            //Otherwise, remove damage from single agent
                            MessageType::Attack(damage) => {
                                if agent.is_follower() {
                                    // Send an attack message to the leader
                                    let leader_id = agent.get_leader_id();
                                    let new_message = AgentMessage::new (
                                        agent.get_id(), 
                                        leader_id,
                                        MessageType::Attack(damage),
                                    );
                                    new_messages.add_message(new_message);
                                } else if agent.is_leader() {
                                    let group_size = agent.get_group_size();
                                    let damage_per_agent = damage / group_size as u8;
                                    let remainder = damage % group_size as u8;
                                    
                                    // Send GroupDamage to all agents in the group
                                    for follower_id in agent.get_followers() {
                                        let new_message = AgentMessage::new (
                                            agent.get_id(), 
                                            follower_id,
                                            MessageType::GroupDamage(damage_per_agent),
                                        );
                                        new_messages.add_message(new_message);
                                    }
                                    
                                    // Leader takes the remainder damage
                                    agent.remove_energy(remainder + damage_per_agent);
                                } else {
                                    agent.remove_energy(damage);
                                    if agent.get_status() == Status::Dead{
                                        let new_message = AgentMessage::new (
                                            agent.get_id(), 
                                            message.sender_id,
                                            MessageType::Reward(agent.get_reward()),
                                        );
                                        new_messages.add_message(new_message);
                                    }
                                }
                            },
                            MessageType::GroupDamage(damage) => {
                                agent.remove_energy(damage);
                                    if agent.get_status() == Status::Dead{
                                        let new_message = AgentMessage::new (
                                            agent.get_id(), 
                                            message.sender_id,
                                            MessageType::Reward(agent.get_reward()),
                                        );
                                        new_messages.add_message(new_message);
                                    }
                            },


                            MessageType::Reward(reward) => 
                            if agent.is_follower() {
                                // Send an attack message to the leader
                                let leader_id = agent.get_leader_id();
                                let new_message = AgentMessage::new (
                                    agent.get_id(), 
                                    leader_id,
                                    MessageType::Reward(reward),
                                );
                                new_messages.add_message(new_message);
                            } else if agent.is_leader() {
                                if agent.get_status() == Status::Dead{
                                    let new_leader = agent.get_followers().first().unwrap().clone();
                                    agent.remove_follower(new_leader);
                                    let new_message = AgentMessage::new (
                                        agent.get_id(), 
                                        new_leader,
                                        MessageType::Inherit(agent.get_followers().clone(), agent.get_reward()),
                                    );
                                    new_messages.add_message(new_message);
                                    
                                    let new_message_2 = AgentMessage::new (
                                        agent.get_id(), 
                                        agent.get_followers().first().unwrap().clone(),
                                        MessageType::Reward(reward),
                                    );
                                    new_messages.add_message(new_message_2);
                                } else{
                                    let group_size = agent.get_group_size();
                                    let reward_per_agent = reward / group_size;
                                    let remainder = reward % group_size;
                                    
                                    // Send GroupReward to all agents in the group
                                    for follower_id in agent.get_followers() {
                                        let new_message = AgentMessage::new (
                                            agent.get_id(), 
                                            follower_id,
                                            MessageType::GroupReward(reward_per_agent),
                                        );
                                        new_messages.add_message(new_message);
                                    }
                                    
                                    // Leader takes the remainder damage
                                    agent.add_reward(remainder + reward_per_agent);
                                }
                            } else {
                                agent.add_reward(reward)
                            },
                            MessageType::GroupReward(reward) => agent.add_reward(reward),



                            MessageType::Steal(amount) => {
                                let stealing_amount: u32;
                            
                                if amount == 0 {
                                    stealing_amount = (agent.get_reward() / 10) as u32; // Perform integer division
                                } else {
                                    stealing_amount = amount;
                                }
                            
                                agent.remove_reward(stealing_amount);
                            
                                let new_message = AgentMessage::new (
                                    agent.get_id(), 
                                    message.sender_id,
                                    MessageType::Reward(stealing_amount),
                                );
                                new_messages.add_message(new_message);
                            },
                            MessageType::MonsterAttack(damage) => {
                                agent.remove_energy(damage);
                                if agent.get_status() == Status::Dead{
                                    let new_message = MonsterMessage::new (
                                        agent.get_id(), 
                                        message.sender_id,
                                        MessageType::Reward(agent.get_reward()),
                                    );
                                    monster_messages.add_message(new_message);
                                }
                            },
                            MessageType::Cooperate(ref agents) => {
                                agent.set_is_leader(true);
                                agent.set_is_follower(false);
                                agent.add_follower(agents.clone());
                                for id in agents{
                                    let new_message = AgentMessage::new(
                                        agent.get_id(),
                                        *id,
                                        MessageType::BecomeFollower,
                                    );
                                    new_messages.add_message(new_message);
                                }
                            },
                            MessageType::BecomeFollower => {
                                //If its a leader or has followers, send error
                                if agent.get_group_size() > 0{
                                    println!("Error in group size handling for BecomeFollower");
                                } 
                                agent.set_is_leader(false);
                                agent.set_is_follower(true);
                                agent.set_leader_id(message.sender_id);
                            },
                            
                            //It assumed the agent did the necessary removal from cooperating on its end
                            MessageType::StopCooperating => {
                                agent.remove_follower(message.sender_id);
                                if agent.get_group_size() <= 0 {
                                    agent.set_is_leader(false);
                                }
                            },

                            MessageType::Move((x, y)) => {
                                agent.move_to(x as f32, y as f32, &mut commands);
                                // Call the move_between_tiles function to move the agent to the next position in the path
                                match world.move_agent(agent.get_id(), x as usize, y as usize){
                                    Ok(it) => it,
                                    Err(_) => println!("Invalid Move"),
                                }
                            },
                            MessageType::Energy(flag, amount) => {
                                if flag{
                                    agent.add_energy(amount);
                                }else{
                                    agent.remove_energy(amount);
                                }
                            },
                            MessageType::Inherit(ref followers, reward) => {
                                let cloned_followers = followers.clone();
                                agent.add_reward(reward);
                                agent.add_follower(cloned_followers);
                                agent.set_is_leader(true);
                            },
                        }
                    }
                }
            }
        }

        agent_messages.messages.clear();
         // Add any messages in new_messages to agent_messages
         for message in &new_messages.messages{
            agent_messages.add_message(message.clone())
         }

         new_messages.messages.clear();
    }
}




//  __  __                 _            
// |  \/  |               | |           
// | \  / | ___  _ __  ___| |_ ___ _ __ 
// | |\/| |/ _ \| '_ \/ __| __/ _ \ '__|
// | |  | | (_) | | | \__ \ ||  __/ |   
// |_|  |_|\___/|_| |_|___/\__\___|_|   
                                                                    

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

impl MonsterMessage{
    pub fn new(
        sender_id: u32,
        receiver_id: u32,
        message_type: MessageType,) -> Self {
            MonsterMessage{
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
                match message.message_type{
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

impl TreasureMessage{
    pub fn new(
        sender_id: u32,
        receiver_id: u32,
        message_type: MessageType,) -> Self {
            TreasureMessage{
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
                match message.message_type{
                    MessageType::Steal(_) => {
                        let new_message = AgentMessage::new (
                            0, 
                            message.sender_id,
                            MessageType::Reward(treasure.get_reward()),
                        );
                        agent_messages.add_message(new_message);
                            
                        commands.entity(treasure.get_entity()).despawn();
                    },
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
    mut world: ResMut<World>,
    mut agent_messages: ResMut<AgentMessages>,
) {
    // Handle received messages
    for mut agent in query_a.iter_mut() {
        if agent.get_energy() == 0 || agent.get_status() == Status::Dead{
            if agent.is_follower(){
                send_agent_message(agent.get_id(), agent.get_leader_id(),
                 MessageType::Reward(agent.get_reward()), &mut agent_messages)
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
        if agent.get_followers().len() == 0{ 
            agent.set_is_leader(false);
        }
    }
    for monster in query_m.iter_mut() {
        if monster.get_energy() == 0 || monster.get_status() == Status::Dead{
            commands.entity(monster.get_entity()).despawn();
        }
    }
}


#[allow(unused_variables)]
//Add error handling if the target is gone/dead
//Don't forget to handle the system for leaving a group
//Don't forget to set status to following
//Fix performing an invalid task until it dies
//Move removing the energy into the message handling
//Fix the movement issue of randomly moving to 0, as well as using th path isntead of the current position
pub fn perform_action(
    mut query: Query<&mut Agent>,
    mut world: ResMut<World>,
    mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
) {
    
    for mut agent in query.iter_mut() {
        if !(agent.is_leader() && !agent.get_followers().is_empty()) {
            let current_target = agent.get_target();
            match current_target {
                Target::Agent => {
                    match world.get_agent_position(agent.get_agent_target_id()) {
                        Ok(agent_position) => {
                            let (x, y) = agent_position;
                            agent.set_tile_target(Some((x as u32, y as u32)));
                        }
                        Err(MyError::AgentNotFound) => {
                            println!("Agent not found in system::perform_action line {} from agent {}",284, agent.get_id());
                        }
                        _ => {} // Handle other errors if needed
                    }
                }
                Target::Monster => {
                    match world.get_monster_position(agent.get_monster_target_id()) {
                        Ok(monster_position) => {
                            let (x, y) = monster_position;
                            agent.set_tile_target(Some((x as u32, y as u32)));
                        }
                        Err(MyError::MonsterNotFound) => {
                            println!("Monster not found in system::perform_action line {}",296);
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
                    println!("Invalid Target in system::perform_action line {} from agent {}",296, agent.get_id());
                }
                Target::Tile => {
                    todo!()
                }
            }

            // Check if the agent's current position is equal to the tile target
            let (x, y) = agent.get_position();
            if (x, y) == agent.get_tile_target().unwrap_or_default() {
            //     // Continue with action logic
                let action = agent.get_action();
                //Match the type of action
                match action {
                    NpcAction::Attack => {
                        //Match the current target for the Attack action
                        match current_target{
                            //For the target Agent of the Attack action
                            Target::Agent => {
                                let id = agent.get_agent_target_id();
                                let message = MessageType::Attack(10);
                                follower_actions(
                                    agent.clone(),
                                    message,
                                    id,
                                    &mut agent_messages,
                                );
                                send_agent_message(
                                    agent.get_id(),
                                    id,
                                    MessageType::Attack(10),
                                    &mut agent_messages,
                                );
                                agent.remove_energy(5);
                                agent.set_status(Status::Working);
                            },
                            Target::Monster => {
                                let id = agent.get_monster_target_id();
                                send_monster_message(
                                    agent.get_id(),
                                    id,
                                    MessageType::Attack(10),
                                    &mut monster_messages,
                                );
                                agent.remove_energy(5);
                                if agent.is_leader() && agent.get_followers().len() > 0{
                                    for agent_id in agent.get_followers(){
                                        send_agent_message(
                                            agent.get_id(),
                                            id,
                                            MessageType::Energy(false, 5),
                                            &mut agent_messages,
                                        );
                                    }
                                }
                                agent.set_status(Status::Working);
                            },
                            _ => println!("Invalid Target in system::perform_action line {}",506),
                        }
                    }
                    NpcAction::Steal => {
                        //Match the current target for the Attack action
                        match current_target{
                            //For the target Agent of the Attack action
                            Target::Agent => {
                                
                                let id = agent.get_agent_target_id();
                                send_agent_message(
                                    agent.get_id(),
                                    id,
                                    MessageType::Steal(10),
                                    &mut agent_messages,
                                );
                                agent.remove_energy(20);
                            },
                            Target::Treasure => {
                                let id = agent.get_agent_target_id();
                                send_treasure_message(
                                    agent.get_id(),
                                    id,
                                    MessageType::Steal(10),
                                    &mut treasure_messages,
                                );
                                let _treasure = world.remove_treasure(id);
                                agent.remove_energy(5);
                            },
                            _ => println!("Invalid Target in system::perform_action line {}",536),
                        }
                    }
                    NpcAction::Rest => {
                        //Match the current target for the rest action
                        match current_target{
                            Target::Tile => {
                                agent.add_energy(10);
                                if agent.get_energy() == agent.get_max_energy() {
                                    agent.set_status(Status::Idle);
                                }
                            },
                            _ => println!("Invalid Target in system::perform_action line {}",548)
                        }
                    }
                    NpcAction::Talk => {
                        // Logic for moving to a monster
                    }
                    NpcAction::None => {
                        // Logic for moving to a monster
                    }
                }
            } else {
                
                // If the agent is not at the target position, initiate travel
                match agent.travel(world.get_grid(), &mut commands) {
                    Ok(_) => {
                        if agent.is_leader() && agent.get_followers().len() > 0{
                            for agent_id in agent.get_followers(){
                                send_agent_message(
                                    agent.get_id(),
                                    agent_id,
                                    MessageType::Move(agent.get_position()),
                                    &mut agent_messages,
                                );
                            }
                        }
                    },
                    Err(_) => println!("Invalid Target in system::perform_action line {}",573),
                }; 
                agent.set_status(Status::Moving);
                // Call the move_between_tiles function to move the agent to the next position in the path
                
                match world.move_agent(agent.get_id(), x as usize, y as usize){
                    Ok(it) => it,
                    Err(_) => println!("Invalid Move"),
                }
            }
        }
        
    }
}

fn follower_actions(
    agent : Agent,
    message : MessageType,
    target_id: u32,
    agent_messages: &mut AgentMessages,
){
    if agent.is_leader() && agent.get_followers().len() > 0{
        for agent_id in agent.get_followers(){
            send_agent_message(
                agent_id,
                target_id,
                message.copy(),
                agent_messages,
            );  
            send_agent_message(
                agent_id,
                agent_id,
                match message{
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

pub fn send_agent_message(
    sender_id: u32,
    receiver_id: u32,
    message_content: MessageType,
    agent_messages: &mut AgentMessages,
) {
    let message = AgentMessage::new (
        sender_id,
        receiver_id,
        message_content,
    );
    agent_messages.messages.push(message);
}

pub fn send_monster_message(
    sender_id: u32,
    receiver_id: u32,
    message_content: MessageType,
    monster_messages: &mut MonsterMessages,
) {
    let message = MonsterMessage::new (
        sender_id,
        receiver_id,
        message_content,
    );
    monster_messages.messages.push(message);
}

pub fn send_treasure_message(
    sender_id: u32,
    receiver_id: u32,
    message_content: MessageType,
    treasure_messages: &mut TreasureMessages,
) {
    let message = TreasureMessage::new (
        sender_id,
        receiver_id,
        message_content,
    );
    treasure_messages.messages.push(message);
}