use crate::entities::agent::Agent;
use crate::entities::monster::Monster;
use crate::entities::treasure::Treasure;
use crate::tile::TileType;
use crate::tile::Tile;
use crate::errors::MyError;


const _WORLD_WIDTH: usize = 30;
const _WORLD_HEIGHT: usize = 30;



pub fn create_world() -> Vec<Vec<Tile>> {
    let map_data: Vec<&str> = vec![
        "vffffffffffffffffffm",
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
        "fffffffffffffffflllm",
    ];
    
    let world: Vec<Vec<Tile>> = map_data
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'm' => Tile::new(TileType::Mountain),
                    'l' => Tile::new(TileType::Lake),
                    'v' => Tile::new(TileType::Village),
                    'd' => Tile::new(TileType::Dungeon),
                    'f' => Tile::new(TileType::Forest),
                    _ => panic!("Invalid tile character: {}", c),
                })
                .collect()
        })
        .collect();

    world
}

pub struct World {
    pub agents: Vec<Agent>,
    pub monsters: Vec<Monster>,
    pub treasures: Vec<Treasure>,
    pub grid: Vec<Vec<Tile>>,
}

impl World {
    
    fn _new() -> Self {
        let agents = Vec::new();
        let monsters = Vec::new(); 
        let treasures = Vec::new(); 
        let grid: Vec<Vec<Tile>> = vec![vec![Tile::new(TileType::Forest); _WORLD_HEIGHT]; _WORLD_WIDTH];
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
    
// ___________.__.__          
// \__    ___/|__|  |   ____  
//   |    |   |  |  | _/ __ \ 
//   |    |   |  |  |_\  ___/ 
//   |____|   |__|____/\____>


    // Function to get the Tile at position (x, y)
    pub fn get_tile(&self, x: usize, y: usize) -> Result<Option<&Tile>, MyError> {
        // Check if the position is valid before attempting to get the Tile
        self.is_valid_position(x, y)?;
        // The position is valid, proceed to get the Tile
        Ok(self.grid.get(y).and_then(|row| row.get(x)))
    }
    
    // Function to get a mutable reference to the Tile at position (x, y)
    fn get_tile_mut(&mut self, x: usize, y: usize) -> Result<Option<&mut Tile>, MyError> {
        // Check if the position is valid before attempting to get the Tile
        self.is_valid_position(x, y)?;

        // The position is valid, proceed to get the Tile
        Ok(self.grid.get_mut(y).and_then(|row| row.get_mut(x)))
    }

    // Function to get the TileType at position (x, y)
    pub fn get_tile_type(&self, x: usize, y: usize) -> Result<Option<TileType>, MyError> {
        // Check if the position is valid before attempting to get the TileType
        self.is_valid_position(x, y)?;
        // The position is valid, proceed to get the TileType
        Ok(self.get_tile(x, y)?.map(|tile| tile.get_tile_type()))
    }

    
//    _____                         __   
//    /  _  \    ____   ____   _____/  |_ 
//   /  /_\  \  / ___\_/ __ \ /    \   __\
//  /    |    \/ /_/  >  ___/|   |  \  |  
//  \____|__  /\___  / \___  >___|  /__|  
//          \//_____/      \/     \/      
 
    // Function to add an agent to the world and its current tile
    pub fn add_agent(&mut self, agent: Agent) -> Result<(), MyError> {
        // Find the position (x, y) of the agent in the world
        let (x, y) = agent.get_position();

        // Check if the position is valid before attempting to get the tile
        self.is_valid_position(x as usize, y as usize)?;

        // Attempt to find the tile in the grid that corresponds to the agent's position
        if let Some(tile) = self.grid.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            // Add the agent to the world's list of agents
            self.agents.push(agent.clone());

            // Add the agent to the tile's list of agents
            tile.add_agent(agent);

            // Return Ok(()) to indicate successful addition
            Ok(())
        } else {
            // The position is out of bounds or the tile is not found, return an error
            // Note: It's a good practice to provide additional information in the error message.
            print!("Attempted to add agent {} to tile at position ({}, {}), but the tile was not found.", agent.id, x, y);
            Err(MyError::TileNotFound)
        }
    }

    // Function to get a reference to an agent by ID
    pub fn get_agent(&self, id: u32) -> Result<&Agent, MyError> {
        // Find the agent with the specified ID in the agents vector
        let agent = self.agents.iter().find(|agent| agent.id == id);

        // Check if the agent is found in the agents vector; if not, return an error
        let agent = agent.ok_or(MyError::AgentNotFound)?;

        // Return a reference to the found agent
        Ok(agent)
    }       
    
    // Function to remove an agent from the world and its tile
    pub fn remove_agent(&mut self, agent_id: u32) -> Result<(), MyError> {
        // Check if the agent is found in the world's agents vector
        if let Some(index) = self.agents.iter().position(|a| a.id == agent_id) {
            // Agent found in the world, remove it from the world's agents vector
            let agent = self.agents.remove(index);

            // Find the tile the agent is currently in based on its position
            let (x, y) = agent.get_position();

            // Get the tile at position (x, y)
            if let Some(tile) = self.get_tile_mut(x as usize, y as usize)? {
                // Remove the agent from the tile
                tile.remove_agent(agent_id)?;
            } else {
                // The position is out of bounds or the tile is not found, return an error
                // Note: It's a good practice to provide additional information in the error message.
                println!("Agent {} not found in tile at position ({}, {}).", agent_id, x, y);
                return Err(MyError::AgentNotFound);
            }

            Ok(())
        } else {
            // Agent not found in the world's agents vector, return an error
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
        // Find the tile the monster is currently in based on its position
        let (x, y) = monster.get_position();

        // Check if the position is valid before attempting to get the tile
        if self.is_valid_position(x as usize, y as usize).is_err() {
            return Err(MyError::PositionError);
        }

        // Add the monster to the world's list of monsters
        self.monsters.push(monster.clone());

        if let Some(tile) = self.grid.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            // Add the monster to the tile's list of monsters
            tile.add_monster(monster);
            Ok(())
        } else {
            // The position is out of bounds, return an error
            Err(MyError::PositionError)
        }
    }

    // Function to get a reference to a monster by ID
    pub fn get_monster(&self, id: u32) -> Result<&Monster, MyError> {
        let monster = self.monsters.iter().find(|monster| monster.id == id);
        // Find the tile the monster is currently in based on its position
        let (x, y) = monster.and_then(|m| Some(m.get_position())).ok_or(MyError::PositionError)?;

        // Check if the position is valid before attempting to get the tile
        if self.is_valid_position(x as usize, y as usize).is_err() {
            return Err(MyError::PositionError);
        }

        self.monsters.iter().find(|m| m.id == id).ok_or(MyError::PositionError)
    }

    // Function to remove a monster from the world and its tile
    pub fn remove_monster(&mut self, monster_id: u32) -> Result<(), MyError> {
        // Check if the monster is found in the world's monsters vector
        if let Some(index) = self.monsters.iter().position(|m| m.id == monster_id) {
            // Monster found in the world, remove it from the world's monsters vector
            let monster = self.monsters.remove(index);

            // Find the tile the monster is currently in based on its position
            let (x, y) = monster.get_position();

            // Get the tile at position (x, y)
            if let Some(tile) = self.get_tile_mut(x as usize, y as usize)? {
                // Remove the monster from the tile
                tile.remove_monster(monster_id)?;
            } else {
                // The position is out of bounds or the tile is not found, return an error
                // Note: It's a good practice to provide additional information in the error message.
                println!("Monster {} not found in tile at position ({}, {}).", monster_id, x, y);
                return Err(MyError::MonsterNotFound);
            }

            Ok(())
        } else {
            // Monster not found in the world's monsters vector, return an error
            Err(MyError::MonsterNotFound)
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
        // Find the tile the treasure is currently in based on its position
        let (x, y) = treasure.get_position();

        // Check if the position is valid before attempting to get the tile
        if self.is_valid_position(x as usize, y as usize).is_err() {
            return Err(MyError::PositionError);
        }

        // Add the treasure to the world's list of treasures
        self.treasures.push(treasure.clone());

        if let Some(tile) = self.grid.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            // Add the treasure to the tile's list of treasures
            tile.add_treasure(treasure);
            Ok(())
        } else {
            // The position is out of bounds, return an error
            Err(MyError::PositionError)
        }
    }

    // Function to get a reference to a treasure by ID
    pub fn get_treasure(&self, id: u32) -> Result<&Treasure, MyError> {
        let treasure = self.treasures.iter().find(|treasure| treasure.id == id);
        // Find the tile the treasure is currently in based on its position
        let (x, y) = treasure.and_then(|t| Some(t.get_position())).ok_or(MyError::PositionError)?;

        // Check if the position is valid before attempting to get the tile
        if self.is_valid_position(x as usize, y as usize).is_err() {
            return Err(MyError::PositionError);
        }

        self.treasures.iter().find(|t| t.id == id).ok_or(MyError::PositionError)
    }

    // Function to remove a treasure from the world and its tile
    pub fn remove_treasure(&mut self, treasure_id: u32) -> Result<(), MyError> {
        // Check if the treasure is found in the world's treasures vector
        if let Some(index) = self.treasures.iter().position(|t| t.id == treasure_id) {
            // Treasure found in the world, remove it from the world's treasures vector
            let treasure = self.treasures.remove(index);

            // Find the tile the treasure is currently in based on its position
            let (x, y) = treasure.get_position();

            // Get the tile at position (x, y)
            if let Some(tile) = self.get_tile_mut(x as usize, y as usize)? {
                // Remove the treasure from the tile
                tile.remove_treasure(treasure_id)?;
            } else {
                // The position is out of bounds or the tile is not found, return an error
                // Note: It's a good practice to provide additional information in the error message.
                println!("Treasure {} not found in tile at position ({}, {}).", treasure_id, x, y);
                return Err(MyError::TreasureNotFound);
            }

            Ok(())
        } else {
            // Treasure not found in the world's treasures vector, return an error
            Err(MyError::TreasureNotFound)
        }
    }

    pub fn find_agents_within_distance(&self, agent: &Agent, distance: f32) -> Vec<&Agent> {
        let mut nearby_agents = Vec::new();
    
        for other_agent in &self.agents {
            if agent.id != other_agent.id {
                let dx = (agent.transform.translation.x - other_agent.transform.translation.x).abs() / 32.0;
                let dy = (agent.transform.translation.y - other_agent.transform.translation.y).abs() / 32.0;
                let squared_distance = dx * dx + dy * dy;
                let calculated_distance = squared_distance.sqrt();
                if calculated_distance <= distance {
                    nearby_agents.push(other_agent);
                }
            }
        }
    
        nearby_agents
    }
}
