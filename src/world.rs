use crate::entities::agent::Agent;
use crate::entities::monster::Monster;
use crate::tile::{TileType, Treasure};
use crate::tile::Tile;


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
        "fffffffffffffffmlllm",
        "ffffmfffffffffmllllm",
        "fffffmffffffffffmllm",
        "fffffffmfffffmmmmllm",
        "mmmmmmmmmmmfmmmmlllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "ffffffffvffffffflllm",
        "fmfffffffffffffflllm",
        "fffffffffffffffflllm",
    ];
    
    let mut world: Vec<Vec<Tile>> = map_data
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

    // Initialize monsters and treasures vectors for each tile in the world
    for row in &mut world {
        for tile in row {
            tile.monsters = Vec::new();
            tile.treasures = Vec::new();
        }
    }

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

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.grid.get(y)?.get(x)
    }

    // Function to add an agent to the world
    pub fn add_agent(&mut self, agent: Agent) {
        // Add the agent to the world's list of agents
        self.agents.push(agent);
    }

    // Function to get a reference to an agent by ID
    pub fn get_agent(&mut self, id: u32) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|a| a.id == id)
    }

    // Function to remove an agent from the world by ID
    pub fn remove_agent(&mut self, id: u32) -> Option<Agent> {
        // Remove the agent from the world's list of agents
        let index = self.agents.iter().position(|a| a.id == id)?;
        let removed_agent = self.agents.remove(index);
        Some(removed_agent)
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
