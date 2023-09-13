use bevy::prelude::*;
use camera::camera_drag_system;
use camera::CameraDragging;
mod tile;
mod components;
mod systems;
mod camera;
mod world;
mod npc;
mod debug;
mod behavior;
use entities::agent::Status;
use entities::agent::Target;
use mcst::NpcAction;
use world::World;
mod movement; 
mod mcst;
mod errors;
mod entities {
    pub mod monster;
    pub mod agent;
    pub mod treasure;
}
use crate::components::{Position, TileComponent, TreasureComponent};
use crate::entities::agent::Agent;
use crate::tile::TileType;
use crate::errors::MyError;

const START_AGENT_COUNT: usize = 5;

#[allow(dead_code)]
fn main() {
    // Begin building the Bevy app.
    App::build()
        // Set the window properties, such as title, width, and height.
        .insert_resource(WindowDescriptor {
            title: "Thesis".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        // Add default Bevy plugins to the app. This includes basic functionality like rendering, input handling, etc.
        .add_plugins(DefaultPlugins)
        // Insert a World resource that contains the game world's grid.
        .insert_resource(world::create_world())
        // Add a system that handles camera drag functionality.
        .add_system(camera_drag_system.system())
        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(setup.system())
        // Add a system that moves agents to a village.
        .add_startup_stage("post_startup", SystemStage::single(debug_system.system()))
        //.add_startup_system(npc::debug.system())
        
        // Insert a CameraDragging resource to track the camera dragging state.
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })

        // Insert AgentMessages resource with an empty vector.
        .insert_resource(AgentMessages {
            messages: Vec::new(),
        })
        
        // Add the agent message system to handle messages between agents.
        .add_system(agent_message_system.system())
        
        // Custom systems here
        .run();
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world: ResMut<World>,
) {
    
    // Load the individual textures
    let forest_texture = asset_server.load("textures/forest.png");
    let mountain_texture = asset_server.load("textures/mountain.png");
    let lake_texture = asset_server.load("textures/water.png");
    let village_texture = asset_server.load("textures/village.png");
    let dungeon_texture = asset_server.load("textures/dungeon.png");

    // Add the materials directly to the `materials` variable
    let forest_material = materials.add(forest_texture.into());
    let mountain_material = materials.add(mountain_texture.into());
    let lake_material = materials.add(lake_texture.into());
    let village_material = materials.add(village_texture.into());
    let dungeon_material = materials.add(dungeon_texture.into());

    for (y, column) in world.grid.iter_mut().enumerate() {
        for (x, tile) in column.iter_mut().enumerate() {
            let treasure = None;
            let material_handle = match tile.lock().unwrap().get_tile_type() {
                TileType::Forest => forest_material.clone(),
                TileType::Mountain => mountain_material.clone(),
                TileType::Lake => lake_material.clone(),
                TileType::Village => village_material.clone(),
                TileType::Dungeon => dungeon_material.clone(),
            };

            let sprite_bundle = SpriteBundle {
                material: material_handle,
                transform: Transform::from_xyz((x as f32) * 32.0, (y as f32) * 32.0, 0.0),
                sprite: Sprite::new(Vec2::new(32.0, 32.0)),
                ..Default::default()
            };

            let mut tile_entity = commands.spawn_bundle(sprite_bundle);
            tile_entity.insert(Position { x: x as i32, y: y as i32 });
            tile_entity.insert(TileComponent { tile_type: tile.lock().unwrap().get_tile_type().clone() });
            tile_entity.insert(Position { x: x as i32, y: y as i32 });

            if let Some(treasure) = treasure {
                tile_entity.insert(TreasureComponent { treasure });
            }

        }
    }


    // Calculate the center of the grid
    let grid_width = world.grid[0].len() as f32;
    let grid_height = world.grid.len() as f32;
    let half_grid_width = grid_width * 16.0;
    let half_grid_height = grid_height * 16.0;

    // Set up the 2D camera at the center of the grid
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(half_grid_width, half_grid_height, 1000.0));

    let mut villages: Vec<(usize, usize)> = Vec::new();
    for (y, column) in world.grid.iter().enumerate() {
        for (x, tile_mutex) in column.iter().enumerate() {
            let tile = tile_mutex.lock().unwrap();
            if tile.get_tile_type() == TileType::Village {
                villages.push((x, y));
            }
        }
    }
    
    for i in 0..START_AGENT_COUNT {
        let village = villages[i % villages.len()];
    
        let agent = Agent::new_agent(
            village.0 as f32,
            village.1 as f32,
            &mut commands,
            &mut materials,
            &asset_server,
        );
    

        // // Try to add the agent to the world
        if let Err(err) = world.add_agent(agent.clone(), &mut commands) {
            // Handle the error here, e.g. print an error message
            match err {
                MyError::TileNotFound => {
                    println!("Failed to add agent: Tile not found.");
                }
                // Handle other error cases if needed
                _ => {
                    println!("Failed to add agent: Unknown error.");
                }
            }
        } 
    }
}

fn debug(
    mut query: Query<&mut Agent>, 
    //world: ResMut<World>,
    //mut agent_messages: ResMut<AgentMessages>,
    //commands: &mut Commands,
) {
    // Query for all mutable Agent components
    for mut agent in query.iter_mut() {
        if agent.get_id() == 1 {
            // Found the desired agent by ID
            agent.set_agent_target_id(2);
            agent.set_target(entities::agent::Target::Agent);
            agent.set_action(mcst::NpcAction::Attack);
            //agent.perform_action(world, commands, agent_messages);

            // Print other agent properties as needed
        }
    }

}

fn debug_system(
    query: Query<&mut Agent>,
    // world: ResMut<World>,
    // agent_messages: ResMut<AgentMessages>,
    // mut commands: Commands,
) {
    debug(query, 
        // world, 
        // agent_messages, 
        // &mut commands
    );
    
}

////////////////////////////////////////////////////////////////


pub struct AgentMessages {
    messages: Vec<AgentMessage>,
}


fn agent_message_system(
    //mut commands: Commands,
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

pub enum MessageType{
    Attack(u8),
}

#[allow(dead_code)]
pub struct AgentMessage {
    sender_id: u32,
    receiver_id: u32,
    message_type: MessageType,
}


//Add error handling if the target is gone/dead
pub fn perform_action(
    mut query: Query<&mut Agent>,
    world: ResMut<World>,
    commands: &mut Commands,
    mut agent_messages: ResMut<AgentMessages>,
) -> Result<(), MyError> {
    
    for mut agent in query.iter_mut() {
        let current_target = agent.get_target();
        match current_target {
            Target::Agent => {
                match world.get_agent_position(agent.get_agent_target_id()) {
                    Ok(agent_position) => {
                        let (x, y) = agent_position;
                        agent.set_tile_target(Some((x as u32, y as u32)));
                    }
                    Err(MyError::AgentNotFound) => {
                        return Err(MyError::AgentNotFound);
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
                        return Err(MyError::MonsterNotFound);
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
                        return Err(MyError::TreasureNotFound);
                    }
                    _ => {} // Handle other errors if needed
                }
            }
            Target::None => {
                return Err(MyError::InvalidTarget);
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
                        Target::Monster => todo!(),
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
                NpcAction::Talk => todo!(),
                NpcAction::None => todo!(),
            }
            // Clear the action after performing it
            agent.set_status(Status::Idle) ;
            
            return Ok(()) // Return Ok to indicate success
        } else {
            // If the agent is not at the target position, initiate travel
            agent.travel(world.get_grid(), commands)?; 
            agent.set_status(Status::Moving);
            // Call the move_between_tiles function to move the agent to the next position in the path
            world.move_agent(agent.get_id(), x as usize, y as usize)?;
            return Ok(()) // Return Ok to indicate success
        }
    }
    return Ok(())
}