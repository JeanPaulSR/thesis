use crate::entities::monster::Monster;
use crate::entities::agent::Agent;
use crate::entities::treasure::Treasure;
use crate::errors::MyError;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    pub agents: Arc<Mutex<Vec<Agent>>>,
    monsters: Arc<Mutex<Vec<Monster>>>,
    treasures: Arc<Mutex<Vec<Treasure>>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Forest,
    Mountain,
    Lake,
    Village,
    Dungeon,
}

impl Tile {
    // Function to create a new instance of Tile with the specified tile type
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            agents: Arc::new(Mutex::new(Vec::new())),
            monsters: Arc::new(Mutex::new(Vec::new())),
            treasures: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Function to get the type of the current tile
    pub fn get_tile_type(&self) -> TileType {
        self.tile_type
    }

    // Function to update the type of the current tile
    pub fn update_tile_type(&mut self, new_tile_type: TileType) {
        self.tile_type = new_tile_type;
    }

    
//    _____                         __   
//    /  _  \    ____   ____   _____/  |_ 
//   /  /_\  \  / ___\_/ __ \ /    \   __\
//  /    |    \/ /_/  >  ___/|   |  \  |  
//  \____|__  /\___  / \___  >___|  /__|  
//          \//_____/      \/     \/      
 
    // Function to add an agent to the tile
    pub fn add_agent(&self, agent: Agent) {
        self.agents.lock().unwrap().push(agent);
    }

    // Function to remove an agent from the tile
    pub fn remove_agent(&mut self, agent_id: u32) -> Result<Agent, MyError> {
        let mut agents = self.agents.lock().unwrap();
        // Check if the agent is found in the tile's agents vector
        if let Some(index) = agents.iter().position(|a| a.id == agent_id) {
            // Agent found, remove it from the tile's agents vector
            let removed_agent = agents.remove(index);
            Ok(removed_agent)
        } else {
            // Agent not found in the tile's agents vector, return an error
            Err(MyError::AgentNotFound)
        }
    }
    
    pub fn get_agents(&self) -> MutexGuard<Vec<Agent>> {
        self.agents.lock().unwrap()
    }
//     _____                          __                
//    /     \   ____   ____   _______/  |_  ___________ 
//   /  \ /  \ /  _ \ /    \ /  ___/\   __\/ __ \_  __ \
//  /    Y    (  <_> )   |  \\___ \  |  | \  ___/|  | \/
//  \____|__  /\____/|___|  /____  > |__|  \___  >__|   
//          \/            \/     \/            \/       
 
    // Function to add a monster to the tile
    pub fn add_monster(&self, monster: Monster) {
        self.monsters.lock().unwrap().push(monster);
    }

    // Function to remove a monster from the tile
    pub fn remove_monster(&mut self, monster_id: u32) -> Result<Monster, MyError> {
        let mut monsters = self.monsters.lock().unwrap();
        // Check if the monster is found in the tile's monsters vector
        if let Some(index) = monsters.iter().position(|m| m.id == monster_id) {
            // Monster found, remove it from the tile's monsters vector
            let removed_monster = monsters.remove(index);
            Ok(removed_monster)
        } else {
            // Monster not found in the tile's monsters vector, return an error
            Err(MyError::MonsterNotFound)
        }
    }

    // // Function to get a mutable reference to a monster by ID
    // pub fn get_monster(&mut self, id: u32) -> Result<&mut Monster, MyError> {
    //     // Lock the monsters vector to ensure thread safety
    //     if let Some(monster) = self.monsters.lock().unwrap().iter_mut().find(|m| m.id == id) {
    //         Ok(monster)
    //     } else {
    //         // Monster not found in the monsters vector, return an error
    //         println!("Monster {} not found in tile.", id);
    //         Err(MyError::MonsterNotFound)
    //     }
    // }
// ___________                                                  
// \__    ___/______   ____ _____    ________ _________   ____  
//   |    |  \_  __ \_/ __ \\__  \  /  ___/  |  \_  __ \_/ __ \ 
//   |    |   |  | \/\  ___/ / __ \_\___ \|  |  /|  | \/\  ___/ 
//   |____|   |__|    \___  >____  /____  >____/ |__|    \___  >
//                        \/     \/     \/                   \/ 


    // Function to add a treasure to the tile
    pub fn add_treasure(&self, treasure: Treasure) {
        self.treasures.lock().unwrap().push(treasure);
    }

    // Function to remove a treasure from the tile
    pub fn remove_treasure(&mut self, treasure_id: u32) -> Result<Treasure, MyError> {
        let mut treasures = self.treasures.lock().unwrap();
        // Check if the treasure is found in the tile's treasures vector
        if let Some(index) = treasures.iter().position(|t| t.id == treasure_id) {
            // Treasure found, remove it from the tile's treasures vector
            let removed_treasure = treasures.remove(index);
            Ok(removed_treasure)
        } else {
            // Treasure not found in the tile's treasures vector, return an error
            Err(MyError::TreasureNotFound)
        }
    }

    // // Function to get a mutable reference to a treasure by ID
    // pub fn get_treasure(&mut self, id: u32) -> Result<&mut Treasure, MyError> {
    //     if let Some(treasure) = self.treasures.lock().unwrap().iter_mut().find(|t| t.id == id) {
    //         Ok(treasure)
    //     } else {
    //         println!("Treasure {} not found in tile.", id);
    //         Err(MyError::TreasureNotFound)
    //     }
    // }
}


