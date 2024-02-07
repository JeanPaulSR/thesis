use std::sync::{Mutex, Arc};

use crate::{world::World, tile::{Tile, TileType}};




#[cfg(test)]
mod tests {
    use crate::{entities::agent::Genes, mcst::NpcAction, mcst::{MCTSNode, self}};


    #[test]
    fn test_calculate_action_score() {
        let genes = Genes {
            greed: 0.8,
            aggression: 0.6,
            social: 0.4,
            self_preservation: 0.2,
            vision: 0.1,
        };
        let attack_score = mcst::calculate_action_score(&genes, NpcAction::Attack);
        assert_eq!(attack_score, 0.6);
    }

    // #[test]
    // fn test_calculate_uct_score() {
    //     let child = MCTSNode {
    //         state_info: todo!()/* Initialize with your GameState */,
    //         action: Some(NpcAction::Attack),
    //         visits: 10,
    //         total_reward: 30,
    //         children: vec![],
    //     };
    //     let parent = MCTSNode {
    //         state_info: todo!()/* Initialize with your GameState */,
    //         action: Some(NpcAction::Steal),
    //         visits: 20,
    //         total_reward: 60,
    //         children: vec![child],
    //     };
    //     let c = 2_f64.sqrt();
    //     let uct_score = calculate_uct_score(&child, &parent, c);
    //     // Define your expected UCT score and compare it with uct_score
    //     assert_eq!(uct_score, 5);
    // }

    // #[test]
    // fn test_select() {
    //     let genes = Genes {
    //         greed: 0.8,
    //         aggression: 0.6,
    //         social: 0.4,
    //         self_preservation: 0.2,
    //         vision: 0.1,
    //     };
    //     let root = MCTSNode {
    //         state_info: todo!()/* Initialize with your GameState */,
    //         action: Some(NpcAction::Steal),
    //         visits: 5,
    //         total_reward: 15,
    //         children: vec![
    //             MCTSNode {
    //                 state_info: todo!()/* Initialize with your GameState */,
    //                 action: Some(NpcAction::Attack),
    //                 visits: 10,
    //                 total_reward: 30,
    //                 children: vec![],
    //             },
    //             // Add more child nodes as needed
    //         ],
    //     };
    //     let selected_index = select(&root, &genes);
    //     // Define your expected selected_index and compare it with selected_index
    //     assert_eq!(selected_index, /* Your expected value */);
    // }
}



pub fn create_world() -> World {
    let map_data: Vec<&str> = vec![
        "vmfffffffffffffffffm",
        "fmfffffffffffffffflm",
        "fffffffvfffffffffllm",
        "ffffffffffffffffllfm",
        "fffffffffffffffllffm",
        "ffffffffffffffllfffm",
        "fffffffffffffllffffm",
        "ffffffffffffffmffllm",
        "ffffmmfffffffffmlllm",
        "ffffmfffffffffmllllm",
        "fffffmffffffffffmllm",
        "fffffffmfffffmmmmllm",
        "mmmmmmmmmmmfmmmmlllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "ffffffllfffffffflllm",
        "ffffffffvffffffflllm",
        "fmfffffffffffffflllm",
        "fmfffffffffffffflllm",
        "fmfffffffffffffflllm",
    ];
    
    let mut world = World::new(); // Create the initial World structure

    // Replace the entire grid with the appropriate tile types
    world.grid = Vec::new(); // Clear the existing grid

    for row_str in map_data.iter() {
        let mut row = Vec::new();
        for c in row_str.chars() {
            let tile_type = match c {
                'm' => TileType::Mountain,
                'l' => TileType::Lake,
                'v' => TileType::Village,
                'd' => TileType::Dungeon,
                'f' => TileType::Forest,
                _ => panic!("Invalid tile character: {}", c),
            };

            let tile = Arc::new(Mutex::new(Tile::new(tile_type)));
            row.push(tile);
        }
        world.grid.push(row);
    }

    world
}


const START_AGENT_COUNT: usize = 5;

//pub fn setup() -> GameState{
//    
//    let mut world = create_world();
//
//    let mut villages: Vec<(usize, usize)> = Vec::new();
//    for (y, column) in world.grid.iter().enumerate() {
//        for (x, tile_mutex) in column.iter().enumerate() {
//            let tile = tile_mutex.lock().unwrap();
//            if tile.get_tile_type() == TileType::Village {
//                villages.push((x, y));
//            }
//        }
//    }
//    
//    let mut agents: Vec<SimpleAgent> = Vec::new();
//    let mut monsters: Vec<SimpleMonster> = Vec::new();
//    let mut treasures: Vec<SimpleTreasure> = Vec::new();
//
//    for i in 0..START_AGENT_COUNT {
//        let village = villages[i % villages.len()];
//    
//        let agent = SimpleAgent::new(
//            village.0 as u32,
//            village.1 as u32,
//        );
//
//        agents.push(agent.clone());
//    
//        if let Err(err) = world.add_simple_agent(agent.clone()) {
//            // Handle the error here, e.g. print an error message
//            match err {
//                MyError::TileNotFound => {
//                    println!("Failed to add agent: Tile not found.");
//                }
//                // Handle other error cases if needed
//                _ => {
//                    println!("Failed to add agent: Unknown error.");
//                }
//            }
//        } 
//    }
//
//    GameState::create_gamestate_simple(agents, monsters, treasures, world.clone())
//}