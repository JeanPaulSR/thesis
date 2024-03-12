use bevy::prelude::Commands;

use crate::entities::agent::Agent;
use crate::entities::monster::Monster;
use crate::entities::treasure::Treasure;
use crate::tests::simple_agent::SimpleAgent;
use crate::tile::TileType;
use crate::tile::Tile;
use crate::errors::MyError;
use std::collections::HashMap;
use bevy::ecs::schedule::ShouldRun;
use std::sync::{Arc, Mutex};

// Primary world constructor with a default map
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
    
    let mut world = World::new();
    world.grid = Vec::new();

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

#[derive(Clone)]
pub struct World {
    pub agents: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub monsters: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub treasures: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub grid: Vec<Vec<Arc<Mutex<Tile>>>>, 
}

impl World {
    
    //Construct a empty world
    pub fn new() -> Self {
        let agents = Arc::new(Mutex::new(HashMap::new()));
        let monsters = Arc::new(Mutex::new(HashMap::new()));
        let treasures = Arc::new(Mutex::new(HashMap::new()));
    
        let mut grid: Vec<Vec<Arc<Mutex<Tile>>>> = Vec::new();
        for _ in 0..30 {
            let row = (0..30)
                .map(|_| Arc::new(Mutex::new(Tile::new(TileType::Forest))))
                .collect();
            grid.push(row);
        }
        
        World {
            agents,
            monsters,
            treasures,
            grid,
        }
    }

    // Function to check if the position (x, y) is within the grid's bounds
    fn is_valid_position(&self, x: usize, y: usize) -> Result<(), MyError> {
        if let Some(row) = self.grid.get(y) {
            if let Some(_) = row.get(x) {
                return Ok(());
            }
        }
        println!("Asked for position ({}, {}), was not found.", x, y);
        Err(MyError::PositionError)
    }

    //Creates a copy of the world
    pub fn copy(&self) -> Self {
        let agents = Arc::new(Mutex::new(self.agents.lock().unwrap().clone()));
        let monsters = Arc::new(Mutex::new(self.monsters.lock().unwrap().clone()));
        let treasures = Arc::new(Mutex::new(self.treasures.lock().unwrap().clone()));

        let mut new_grid = Vec::with_capacity(self.grid.len());
        for row in &self.grid {
            let mut new_row = Vec::with_capacity(row.len());
            for tile_arc_mutex in row {
                let tile_mutex = Mutex::new(tile_arc_mutex.lock().unwrap().clone());
                let arc_tile = Arc::new(tile_mutex);
                new_row.push(arc_tile);
            }
            new_grid.push(new_row);
        }

        World {
            agents,
            monsters,
            treasures,
            grid: new_grid,
        }
    }

// ___________.__.__          
// \__    ___/|__|  |   ____  
//   |    |   |  |  | _/ __ \ 
//   |    |   |  |  |_\  ___/ 
//   |____|   |__|____/\____>


    // Function to get the Tile at position (x, y)
    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Arc<Mutex<Tile>>, MyError> {
        match self.is_valid_position(x, y) {
            Ok(_) => {
                let grid_row = &self.grid[y];
                match grid_row.get(x) {
                    Some(tile) => Ok(tile),
                    None => Err(MyError::TileNotFound),
                }
            }
            Err(_) => Err(MyError::TileNotFound),
        }
    }
    
    // Function to get a mutable reference to the Tile at position (x, y)
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Result<Option<&mut Arc<Mutex<Tile>>>, MyError> {
        match self.is_valid_position(x, y) {
            Ok(_) => {
                let grid_row = &mut self.grid[y];
                Ok(grid_row.get_mut(x))
            }
            Err(_) => Err(MyError::TileNotFound),
        }
    }

    // Function to get the TileType at position (x, y)
    pub fn get_tile_type(&self, x: usize, y: usize) -> Result<Option<TileType>, MyError> {
        self.is_valid_position(x, y)?;
    
        match self.get_tile(x, y) {
            Ok(tile) => {
                let tile_lock = tile.lock().unwrap();
                Ok(Some(tile_lock.get_tile_type().clone()))
            }
            Err(_) => Err(MyError::TileNotFound),
        }
    }

    // Function to get the position of an agent using their id
    pub fn get_agent_position(&self, agent_id: u32) -> Result<(usize, usize), MyError> {
        match self.agents.lock().unwrap().get(&agent_id) {
            Some(position) => Ok(*position),
            None => Err(MyError::AgentNotFound),
        }
    }
    
    // Function to get the position of a monster using their id
    pub fn get_monster_position(&self, monster_id: u32) -> Result<(usize, usize), MyError> {
        match self.monsters.lock().unwrap().get(&monster_id) {
            Some(position) => Ok(*position),
            None => Err(MyError::MonsterNotFound),
        }
    }
    
    // Function to get the position of a treasure using its id
    pub fn get_treasure_position(&self, treasure_id: u32) -> Result<(usize, usize), MyError> {
        match self.treasures.lock().unwrap().get(&treasure_id) {
            Some(position) => Ok(*position),
            None => Err(MyError::TreasureNotFound),
        }
    }
    
    // Function to return a clone of the grid
    pub fn get_grid(&self) -> Vec<Vec<Tile>> {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile_lock| tile_lock.lock().unwrap().clone())
                    .collect()
            })
            .collect()
    }
//    _____                         __   
//    /  _  \    ____   ____   _____/  |_ 
//   /  /_\  \  / ___\_/ __ \ /    \   __\
//  /    |    \/ /_/  >  ___/|   |  \  |  
//  \____|__  /\___  / \___  >___|  /__|  
//          \//_____/      \/     \/      
 
    // Function to add an agent to the world and its current tile
    pub fn add_agent(&mut self, agent: Agent, commands: &mut Commands,) -> Result<(), MyError> {
        let (x, y) = agent.get_position();
        self.is_valid_position(x as usize, y as usize)?;

        let mut agents_vector = self.agents.lock().unwrap();
        agents_vector.insert(agent.get_id(), (x as usize, y as usize));
        let entity = agent.get_entity();
        commands.entity(entity).insert(agent);

        Ok(())
    }

    // Function to add an agent to the world and its current tile
    pub fn add_simple_agent(&mut self, agent: SimpleAgent,) -> Result<(), MyError> {
        let (x, y) = agent.get_position();

        self.is_valid_position(x as usize, y as usize)?;
        let mut agents = self.agents.lock().unwrap();
        agents.insert(agent.get_id(), (x as usize, y as usize));
        
        Ok(())
    }

    // Function to remove an agent from the world and its tile
    pub fn remove_agent(&mut self, agent_id: u32) -> Result<(), MyError> {
        let mut agents_lock = self.agents.lock().unwrap();
        if let Some(_) = agents_lock.get(&agent_id) {
            agents_lock.remove(&agent_id);
            return Ok(());
        }
        Err(MyError::AgentNotFound)
    }
    
    // Function to print agents' positions
    pub fn print_agents(&self) {
        let agents_positions = self.agents.lock().unwrap();

        println!("Agents:");
        for (id, (pos_x, pos_y)) in agents_positions.iter() {
            println!("Agent ID: {}, Position: ({}, {})", id, pos_x, pos_y);
        }
    }

    // Function to move the agent
    pub fn move_agent(
        &self,
        agent_id: u32,
        pos_y2: usize,
        pos_x2: usize,
    ) -> Result<(), MyError> {
        let mut agents_positions = self.agents.lock().unwrap();
        if let Some((pos_y1, pos_x1)) = agents_positions.get_mut(&agent_id) {
            *pos_x1 = pos_x2;
            *pos_y1 = pos_y2;
            
            println!("Agent ID: {}, ({} , {})", agent_id, pos_y2, pos_x2);
            Ok(())
        } else {
            Err(MyError::AgentNotFound)
        }
    }


    
//     _____                          __                
//    /     \   ____   ____   _______/  |_  ___________ 
//   /  \ /  \ /  _ \ /    \ /  ___/\   __\/ __ \_  __ \
//  /    Y    (  <_> )   |  \\___ \  |  | \  ___/|  | \/
//  \____|__  /\____/|___|  /____  > |__|  \___  >__|   
//          \/            \/     \/            \/       
 
    // Function to add a monster to the world and its current tile
    pub fn add_monster(&mut self, monster: Monster) -> Result<(), MyError> {
        let (x, y) = monster.get_position();
        self.is_valid_position(x as usize, y as usize)?;

        let mut monsters = self.monsters.lock().unwrap();
        monsters.insert(monster.get_id(), (x as usize, y as usize));
        Ok(())
    }

    // Function to remove a monster from the world and its tile
    pub fn remove_monster(&mut self, monster_id: u32) -> Result<(), MyError> {
        let mut monsters_lock = self.monsters.lock().unwrap();
        if let Some(_) = monsters_lock.get(&monster_id) {
            monsters_lock.remove(&monster_id);
            return Ok(());
        }
        Err(MyError::MonsterNotFound)
    }

// ___________                                                  
// \__    ___/______   ____ _____    ________ _________   ____  
//   |    |  \_  __ \_/ __ \\__  \  /  ___/  |  \_  __ \_/ __ \ 
//   |    |   |  | \/\  ___/ / __ \_\___ \|  |  /|  | \/\  ___/ 
//   |____|   |__|    \___  >____  /____  >____/ |__|    \___  >
//                        \/     \/     \/                   \/ 

    // Function to add a treasure to the world and its current tile
    pub fn add_treasure(&mut self, treasure: Treasure) -> Result<(), MyError> {
        let (x, y) = treasure.get_position();
        self.is_valid_position(x as usize, y as usize)?;
        let mut treasures = self.treasures.lock().unwrap();
        treasures.insert(treasure.get_id(), (x as usize, y as usize));
        Ok(())
    }

    // Function to remove a treasure from the world and its tile
    pub fn remove_treasure(&mut self, treasure_id: u32) -> Result<(), MyError> {
        let mut treasures_lock = self.treasures.lock().unwrap();
        if let Some(_) = treasures_lock.get(&treasure_id) {
            treasures_lock.remove(&treasure_id);
            return Ok(());
        }
        Err(MyError::TreasureNotFound)
    }

}
