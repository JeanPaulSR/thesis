use bevy::prelude::Commands;

use crate::entities::agent::Agent;
use crate::entities::agent::AgentAction;
use crate::entities::agent::Genes;
use crate::entities::agent::Status;
use crate::entities::agent::Target;
use crate::entities::monster::Monster;
use crate::entities::treasure::Treasure;
use crate::mcst::NpcAction;
use crate::tile::TileType;
use crate::tile::Tile;
use crate::errors::MyError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};



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

pub struct World {
    pub agents: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub monsters: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub treasures: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub grid: Vec<Vec<Arc<Mutex<Tile>>>>, 
}


impl World {
    
    fn new() -> Self {
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
// ___________.__.__          
// \__    ___/|__|  |   ____  
//   |    |   |  |  | _/ __ \ 
//   |    |   |  |  |_\  ___/ 
//   |____|   |__|____/\____>


    // Function to get the Tile at position (x, y)
    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Arc<Mutex<Tile>>, MyError> {
        // Check if the position is valid before attempting to get the Tile
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
        // Check if the position is valid before attempting to get the Tile
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
        // Check if the position is valid before attempting to get the TileType
        self.is_valid_position(x, y)?;
    
        // The position is valid, proceed to get the TileType
        match self.get_tile(x, y) {
            Ok(tile) => {
                let tile_lock = tile.lock().unwrap(); // Lock the tile to access its type
                Ok(Some(tile_lock.get_tile_type().clone()))
            }
            Err(_) => Ok(None), // Handle the case when the tile is not found
        }
    }

    pub fn get_agent_position(&self, agent_id: u32) -> Result<(usize, usize), MyError> {
        match self.agents.lock().unwrap().get(&agent_id) {
            Some(position) => Ok(*position),
            None => Err(MyError::AgentNotFound),
        }
    }
    
    pub fn get_monster_position(&self, monster_id: u32) -> Result<(usize, usize), MyError> {
        match self.monsters.lock().unwrap().get(&monster_id) {
            Some(position) => Ok(*position),
            None => Err(MyError::MonsterNotFound),
        }
    }
    
    pub fn get_treasure_position(&self, treasure_id: u32) -> Result<(usize, usize), MyError> {
        match self.treasures.lock().unwrap().get(&treasure_id) {
            Some(position) => Ok(*position),
            None => Err(MyError::TreasureNotFound),
        }
    }
    
    
//    _____                         __   
//    /  _  \    ____   ____   _____/  |_ 
//   /  /_\  \  / ___\_/ __ \ /    \   __\
//  /    |    \/ /_/  >  ___/|   |  \  |  
//  \____|__  /\___  / \___  >___|  /__|  
//          \//_____/      \/     \/      
 
    // Function to add an agent to the world and its current tile
    pub fn add_agent(&mut self, agent: Agent) -> Result<(), MyError> {
        let (x, y) = agent.get_position();

        // Check if the position is valid before attempting to get the tile
        self.is_valid_position(x as usize, y as usize)?;

        // Attempt to find the tile in the grid that corresponds to the agent's position
        if let Some(tile) = self.grid.get(y as usize).and_then(|row| row.get(x as usize)) {
            // Lock the tile to safely add the agent
            let tile_lock = tile.lock().unwrap();

            // Lock the agents list to safely add the agent
            let mut agents = self.agents.lock().unwrap();

            // Add the agent's position to the agents hash map
            agents.insert(agent.get_id(), (x as usize, y as usize));

            // Add the agent to the tile's list of agents
            tile_lock.add_agent(agent);

            // Return Ok(()) to indicate successful addition
            Ok(())
        } else {
            // The position is out of bounds or the tile is not found, return an error
            println!("Attempted to add agent {} to tile at position ({}, {}), but the tile was not found.", agent.get_id(), x, y);
            Err(MyError::TileNotFound)
        }
    }

    // Function to remove an agent from the world and its tile
    pub fn remove_agent(&mut self, agent_id: u32) -> Result<(), MyError> {
        // Lock the agents hash map
        let mut agents_lock = self.agents.lock().unwrap();

        // Check if the agent's position is saved in the hash map
        if let Some((x, y)) = agents_lock.get(&agent_id) {
            // Get the tile at the agent's position
            if let Some(tile_mutex) = self.grid.get(*y as usize).and_then(|row| row.get(*x as usize)) {
                let mut tile = tile_mutex.lock().unwrap();

                // Remove the agent from the hash map and tile
                agents_lock.remove(&agent_id);
                tile.remove_agent(agent_id)?;

                return Ok(());
            }
        }

        // Agent not found, return an error
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

     //Currently posy and posx are flipped
     pub fn move_agent(
        &self,
        agent_id: u32,
        pos_y2: usize,
        pos_x2: usize,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let mut agents_positions = self.agents.lock().unwrap();
        
        // Check if the agent exists and get its position
        if let Some((pos_y1, pos_x1)) = agents_positions.get_mut(&agent_id) {
            // Lock both tiles' mutexes in a consistent order to avoid deadlocks
            let (tile1, tile2) = if *pos_x1 < pos_x2 || (*pos_x1 == pos_x2 && *pos_y1 < pos_y2) {
                let tile1 = self.grid[*pos_x1][*pos_y1].clone();
                let tile2 = self.grid[pos_x2][pos_y2].clone();
                (tile1, tile2)
            } else {
                let tile2 = self.grid[pos_x2][pos_y2].clone();
                let tile1 = self.grid[*pos_x1][*pos_y1].clone();
                (tile1, tile2)
            };

            // Lock the tiles' mutexes in the determined order
            let mut lock1 = tile1.lock().unwrap();
            let lock2 = tile2.lock().unwrap();

            // Update the agent's position directly
            *pos_x1 = pos_x2;
            *pos_y1 = pos_y2;

            // Remove the agent from Tile 1
            let mut removed_agent = lock1.remove_agent(agent_id)?;

            // Move the agent's sprite to the new position
            removed_agent.move_to( pos_y2 as f32, pos_x2 as f32,commands);

            // Add the agent to Tile 2
            lock2.add_agent(removed_agent);

            Ok(())
        } else {
            // Agent was not found in the agent HashMap
            Err(MyError::AgentNotFound)
        }
    }

    pub fn set_agent_action(&self, agent_id: u32, action: AgentAction) -> Result<(), MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let mut agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock by finding its index
                if let Some(index) = agents_lock.iter().position(|a| a.get_id() == agent_id) {
                    // Get a mutable reference to the agent at the found index
                    let agent = &mut agents_lock[index];
                    // Call the specific function based on the action
                    match action {
                        AgentAction::SetGene(gene) => agent.set_genes(gene),
                        AgentAction::SetEnergy(energy) => agent.set_energy(energy),
                        AgentAction::RemoveEnergy(energy) => agent.remove_energy(energy),
                        AgentAction::SetMaxEnergy(energy) => agent.set_max_energy(energy),
                        AgentAction::SetTarget(target) => agent.set_target(target),
                        AgentAction::SetAgentId(id) => agent.set_agent_target_id(id),
                        AgentAction::SetMonsterId(id) => agent.set_monster_target_id(id),
                        AgentAction::SetTreasureId(id) => agent.set_treasure_target_id(id),
                        AgentAction::SetStatus(status) => agent.set_status(status),
                        AgentAction::SetAction(npc_action) => agent.set_action(npc_action),
                        // Add more actions as needed
                    }
                    return Ok(());
                } else {
                    println!("Agent not found");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_genes(&self, agent_id: u32) -> Result<Genes, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_genes().clone());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }   

    pub fn get_agent_energy(&self, agent_id: u32) -> Result<u8, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_energy());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_max_energy(&self, agent_id: u32) -> Result<u8, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_max_energy());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_target(&self, agent_id: u32) -> Result<Target, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_target());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_agent_target_id(&self, agent_id: u32) -> Result<u32, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_agent_target_id());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_monster_target_id(&self, agent_id: u32) -> Result<u32, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_monster_target_id());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_treasure_target_id(&self, agent_id: u32) -> Result<u32, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_treasure_target_id());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_tile_target(&self, agent_id: u32) -> Result<Option<(u32, u32)>, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Return a clone of the agent's genes
                    return Ok(agent.get_tile_target());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_status(&self, agent_id: u32) -> Result<Status, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Call the function you want on the agent
                    return Ok(agent.get_status());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
    }

    pub fn get_agent_action(&self, agent_id: u32) -> Result<Option<NpcAction>, MyError> {
        if let Some((row, col)) = self.agents.lock().unwrap().get(&agent_id) {
            if let Some(tile) = self.grid.get(*row).and_then(|row| row.get(*col)) {
                let tile_lock = tile.lock().unwrap();
                let agents_lock = tile_lock.get_agents();
                
                // Now you can find the specific agent within agents_lock
                if let Some(agent) = agents_lock.iter().find(|a| a.get_id() == agent_id) {
                    // Call the function you want on the agent
                    return Ok(agent.get_action());
                } else {
                    println!("Agent not found attempting to get status");
                    return Err(MyError::AgentNotFound);
                }
            }
        }
        println!("Agent not found attempting to get status");
        Err(MyError::AgentNotFound)
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

        // Check if the position is valid before attempting to get the tile
        self.is_valid_position(x as usize, y as usize)?;

        // Attempt to find the tile in the grid that corresponds to the monster's position
        if let Some(tile) = self.grid.get(y as usize).and_then(|row| row.get(x as usize)) {
            // Lock the tile to safely add the monster
            let tile_lock = tile.lock().unwrap();

            // Lock the monsters list to safely add the monster
            let mut monsters = self.monsters.lock().unwrap();

            // Add the monster's position to the monsters hash map
            monsters.insert(monster.id, (x as usize, y as usize));

            // Add the monster to the tile's list of monsters
            tile_lock.add_monster(monster);

            // Return Ok(()) to indicate successful addition
            Ok(())
        } else {
            // The position is out of bounds or the tile is not found, return an error
            println!("Attempted to add monster {} to tile at position ({}, {}), but the tile was not found.", monster.id, x, y);
            Err(MyError::TileNotFound)
        }
    }

    // Function to remove a monster from the world and its tile
    pub fn remove_monster(&mut self, monster_id: u32) -> Result<(), MyError> {
        // Lock the monsters hash map
        let mut monsters_lock = self.monsters.lock().unwrap();

        // Check if the monster's position is saved in the hash map
        if let Some((x, y)) = monsters_lock.get(&monster_id) {
            // Get the tile at the monster's position
            if let Some(tile_mutex) = self.grid.get(*y as usize).and_then(|row| row.get(*x as usize)) {
                let mut tile = tile_mutex.lock().unwrap();

                // Remove the monster from the hash map and tile
                monsters_lock.remove(&monster_id);
                tile.remove_monster(monster_id)?;

                return Ok(());
            }
        }

        // Monster not found, return an error
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

        // Check if the position is valid before attempting to get the tile
        self.is_valid_position(x as usize, y as usize)?;

        // Attempt to find the tile in the grid that corresponds to the treasure's position
        if let Some(tile) = self.grid.get(y as usize).and_then(|row| row.get(x as usize)) {
            // Lock the tile to safely add the treasure
            let tile_lock = tile.lock().unwrap();

            // Lock the treasures list to safely add the treasure
            let mut treasures = self.treasures.lock().unwrap();

            // Add the treasure's position to the treasures hash map
            treasures.insert(treasure.id, (x as usize, y as usize));

            // Add the treasure to the tile's list of treasures
            tile_lock.add_treasure(treasure);

            // Return Ok(()) to indicate successful addition
            Ok(())
        } else {
            // The position is out of bounds or the tile is not found, return an error
            println!("Attempted to add treasure {} to tile at position ({}, {}), but the tile was not found.", treasure.id, x, y);
            Err(MyError::TileNotFound)
        }
    }

    // Function to remove a treasure from the world and its tile
    pub fn remove_treasure(&mut self, treasure_id: u32) -> Result<(), MyError> {
        // Lock the treasures hash map
        let mut treasures_lock = self.treasures.lock().unwrap();

        // Check if the treasure's position is saved in the hash map
        if let Some((x, y)) = treasures_lock.get(&treasure_id) {
            // Get the tile at the treasure's position
            if let Some(tile_mutex) = self.grid.get(*y as usize).and_then(|row| row.get(*x as usize)) {
                let mut tile = tile_mutex.lock().unwrap();

                // Remove the treasure from the hash map and tile
                treasures_lock.remove(&treasure_id);
                tile.remove_treasure(treasure_id)?;

                return Ok(());
            }
        }

        // Treasure not found, return an error
        Err(MyError::TreasureNotFound)
    }

}
