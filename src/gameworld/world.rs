// Bevy imports
use bevy::asset::{AssetServer, Assets};
use bevy::ecs::system::{Res, ResMut};
use bevy::prelude::{Commands, Resource};
use bevy::sprite::TextureAtlas;

// External crate imports
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;

// Standard library imports
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};

// Internal crate imports
use crate::entities::agent::{Agent, Target};
use crate::entities::monster::Monster;
use crate::entities::treasure::Treasure;
use crate::errors::MyError;
use crate::gameworld::position::Position;
use crate::gameworld::tile::Tile;
use crate::gameworld::tile_types::TileType;
use crate::movement::find_path;

#[derive(Clone, Resource)]
pub struct GameWorld {
    pub agents: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub monsters: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub treasures: Arc<Mutex<HashMap<u32, (usize, usize)>>>,
    pub grid: Vec<Vec<Arc<Mutex<Tile>>>>,
    pub tiles: HashMap<Position, Arc<Mutex<Tile>>>,
    pub width_mind : i32,
    pub height_min : i32,
    pub width_max : i32,
    pub height_max : i32,
}

/// Standalone function to initialize the game world.
pub fn initialize(text_file_name: &str) -> io::Result<GameWorld> {
    GameWorld::initialize(text_file_name)
}


impl GameWorld {
    /// Reads a text file and returns a vector of strings, where each string represents a row in the world map.
    pub fn read_world(text_file_name: &str) -> io::Result<Vec<String>> {
        let file_path = format!("./worlds/{}.txt", text_file_name);
        let path = Path::new(&file_path);

        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let map_data: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        Ok(map_data)
    }

    //Construct a empty world
    pub fn new() -> Self {
        let agents = Arc::new(Mutex::new(HashMap::new()));
        let monsters = Arc::new(Mutex::new(HashMap::new()));
        let treasures = Arc::new(Mutex::new(HashMap::new()));
    
        let grid: Vec<Vec<Arc<Mutex<Tile>>>> = Vec::new();
        let tiles: HashMap<Position, Arc<Mutex<Tile>>> = HashMap::new();
    
        GameWorld {
            agents,
            monsters,
            treasures,
            grid,
            tiles,
            width_mind: i32::MAX,
            height_min: i32::MAX,
            width_max: i32::MIN,
            height_max: i32::MIN,
        }
    }

    /// Creates a world using a vector of strings as the map data.
    pub fn create_world(map_data: Vec<String>) -> Self {
        let mut world = GameWorld::new();
        world.grid = Vec::new();

        // Initialize min and max values
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for (y, row_str) in map_data.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in row_str.chars().enumerate() {
                let tile_type = match c {
                    'm' => TileType::Mountain,
                    'l' => TileType::Lake,
                    'v' => TileType::Village,
                    'd' => TileType::Dungeon,
                    'f' => TileType::Forest,
                    _ => panic!("Invalid tile character: {}", c),
                };

                let tile = Arc::new(Mutex::new(Tile::new(tile_type)));
                row.push(tile.clone());

                // Add the tile to the tiles HashMap with its position
                let position = Position { x: x as i32, y: y as i32 };
                world.tiles.insert(position, tile);

                // Update min and max values
                if position.x < min_x {
                    min_x = position.x;
                }
                if position.y < min_y {
                    min_y = position.y;
                }
                if position.x > max_x {
                    max_x = position.x;
                }
                if position.y > max_y {
                    max_y = position.y;
                }
            }
            world.grid.push(row);
        }

        // Store the min and max values in the world
        world.width_mind = min_x;
        world.height_min = min_y;
        world.width_max = max_x;
        world.height_max = max_y;

        world
    }


    pub fn initialize(text_file_name: &str) -> io::Result<Self> {
        let map_data = Self::read_world(text_file_name)?;
        Ok(Self::create_world(map_data))
    }

    // Function to check if the position (x, y) is within the grid's bounds
    fn is_valid_position(&self, x: usize, y: usize) -> Result<(), MyError> {
        let position = Position { x: x as i32, y: y as i32 };
        if self.tiles.contains_key(&position) {
            Ok(())
        } else {
            println!("Asked for position ({}, {}), was not found.", x, y);
            Err(MyError::PositionError)
        }
    }

    pub fn get_tiles(&self) -> &HashMap<Position, Arc<Mutex<Tile>>> {
        &self.tiles
    }

    pub fn get_width_mind(&self) -> i32 {
        self.width_mind
    }

    pub fn get_height_min(&self) -> i32 {
        self.height_min
    }

    pub fn get_width_max(&self) -> i32 {
        self.width_max
    }

    pub fn get_height_max(&self) -> i32 {
        self.height_max
    }

//      __  ______     __________     __  ____________  ______
//     / / / / __ \   /_  __/ __ \   / / / / ____/ __ \/ ____/
//    / / / / /_/ /    / / / / / /  / /_/ / __/ / /_/ / __/   
//   / /_/ / ____/    / / / /_/ /  / __  / /___/ _, _/ /___   
//   \____/_/        /_/  \____/  /_/ /_/_____/_/ |_/_____/

    //Creates a copy of the world
    pub fn copy(&self) -> Self {
        let agents = Arc::new(Mutex::new(self.agents.lock().unwrap().clone()));
        let monsters = Arc::new(Mutex::new(self.monsters.lock().unwrap().clone()));
        let treasures = Arc::new(Mutex::new(self.treasures.lock().unwrap().clone()));
    
        // Deep copy the grid
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
    
        // Deep copy the tiles HashMap
        let mut new_tiles = HashMap::new();
        for (position, tile_arc_mutex) in &self.tiles {
            let tile_mutex = Mutex::new(tile_arc_mutex.lock().unwrap().clone());
            let arc_tile = Arc::new(tile_mutex);
            new_tiles.insert(*position, arc_tile);
        }
    
        GameWorld {
            agents,
            monsters,
            treasures,
            grid: new_grid,
            tiles: new_tiles,
            width_mind: self.width_mind,
            height_min: self.height_min,
            width_max: self.width_max,
            height_max: self.height_max,
        }
    }

    pub fn is_next_to(&self, position: (usize, usize), target: Target, number: u32) -> bool {
        // Define the directions to check for adjacency
        let directions = [
            (0, 1),   // Right
            (1, 0),   // Down
            (0, -1),  // Left
            (-1, 0),  // Up
            (-1, -1), // Up-Left
            (-1, 1),  // Up-Right
            (1, -1),  // Down-Left
            (1, 1),   // Down-Right
        ];

        // Helper function to check bounds
        let is_within_bounds = |x: isize, y: isize| -> Option<(usize, usize)> {
            if x >= 0 && y >= 0 {
                Some((x as usize, y as usize))
            } else {
                None
            }
        };

        match target {
            Target::Agent => {
                if let Some(agents) = self.agents.lock().ok() {
                    if let Some(&(x, y)) = agents.get(&number) {
                        return directions.iter().any(|&(dx, dy)| {
                            is_within_bounds(position.0 as isize + dx, position.1 as isize + dy)
                                == Some((x, y))
                        });
                    }
                }
            }
            Target::Monster => {
                if let Some(monsters) = self.monsters.lock().ok() {
                    if let Some(&(x, y)) = monsters.get(&number) {
                        return directions.iter().any(|&(dx, dy)| {
                            is_within_bounds(position.0 as isize + dx, position.1 as isize + dy)
                                == Some((x, y))
                        });
                    }
                }
            }
            Target::Treasure => {
                if let Some(treasures) = self.treasures.lock().ok() {
                    if let Some(&(x, y)) = treasures.get(&number) {
                        return directions.iter().any(|&(dx, dy)| {
                            is_within_bounds(position.0 as isize + dx, position.1 as isize + dy)
                                == Some((x, y))
                        });
                    }
                }
            }
            Target::Tile | Target::None => return false,
        }

        false
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
    pub fn get_tile_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> Result<Option<&mut Arc<Mutex<Tile>>>, MyError> {
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

    pub fn get_valid_monster_spawns(&self) -> Vec<Vec<(u32, u32)>> {
        let world = &self;
        let mut valid_spawns: Vec<Vec<(u32, u32)>> = Vec::new();

        for (row_index, row) in world.grid.iter().enumerate() {
            let mut valid_row_spawns: Vec<(u32, u32)> = Vec::new();

            for (col_index, tile_mutex) in row.iter().enumerate() {
                let tile = tile_mutex.lock().unwrap();

                if !tile.is_monster_spawn() {
                    continue;
                }

                // Check if atop another monster
                if world
                    .monsters
                    .lock()
                    .unwrap()
                    .values()
                    .any(|pos| pos == &(row_index, col_index))
                {
                    continue;
                }

                // Check if within 5 tiles of an agent
                for agent_pos in world.agents.lock().unwrap().values() {
                    let agent_row = agent_pos.0;
                    let agent_col = agent_pos.1;
                    let distance_squared = ((row_index as isize - agent_row as isize).pow(2)
                        + (col_index as isize - agent_col as isize).pow(2))
                        as usize;

                    if distance_squared <= 25 {
                        continue;
                    }
                }

                valid_row_spawns.push((row_index as u32, col_index as u32));
            }

            if !valid_row_spawns.is_empty() {
                valid_spawns.push(valid_row_spawns);
            }
        }

        valid_spawns
    }

    // Function to check if the position (x, y) is within the grid's bounds
    fn is_valid_position_bool(&self, x: isize, y: isize) -> bool {
        x >= 0
            && y >= 0
            && (y as usize) < self.grid.len()
            && (x as usize) < self.grid[y as usize].len()
    }

    pub fn set_valid_monster_spawns(&self) {
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
                                            let mut new_tile = world.grid[new_row as usize]
                                                [new_col as usize]
                                                .lock()
                                                .unwrap();
                                            new_tile.set_monster_spawn(false);
                                        }
                                    }
                                }
                            }
                        }
                        TileType::Mountain => {
                            tile.set_monster_spawn(false);
                        }
                        TileType::Lake => {
                            tile.set_monster_spawn(false);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn find_closest_villages(&self) -> Vec<(u32, (u32, u32))> {
        let agents = self.agents.lock().unwrap();
        let mut village_positions = HashMap::new();

        // Collect all village positions and their agent IDs
        for (&agent_id, &(x, y)) in agents.iter() {
            let tile = self.grid[y][x].lock().unwrap();
            if tile.get_tile_type() == TileType::Village {
                village_positions.insert(agent_id, (x, y));
            }
        }

        let mut result = Vec::new();

        // Find the closest village for each village
        for (&agent_id, &(x1, y1)) in &village_positions {
            let mut closest_village = None;
            let mut min_distance = std::f64::MAX;

            for (&other_id, &(x2, y2)) in &village_positions {
                if agent_id != other_id {
                    let distance =
                        ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt();
                    if distance < min_distance {
                        min_distance = distance;
                        closest_village = Some((other_id, (x2 as u32, y2 as u32)));
                    }
                }
            }

            if let Some(closest) = closest_village {
                result.push((agent_id, closest.1));
            }
        }

        result
    }


    /// Finds the closest tile of the specified `TileType` from a given position.
    ///
    /// # Arguments
    /// - `position`: The starting position `(x, y)` in grid coordinates.
    /// - `tile_type`: The type of tile to search for (e.g., `TileType::Forest`).
    ///
    /// # Returns
    /// - `Some((x, y))`: The position of the closest tile of the specified type.
    /// - `None`: If no matching tile is found or no path exists.
    pub fn find_closest_tiletype(
        &self,
        position: (usize, usize),
        tile_type: TileType,
    ) -> Option<(usize, usize)> {
        let mut closest_tile = None;
        let mut min_cost = i32::MAX;
        let grid: Vec<Vec<Tile>> = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| tile.lock().unwrap().clone())
                    .collect()
            })
            .collect();
        let mut target_positions = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let tile = tile.lock().unwrap();
                if tile.get_tile_type() == tile_type {
                    target_positions.push((x as i32, y as i32));
                }
            }
        }
        for &target_pos in &target_positions {
            if let Some(path) = find_path(
                grid.clone(),
                (position.0 as i32, position.1 as i32),
                target_pos,
            ) {
                let cost = path.len() as i32;
                if cost < min_cost {
                    min_cost = cost;
                    closest_tile = Some((target_pos.0 as usize, target_pos.1 as usize));
                }
            }
        }
        closest_tile
    }
    //    _____                         __
    //    /  _  \    ____   ____   _____/  |_
    //   /  /_\  \  / ___\_/ __ \ /    \   __\
    //  /    |    \/ /_/  >  ___/|   |  \  |
    //  \____|__  /\___  / \___  >___|  /__|
    //          \//_____/      \/     \/

    // Function to add an agent to the world and its current tile
    pub fn add_agent(&mut self, agent: Agent, commands: &mut Commands) -> Result<(), MyError> {
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
    pub fn move_agent(&self, agent_id: u32, pos_y2: usize, pos_x2: usize) -> Result<(), MyError> {
        let mut agents_positions = self.agents.lock().unwrap();
        if let Some((pos_y1, pos_x1)) = agents_positions.get_mut(&agent_id) {
            *pos_x1 = pos_x2;
            *pos_y1 = pos_y2;

            Ok(())
        } else {
            Err(MyError::AgentNotFound)
        }
    }

    // Function to spawn agents based on START_AGENT_COUNT
    // Function to populate agents in villages up to a given count
    pub fn populate_agents(
        &mut self,
        start_agent_count: usize,
        commands: &mut Commands,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
    ) {
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
                asset_server,
                texture_atlases,
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

    pub fn find_closest_agents(&self) -> Vec<(u32, u32)> {
        let agents = self.agents.lock().unwrap();
        let mut result = Vec::new();

        for (&agent_id, &(x1, y1)) in agents.iter() {
            let mut closest_id = None;
            let mut min_distance = std::f64::MAX;

            for (&other_id, &(x2, y2)) in agents.iter() {
                if agent_id != other_id {
                    let distance =
                        ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt();
                    if distance < min_distance {
                        min_distance = distance;
                        closest_id = Some(other_id);
                    }
                }
            }

            if let Some(id) = closest_id {
                result.push((agent_id, id));
            }
        }

        result
    }

    pub fn get_agent_positions(&self) -> Vec<(u32, (usize, usize))> {
        // Lock the Mutex to safely access the agents HashMap
        let agents_guard = self.agents.lock().unwrap();

        // Initialize an empty vector to store the positions
        let mut positions = Vec::new();
        // Iterate over the agents and explicitly add each one to the vector
        for (id, pos) in agents_guard.iter() {
            positions.push((id.clone(), pos.clone()));
        }

        // Return the vector of positions
        positions
    }

    //     _____                          __
    //    /     \   ____   ____   _______/  |_  ___________
    //   /  \ /  \ /  _ \ /    \ /  ___/\   __\/ __ \_  __ \
    //  /    Y    (  <_> )   |  \\___ \  |  | \  ___/|  | \/
    //  \____|__  /\____/|___|  /____  > |__|  \___  >__|
    //          \/            \/     \/            \/

    // Function to add a monster to the world and its current tile
    pub fn add_monster(
        &mut self,
        monster: Monster,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        let (x, y) = monster.get_position();
        self.is_valid_position(x as usize, y as usize)?;

        let mut monsters = self.monsters.lock().unwrap();
        monsters.insert(monster.get_id(), (x as usize, y as usize));
        let entity = monster.get_entity();

        // Insert `Monster` component into the entity
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
        random: &mut StdRng,
        max_monsters: usize,
        commands: &mut Commands,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
    ) {
        let mut monsters_added = 0;
        let mut all_spawns: Vec<(u32, u32)> = valid_spawns.into_iter().flatten().collect();
        all_spawns.shuffle(random);

        for (row, col) in all_spawns {
            if monsters_added >= max_monsters {
                return;
            }

            let x = col as f32;
            let y = row as f32;

            let monster =
                Monster::new_monster(x * 32.0, y * 32.0, commands, texture_atlases, asset_server);
            if let Err(err) = self.add_monster(monster, commands) {
                eprintln!("Error adding monster: {:?}", err);
            }

            monsters_added += 1;
        }
    }

    pub fn find_closest_monsters(&self) -> Vec<(u32, u32, (u32, u32))> {
        let agents = self.agents.lock().unwrap();
        let monsters = self.monsters.lock().unwrap();
        let mut result = Vec::new();
        let mut position = (0, 0);

        for (&agent_id, &(x1, y1)) in agents.iter() {
            let mut closest_id = None;
            let mut min_distance = std::f64::MAX;

            for (&monster_id, &(x2, y2)) in monsters.iter() {
                let distance =
                    ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    closest_id = Some(monster_id);
                    position = (x2 as u32, y2 as u32);
                }
            }

            if let Some(id) = closest_id {
                result.push((agent_id, id, position));
            }
        }

        result
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

    pub fn find_closest_treasures(&self) -> Vec<(u32, u32, (u32, u32))> {
        let agents = self.agents.lock().unwrap();
        let treasures = self.treasures.lock().unwrap();
        let mut result = Vec::new();
        let mut position = (0, 0);

        for (&agent_id, &(x1, y1)) in agents.iter() {
            let mut closest_id = None;
            let mut min_distance = std::f64::MAX;

            for (&treasure_id, &(x2, y2)) in treasures.iter() {
                let distance =
                    ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    closest_id = Some(treasure_id);
                    position = (x2 as u32, y2 as u32);
                }
            }

            if let Some(id) = closest_id {
                result.push((agent_id, id, position));
            }
        }

        result
    }

    // Function to get a tile by its position using the tiles HashMap
    pub fn get_tile_by_position(&self, position: &Position) -> Option<Arc<Mutex<Tile>>> {
        self.tiles.get(position).cloned()
    }
}
