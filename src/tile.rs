use crate::entities::monster::Monster;
use crate::entities::agent::Agent;
use crate::entities::treasure::Treasure;

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    agents: Vec<Agent>,
    monsters: Vec<Monster>, 
    treasures: Vec<Treasure>,
}

#[derive(Clone, Copy, PartialEq)]
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
            agents: Vec::new(),
            monsters: Vec::new(),
            treasures: Vec::new(),
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

    // Function to add an agent to the tile
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    // Function to remove an agent from the tile
    pub fn remove_agent(&mut self, agent_id: u32) -> bool {
        if let Some(index) = self.agents.iter().position(|a| a.id == agent_id) {
            self.agents.remove(index);
            true
        } else {
            false
        }
    }

    // Function to get a reference to an agent by ID
    pub fn get_agent(&mut self, id: u32) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|a| a.id == id)
    }

    // Function to add a monster to the tile
    pub fn add_monster(&mut self, monster: Monster) {
        self.monsters.push(monster);
    }

    // Function to remove an agent from the tile
    pub fn remove_monster(&mut self, monster_id: u32) -> bool {
        if let Some(index) = self.monsters.iter().position(|m| m.id == monster_id) {
            self.monsters.remove(index);
            true
        } else {
            false
        }
    }

    // Function to get a reference to a monster by ID
    pub fn get_monster(&mut self, id: u32) -> Option<&mut Monster> {
        self.monsters.iter_mut().find(|m| m.id == id)
    }

    // Function to add a treasure to the tile
    pub fn add_treasure(&mut self, treasure: Treasure) {
        self.treasures.push(treasure);
    }

    // Function to remove an agent from the tile
    pub fn remove_treasure(&mut self, treasure_id: u32) -> bool {
        if let Some(index) = self.treasures.iter().position(|t| t.id == treasure_id) {
            self.treasures.remove(index);
            true
        } else {
            false
        }
    }

    // Function to get a reference to a treasure by ID
    pub fn get_treasure(&mut self, id: u32) -> Option<&mut Treasure> {
        self.treasures.iter_mut().find(|t| t.id == id)
    }
}


