use crate::entities::monster::Monster;
use crate::entities::agent::Agent;


#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub agents: Vec<Agent>,
    pub monsters: Vec<Monster>, 
    pub treasures: Vec<Treasure>,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum TileType {
    Forest,
    Mountain,
    Lake,
    Village,
    Dungeon,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            agents: Vec::new(),
            monsters: Vec::new(),
            treasures: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    pub fn remove_agent(&mut self, agent_id: u32) -> Option<Agent> {
        if let Some(index) = self.agents.iter().position(|a| a.id == agent_id) {
            Some(self.agents.remove(index))
        } else {
            None
        }
    }

    // Similar functions can be implemented for monsters and treasures

    // You can add functions to get agents, monsters, and treasures on this tile if needed
}

#[derive(Clone)]
pub struct Treasure {
    // Add treasure properties
}

