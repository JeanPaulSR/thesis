// Bevy imports
use bevy::prelude::Resource;

// Standard library imports
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};

// Internal crate imports
use crate::gameworld::position::Position;
use crate::gameworld::tile::Tile;
use crate::gameworld::tile_types::TileType;

#[derive(Clone, Resource)]
pub struct GameWorld {
    pub tiles: HashMap<Position, Arc<Mutex<Tile>>>,
    pub width_mind: i32,
    pub height_min: i32,
    pub width_max: i32,
    pub height_max: i32,
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

    // Construct an empty world
    pub fn new() -> Self {
        let tiles: HashMap<Position, Arc<Mutex<Tile>>> = HashMap::new();

        GameWorld {
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

        // Initialize min and max values
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for (y, row_str) in map_data.iter().enumerate() {
            for (x, c) in row_str.chars().enumerate() {
                let tile_type = match c {
                    'm' => TileType::Mountain,
                    'l' => TileType::Lake,
                    'v' => TileType::Village,
                    'd' => TileType::Dungeon,
                    'f' => TileType::Forest,
                    'F' => TileType::Farm,
                    'M' => TileType::Mine,
                    _ => panic!("Invalid tile character: {}", c),
                };

                let tile = Arc::new(Mutex::new(Tile::new(tile_type)));

                // Add the tile to the tiles HashMap with its position
                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };
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
    pub fn is_valid_position(&self, x: i32, y: i32) -> bool {
        let position = Position { x, y };
        self.tiles.contains_key(&position)
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

    // Creates a copy of the world
    pub fn copy(&self) -> Self {
        // Deep copy the tiles HashMap
        let mut new_tiles = HashMap::new();
        for (position, tile_arc_mutex) in &self.tiles {
            let tile_mutex = Mutex::new(tile_arc_mutex.lock().unwrap().clone());
            let arc_tile = Arc::new(tile_mutex);
            new_tiles.insert(*position, arc_tile);
        }

        GameWorld {
            tiles: new_tiles,
            width_mind: self.width_mind,
            height_min: self.height_min,
            width_max: self.width_max,
            height_max: self.height_max,
        }
    }

    // Function to get the Tile at position (x, y)
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Arc<Mutex<Tile>>> {
        let position = Position { x, y };
        self.tiles.get(&position).cloned()
    }

    // Function to get a mutable reference to the Tile at position (x, y)
    pub fn get_tile_mut(&self, x: i32, y: i32) -> Option<Arc<Mutex<Tile>>> {
        self.get_tile(x, y)
    }

    // Function to get the TileType at position (x, y)
    pub fn get_tile_type(&self, x: i32, y: i32) -> Option<TileType> {
        self.get_tile(x, y).map(|tile| {
            let tile_lock = tile.lock().unwrap();
            tile_lock.get_tile_type().clone()
        })
    }

    pub fn find_closest_villages(&self) -> Vec<(Position, Position)> {
        let village_positions: Vec<Position> = self
            .tiles
            .iter()
            .filter_map(|(position, tile)| {
                let tile_lock = tile.lock().unwrap();
                if tile_lock.get_tile_type() == TileType::Village {
                    Some(*position)
                } else {
                    None
                }
            })
            .collect();

        let mut result = Vec::new();

        for &village_pos in &village_positions {
            let mut closest_village = None;
            let mut min_distance = f64::MAX;

            for &other_village_pos in &village_positions {
                if village_pos != other_village_pos {
                    let distance = (((village_pos.x - other_village_pos.x).pow(2)
                        + (village_pos.y - other_village_pos.y).pow(2)) as f64)
                        .sqrt();
                    if distance < min_distance {
                        min_distance = distance;
                        closest_village = Some(other_village_pos);
                    }
                }
            }

            if let Some(closest) = closest_village {
                result.push((village_pos, closest));
            }
        }

        result
    }

    pub fn find_closest_tiletype(&self, position: Position, tile_type: TileType) -> Option<Position> {
        let mut closest_tile = None;
        let mut min_cost = i32::MAX;

        for (&target_pos, tile) in &self.tiles {
            let tile_lock = tile.lock().unwrap();
            if tile_lock.get_tile_type() == tile_type {
                let cost = (position.x - target_pos.x).abs() + (position.y - target_pos.y).abs();
                if cost < min_cost {
                    min_cost = cost;
                    closest_tile = Some(target_pos);
                }
            }
        }

        closest_tile
    }

    // Function to check if the position is within the grid's bounds
    pub fn is_within_bounds(&self, position: Position) -> bool {
        position.x >= self.width_mind
            && position.x <= self.width_max
            && position.y >= self.height_min
            && position.y <= self.height_max
    }
}
