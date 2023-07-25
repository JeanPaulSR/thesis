use bevy::prelude::*;
use crate::World;
use crate::movement::find_path;
use crate::tile::TileType;
use crate::entities::monster::Monster;
use crate::entities::agent::Agent;

pub fn debug(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut world: ResMut<World>,
) {
    let mut villages: Vec<(f32, f32)> = Vec::new();
    for (y, row) in world.grid.iter().rev().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
                if let TileType::Village = tile_type {
                    villages.push((x as f32, y as f32));
                }
            }
    }
    let n = 10;
    for i in 0..n {
        let tuple = villages[i % villages.len()];
        let agent = Agent::new_agent(tuple.0 * 32.0, tuple.1 * 32.0, &mut commands, &mut materials, &asset_server);
        world.agents.push(agent);
    }
    // Spawn an agent using the `new_random` function
    let  agent = Agent::new_agent(0.0 , 0.0 , &mut commands, &mut materials, &asset_server);
    let _agent2 = Agent::new_agent(1.0  , 1.0  , &mut commands, &mut materials, &asset_server);
    let _agent3 = Agent::new_agent(2.0  , 2.0  , &mut commands, &mut materials, &asset_server);
    world.add_agent(_agent2);
    //agent.travel(2.0, 3.0, &mut commands);
    //update_agent_transform(&mut agent, new_transform, &mut commands);
    world.add_agent(agent.clone());
    let mut monster = Monster::new_monster(3.0 * 32.0, 3.0 * 32.0, &mut commands, &mut materials, &asset_server);
    monster.travel(7.0, 1.0, &mut commands);
    let start_pos = (0, 0);
    let end_pos = (5, 5);

    if let Some(tile_type) = world.get(1, 1) {
        println!("TileType at position (1, 1): {:?}", tile_type);
    } else {
        println!("Invalid position (5, 5)");
    }
    
    if let Some(path) = find_path(&world, start_pos, end_pos) {
        println!("Found path: {:?}", path);
    } else {
        println!("Failed to find path.");
    }   
    let nearby = world.find_agents_within_distance(&agent.clone(), 3.0);
    for a in nearby {
        a.print();
    }
}

