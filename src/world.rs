use bevy::asset::AssetServer;
use bevy::asset::Assets;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::prelude::Commands;
use bevy::sprite::ColorMaterial;
use rand::thread_rng;
use rand::prelude::SliceRandom;

use crate::entities::agent::Agent;
use crate::entities::monster::Monster;
use crate::entities::treasure::Treasure;
use crate::tests::simple_agent::SimpleAgent;
use crate::tile::TileType;
use crate::tile::Tile;
use crate::errors::MyError;
use std::collections::HashMap;

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

    

    pub fn find_valid_monster_spawns(&self) -> Vec<Vec<(u32, u32)>> {
        let world = &self;
        let mut valid_spawns: Vec<Vec<(u32, u32)>> = Vec::new();

        for (row_index, row) in world.grid.iter().enumerate() {
            let mut valid_row_spawns: Vec<(u32, u32)> = Vec::new();

            for (col_index, tile_mutex) in row.iter().enumerate() {
                let tile = tile_mutex.lock().unwrap();
                let tile_type = tile.get_tile_type();

                // Check conditions for valid spawn
                let mut is_valid_spawn = true;

                // Check if within 5 tiles of an agent
                for agent_pos in world.agents.lock().unwrap().values() {
                    let agent_row = agent_pos.0;
                    let agent_col = agent_pos.1;
                    let distance_squared = ((row_index as isize - agent_row as isize).pow(2)
                        + (col_index as isize - agent_col as isize).pow(2)) as usize;

                    if distance_squared <= 25 {
                        is_valid_spawn = false;
                        break;
                    }
                }

                // Check if atop another monster
                if world.monsters.lock().unwrap().values().any(|pos| pos == &(row_index, col_index)) {
                    is_valid_spawn = false;
                }

                // Check if within 10 tiles of a village
                match tile_type {
                    TileType::Village => {
                        is_valid_spawn = false;
                        break;
                    }
                    _ => {}
                }

                // Check if on top of a Mountain or Lake
                match tile_type {
                    TileType::Mountain | TileType::Lake => {
                        is_valid_spawn = false;
                        break;
                    }
                    _ => {}
                }

                if is_valid_spawn {
                    valid_row_spawns.push((row_index as u32, col_index as u32));
                }
            }

            if !valid_row_spawns.is_empty() {
                valid_spawns.push(valid_row_spawns);
            }
        }

        valid_spawns
    }


    // Function to check if the position (x, y) is within the grid's bounds
    fn is_valid_position_bool(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (y as usize) < self.grid.len() && (x as usize) < self.grid[y as usize].len()
    }

    pub fn set_valid_spawns(&self) {
        let world = &self;
        for (row_index, row) in world.grid.iter().enumerate() {
            for (col_index, tile_mutex) in row.iter().enumerate() {
                let mut tile = tile_mutex.lock().unwrap();
                if tile.is_monster_spawn() {
                    let tile_type = tile.get_tile_type().clone();
                    match tile_type {
                        TileType::Village => {
                            tile.set_monster_spawn(false);
                            for i in -5..=5 {
                                for j in -5..=5 {
                                    if j != 0 || i != 0 {
                                        let new_row = row_index as isize + i;
                                        let new_col = col_index as isize + j;
                                        if world.is_valid_position_bool(new_col, new_row) {
                                            let mut new_tile = world.grid[new_row as usize][new_col as usize].lock().unwrap();
                                            new_tile.set_monster_spawn(false);
                                        }
                                    }
                                }
                            }
                        }
                        TileType::Mountain => {
                            tile.set_monster_spawn(false);
                        }
                        TileType::Lake =>{
                            tile.set_monster_spawn(false);
                        } 
                        _ => {}
                    }
                }
            }
        }
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

    // Function to spawn agents based on START_AGENT_COUNT
    // Function to populate agents in villages up to a given count
    pub fn populate_agents(&mut self, start_agent_count: usize, commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>, asset_server: &Res<AssetServer>) {
        let mut villages: Vec<(usize, usize)> = Vec::new();
        for (y, column) in self.grid.iter().enumerate() {
            for (x, tile_mutex) in column.iter().enumerate() {
                let tile = tile_mutex.lock().unwrap();
                if tile.get_tile_type() == TileType::Village {
                    villages.push((x, y));
                }
            }
        }

        for i in 0..start_agent_count {
            let village = villages[i % villages.len()];

            let agent = Agent::new_agent(
                village.0 as f32,
                village.1 as f32,
                commands,
                materials,
                asset_server,
            );

            // Try to add the agent to the world
            if let Err(err) = self.add_agent(agent.clone(), commands) {
                match err {
                    MyError::TileNotFound => {
                        println!("Failed to add agent: Tile not found.");
                    }
                    _ => {
                        println!("Failed to add agent: Unknown error.");
                    }
                }
            } 
        }
    }

    
//     _____                          __                
//    /     \   ____   ____   _______/  |_  ___________ 
//   /  \ /  \ /  _ \ /    \ /  ___/\   __\/ __ \_  __ \
//  /    Y    (  <_> )   |  \\___ \  |  | \  ___/|  | \/
//  \____|__  /\____/|___|  /____  > |__|  \___  >__|   
//          \/            \/     \/            \/       
 
    // Function to add a monster to the world and its current tile
    pub fn add_monster(&mut self, monster: Monster, commands: &mut Commands,) -> Result<(), MyError> {
        let (x, y) = monster.get_position();
        self.is_valid_position(x as usize, y as usize)?;

        let mut monsters = self.monsters.lock().unwrap();
        monsters.insert(monster.get_id(), (x as usize, y as usize));
        let entity = monster.get_entity();
        commands.entity(entity).insert(monster);
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
    
    pub fn populate_monsters(
        &mut self,
        valid_spawns: Vec<Vec<(u32, u32)>>,
        max_monsters: usize,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) {
        let mut rng = rand::thread_rng();
        let mut monsters_added = 0;
        let mut all_spawns: Vec<(u32, u32)> = valid_spawns.into_iter().flatten().collect();
        all_spawns.shuffle(&mut rng);

        for (row, col) in all_spawns {
            if monsters_added >= max_monsters {
                return;
            }

            let x = row as f32; 
            let y = col as f32; 
            let monster = Monster::new_monster(x, y, commands, materials, asset_server);
            if let Err(err) = self.add_monster(monster, commands) {
                eprintln!("Error adding monster: {:?}", err);
            }

            monsters_added += 1;
        }
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
