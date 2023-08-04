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
        for (x, tile) in row.iter().enumerate() {
            if tile.get_tile_type() == TileType::Village {
                villages.push((x as f32, y as f32));
            }
        }
    }

    let n = 10;
    for i in 0..n {
        let tuple = villages[i % villages.len()];
        let agent = Agent::new_agent(tuple.0 * 32.0, tuple.1 * 32.0, &mut commands, &mut materials, &asset_server);

        // Save the agent into its corresponding tile
        if let Some(tile) = world.grid.get_mut(tuple.1 as usize).and_then(|row| row.get_mut(tuple.0 as usize)) {
            tile.add_agent(agent.clone());
        }

        world.agents.push(agent.clone());
    }

    let agent = Agent::new_agent(0.0, 0.0, &mut commands, &mut materials, &asset_server);
    let _agent2 = Agent::new_agent(1.0, 1.0, &mut commands, &mut materials, &asset_server);
    let _agent3 = Agent::new_agent(2.0, 2.0, &mut commands, &mut materials, &asset_server);

    world.add_agent(_agent2).ok();
    world.add_agent(agent.clone()).ok();

    let mut monster = Monster::new_monster(3.0 * 32.0, 3.0 * 32.0, &mut commands, &mut materials, &asset_server);
    monster.travel(7.0, 1.0, &mut commands);

    let start_pos = (0, 0);
    let end_pos = (5, 5);

    // if let Ok(Some(_)) = world.get_tile_type(1000, 1000) {
    //     println!("TileType at position (1, 1): Found tile"); 
    // } else {
    //     println!("Invalid position (5, 5)");
    // }

    // if let Some(path) = find_path(&world, start_pos, end_pos) {
    //     println!("Found path: {:?}", path);
    // } else {
    //     println!("Failed to find path.");
    // }

    // let nearby = world.find_agents_within_distance(&agent.clone(), 3.0);
    // for a in nearby {
    //     a.print();
    // }
}