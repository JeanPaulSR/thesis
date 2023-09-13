use bevy::prelude::*;
use crate::entities::agent::{Agent, Status, Target};
use crate::components::Position;
use crate::debug::debug;
use crate::errors::MyError;
use crate::mcst::NpcAction;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use crate::World;
    

pub fn debug_system(_time: Res<Time>) {
    // Call the debug function with a command string
    debug("test_movement");
}

pub fn _get_tiles_in_range2(
    x: i32,
    y: i32,
    vision_range: i32,
    world: &Res<World>,
) -> Vec<Position> {
    let mut tiles_in_range = HashSet::new();
    let max_distance = vision_range as f32 * 32.0;
    for (ty, row) in world.grid.iter().enumerate() {
        for (tx, _) in row.iter().enumerate() {
            let distance = ((tx as i32 - x).pow(2) + (ty as i32 - y).pow(2)) as f32;
            if distance <= max_distance.powi(2) {
                tiles_in_range.insert(Position { x: tx as i32, y: ty as i32 });
            }
        }
    }
    tiles_in_range.into_iter().collect()
}

pub struct AgentMessages {
    pub(crate) messages: Vec<AgentMessage>,
}

#[derive()]
pub enum MessageType{
    Attack(u8),
}

#[allow(dead_code)]
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

pub fn agent_message_system(
    mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
    mut query: Query<&mut Agent>,
) {
    // Handle received messages
    for message in &agent_messages.messages {
        let receiver_id = message.receiver_id;

        for mut agent in query.iter_mut() {
            if agent.get_id() == receiver_id {
                match message.message_type{
                    MessageType::Attack(damage) => agent.remove_energy(damage),
                }
            }
        }
    }

    // Clean up processed messages
    agent_messages.messages.clear();
}

#[allow(unused_variables)]
//Add error handling if the target is gone/dead
pub fn perform_action(
    mut query: Query<&mut Agent>,
    mut world: ResMut<World>,
    mut commands: Commands,
    mut agent_messages: ResMut<AgentMessages>,
) {
    
    for mut agent in query.iter_mut() {
        
        // thread::sleep(Duration::from_millis(100));
        println!("ID: {}", agent.get_id());
        let current_target = agent.get_target();
        match current_target {
            Target::Agent => {
                match world.get_agent_position(agent.get_agent_target_id()) {
                    Ok(agent_position) => {
                        let (x, y) = agent_position;
                        agent.set_tile_target(Some((x as u32, y as u32)));
                    }
                    Err(MyError::AgentNotFound) => {
                        println!("Agent not found.");
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
                        println!("Monster not found.");
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
                println!("Invalid Target");
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
                            agent.send_message(
                                id,
                                MessageType::Attack(10),
                                &mut agent_messages,
                            )
                        },
                        Target::Monster => {
                            let id = agent.get_monster_target_id();

                        },
                        Target::None => todo!(),
                        Target::Tile => todo!(),
                        Target::Treasure => todo!(),
                    }
                    // Attack formula
                    // Agents have 3 lives
                    // Every time an agent attacks something they lose a life
                }
                NpcAction::Steal => {
                    // Logic for moving to a treasure
                }
                NpcAction::Rest => {
                    // Logic for moving to a monster
                }
                NpcAction::Talk => {
                    // Logic for moving to a monster
                }
                NpcAction::None => {
                    // Logic for moving to a monster
                }
            }
            // Clear the action after performing it
            agent.set_status(Status::Idle) ;
        } else {
            // If the agent is not at the target position, initiate travel
            match agent.travel(world.get_grid(), &mut commands) {
                Ok(it) => it,
                Err(_) => println!("Invalid Target"),
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