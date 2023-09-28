use bevy::prelude::*;
use crate::entities::agent::{Agent, Status, Target};
use crate::entities::monster::Monster;
use crate::errors::MyError;
use crate::mcst::NpcAction;
use crate::World;

#[derive()]
#[derive(Clone)]
pub enum MessageType{
    Attack(u8),
    MonsterAttack(u8),
    Reward(u32),
    Steal(u32),
    Cooperate(Vec<u32>),
}

impl MessageType {
    // Manually implement the copy method
    pub fn copy(&self) -> Self {
        match self {
            MessageType::Attack(val) => MessageType::Attack(*val),
            MessageType::MonsterAttack(val) => MessageType::MonsterAttack(*val),
            MessageType::Reward(val) => MessageType::Reward(*val),
            MessageType::Steal(val) => MessageType::Steal(*val),
            MessageType::Cooperate(vec) => MessageType::Cooperate(vec.clone()),
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
#[derive(Copy, Clone)]
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

pub fn agent_message_system(
    //mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut query: Query<&mut Agent>,
) {
    
    let mut new_messages = AgentMessages::new();
    //println!("Checking messages");
    // Handle received messages
    while !agent_messages.messages.is_empty() {
        for message in &mut agent_messages.messages {
            
            let receiver_id = message.receiver_id;

            for mut agent in query.iter_mut() {
                if agent.get_status() != Status::Dead{
                    if agent.get_id() == receiver_id {
                        match message.message_type{
                            MessageType::Attack(damage) => {
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
                            MessageType::Reward(reward) => agent.add_reward(reward),
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
                            MessageType::Cooperate(_) => todo!(),
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

    // Clean up processed messages
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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

pub fn cleanup_system(
    mut commands: Commands,
    mut query_a: Query<&mut Agent>,
    mut query_m: Query<&mut Monster>,
) {
    // Handle received messages
    for agent in query_a.iter_mut() {
        // if agent.get_id() == 2 && agent.get_status() == Status::Dead{
        //     println!("Dead");
        // }
        if agent.get_energy() == 0 || agent.get_status() == Status::Dead{
            commands.entity(agent.get_entity()).despawn();
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
pub fn perform_action(
    mut query: Query<&mut Agent>,
    mut world: ResMut<World>,
    mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
) {
    
    for mut agent in query.iter_mut() {
        if agent.is_leader() && !agent.get_followers().is_empty() {
            let current_target = agent.get_target();
            match current_target {
                Target::Agent => {
                    match world.get_agent_position(agent.get_agent_target_id()) {
                        Ok(agent_position) => {
                            let (x, y) = agent_position;
                            agent.set_tile_target(Some((x as u32, y as u32)));
                        }
                        Err(MyError::AgentNotFound) => {
                            println!("Agent not found in system::perform_action line {}",284);
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
                                agent.set_status(Status::Working);
                            },
                            Target::Treasure => todo!(),
                            Target::None => todo!(),
                            Target::Tile => todo!(),
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
                            Target::Tile => todo!(),
                            _ => println!("Invalid Target in system::perform_action line {}",366),
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
                            _ => println!("Invalid Target in system::perform_action line {}",366)
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
                    Ok(it) => it,
                    Err(_) => println!("Invalid Target in system::perform_action line {}",383),
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