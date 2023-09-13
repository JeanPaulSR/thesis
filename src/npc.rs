// use bevy::prelude::*;
// use crate::World;
// use crate::errors::MyError;
// use crate::mcst::NpcAction;
// use crate::movement::find_path;
// use crate::tile::TileType;
// use crate::entities::monster::Monster;
// use crate::entities::agent::{Agent, AgentAction, Status};
// use bevy::prelude::Commands;

// #[allow(dead_code)]
// pub fn debug(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     asset_server: Res<AssetServer>,
//     mut world: ResMut<World>,
// ) {
//     let mut villages: Vec<(usize, usize)> = Vec::new();
//     for (y, row) in world.grid.iter().enumerate() {
//         for (x, tile_mutex) in row.iter().enumerate() {
//             let tile = tile_mutex.lock().unwrap();
//             if tile.get_tile_type() == TileType::Village {
//                 villages.push((x, y));
//             }
//         }
//     }

//     let n = 10;
//     for i in 0..n {
//         let village = villages[i % villages.len()];
//         let agent = Agent::new_agent(
//             village.0 as f32,
//             village.1 as f32,
//             &mut commands,
//             &mut materials,
//             &asset_server,
//         );

//         if let Ok(Some(tile_mutex)) = world.get_tile_mut(village.0, village.1) {
//             let tile = tile_mutex.lock().unwrap();
//             tile.add_agent(agent.clone());
//         }

//         // Try to add the agent to the world
//         if let Err(err) = world.add_agent(agent.clone()) {
//             // Handle the error here, e.g. print an error message
//             match err {
//                 MyError::TileNotFound => {
//                     println!("Failed to add agent: Tile not found.");
//                 }
//                 // Handle other error cases if needed
//                 _ => {
//                     println!("Failed to add agent: Unknown error.");
//                 }
//             }
//         } 
//     }

    
//     let start_pos = (0, 0);
//     let end_pos = (4, 0);
//     let agent = Agent::new_agent(0.0, 0.0, &mut commands, &mut materials, &asset_server);
//     let _agent2 = Agent::new_agent(0.0, 5.0, &mut commands, &mut materials, &asset_server);
    
//     world.add_agent(_agent2.clone()).ok();
//     let _agent3 = Agent::new_agent(2.0, 2.0, &mut commands, &mut materials, &asset_server);

//     world.print_agents();
//     match world.move_agent(3, 7, 17, &mut commands) {
//         Ok(()) => {
//             // Move successful
//             // Do something here if needed
//         }
//         Err(err) => {
//             // Handle the error
//             eprintln!("Could not move agent. Error: {:?}", err);
//         }
//     }
    
//     world.print_agents();
//     match world.move_agent(3, 8, 18, &mut commands) {
//         Ok(()) => {
//             // Move successful
//             // Do something here if needed
//         }
//         Err(err) => {
//             // Handle the error
//             eprintln!("Could not move agent. Error: {:?}", err);
//         }
//     }

//     world.add_agent(agent.clone()).ok();

//     let mut monster = Monster::new_monster(3.0 * 32.0, 3.0 * 32.0, &mut commands, &mut materials, &asset_server);
//     monster.travel(7.0, 1.0, &mut commands);

//     if let Ok(_) = world.get_tile(0, 1) {
//         println!("Tile at Position: ({}, {}), TileType: {:?}", 0, 1, world.get_tile_type(0, 1));
//     } else {
//         println!("Invalid position (0, 1)");
//     }

//     if let Some(path) = find_path(&world, start_pos, end_pos) {
//         println!("Found path: {:?}", path);
//     } else {
//         println!("Failed to find path.");
//     }
//     world.set_agent_action(1, AgentAction::SetAction(NpcAction::Attack));


//     //Set agent 1 action to attack
//     if let Ok(status) = world.get_agent_status(1) {
//         // Agent status retrieved successfully
//         println!("Agent status: {:?}", status);
//     } else {
//         // Agent not found error
//         eprintln!("Agent not found while attempting to get action");
//     }
//     if let Ok(action) = world.get_agent_action(1) {
//         // Agent status retrieved successfully
//         println!("Agent status: {:?}", action);
//     } else {
//                 // Agent not found error
//                 eprintln!("Agent not found while attempting to get status");
//     }

//     let mut status = match world.get_agent_status(1) {
//         Ok(status) => status,
//         Err(err) => {
//             // Handle the error here, e.g., print an error message
//             println!("Error: {:?}", err);
//             // You can return a default or some other value here if needed
//             Status::Idle
//         }
//     };
//     while status == Status::Moving {
        
//         status = match world.get_agent_status(1) {
//             Ok(status) => status,
//             Err(err) => {
//                 // Handle the error here, e.g., print an error message
//                 println!("Error: {:?}", err);
//                 // You can return a default or some other value here if needed
//                 Status::Idle
//             }
//         };
//     }
//     world.set_agent_action(1, AgentAction::Print);

// }