//For loop that takes all the agents

//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity

use std::collections::HashMap;

use crate::{entities::{agent::Agent, monster::{Monster, SimpleMonster}, treasure::{Treasure, SimpleTreasure}}, tile::Tile, world::World, tests::simple_agent::SimpleAgent};

#[allow(dead_code)]
pub struct GameState {
    agents: HashMap<u32, SimpleAgent>,
    monsters: HashMap<u32, SimpleMonster>,
    treasures: HashMap<u32, SimpleTreasure>,
    grid: Vec<Vec<Tile>>,
    // Add other relevant game state information here
}

#[allow(dead_code)]
impl GameState {
    pub fn create_gamestate(agents: Vec<Agent>, monsters: Vec<Monster>, treasures: Vec<Treasure>, world: World) -> Self {
        let mut agent_map = HashMap::new();
        let mut monster_map = HashMap::new();
        let mut treasure_map = HashMap::new();

        for agent in agents {
            let id = agent.get_id();
            agent_map.insert(id, SimpleAgent::simple_agent_convert(&agent));
        }

        for monster in monsters {
            let id = monster.get_id();
            monster_map.insert(id, SimpleMonster::new(&monster));
        }

        for treasure in treasures {
            let id = treasure.get_id();
            treasure_map.insert(id, SimpleTreasure::new(&treasure));
        }

        // Create the GameState struct
        GameState {
            agents: agent_map,
            monsters: monster_map,
            treasures: treasure_map,
            grid: world.get_grid(),
        }
    }

    pub fn create_gamestate_simple(agents: Vec<SimpleAgent>, monsters: Vec<SimpleMonster>, treasures: Vec<SimpleTreasure>, world: World) -> Self {
        let mut agent_map = HashMap::new();
        let mut monster_map = HashMap::new();
        let mut treasure_map = HashMap::new();

        for agent in agents {
            let id = agent.get_id();
            agent_map.insert(id, agent);
        }

        for monster in monsters {
            let id = monster.get_id();
            monster_map.insert(id, monster);
        }

        for treasure in treasures {
            let id = treasure.get_id();
            treasure_map.insert(id, treasure);
        }

        // Create the GameState struct
        GameState {
            agents: agent_map,
            monsters: monster_map,
            treasures: treasure_map,
            grid: world.get_grid(),
        }
    }
}

