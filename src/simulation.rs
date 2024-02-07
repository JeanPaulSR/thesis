//For loop that takes all the agents

//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity
use bevy::prelude::*;
use crate::entities::agent::{Status, Target};
use crate::mcst::NpcAction;
use crate::World;
use crate::AgentMessages;
use crate::MonsterMessages;
use crate::TreasureMessages;
use crate::mcst;

use std::collections::HashMap;


use crate::entities::{agent::Agent, monster::{Monster, SimpleMonster}, treasure::{Treasure, SimpleTreasure}};

#[allow(dead_code)]
#[derive(Clone)]
pub struct GameState {
    agents: HashMap<u32, AgentData>,
    monsters: HashMap<u32, SimpleMonster>,
    treasures: HashMap<u32, SimpleTreasure>,
}

#[derive(Clone)]
pub struct AgentData{
    id: u32,
    action: NpcAction,
    status: Status,
    target: Target,
    target_id: u32,
    tile_target: Option<(u32, u32)>,
}

impl AgentData{
    fn new_agentdata(agent_id: u32, action_revieved: NpcAction, status_revieved: Status, target_revieved: Target,
    target_id_revieved: u32, tile_target_revieved: Option<(u32, u32)>) -> Self{
        AgentData{
            id: agent_id,
            action: action_revieved,
            status: status_revieved,
            target: target_revieved,
            target_id: target_id_revieved,
            tile_target: tile_target_revieved,
        }
    }
    
    fn new_agentdata_agent(agent: &Agent) -> Self {
        AgentData {
            id: agent.get_id(),
            action: agent.get_action(),
            status: agent.get_status(),
            target: agent.get_target(),
            target_id: match agent.get_target() {
                Target::Agent => agent.get_agent_target_id(),
                Target::Monster => agent.get_monster_target_id(),
                Target::Treasure => agent.get_treasure_target_id(),
                _ => u32::MAX,
            },
            tile_target: agent.get_tile_target(),
        }
    }
}

#[allow(dead_code)]
impl GameState {
    pub fn create_gamestate(agents: Vec<Agent>, monsters: Vec<Monster>, treasures: Vec<Treasure>) -> Self {
        let mut agent_map = HashMap::new();
        let mut monster_map = HashMap::new();
        let mut treasure_map = HashMap::new();

        for agent in agents {
            let id = agent.get_id();
            agent_map.insert(id, AgentData::new_agentdata_agent(&agent));
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
        }
    }

    pub fn new() -> Self {
        GameState {
            agents: HashMap::new(),
            monsters: HashMap::new(),
            treasures: HashMap::new(),
        }
    }

    // Getters for individual Agent and Monster
    pub fn get_agent(&self, agent_id: u32) -> Option<&AgentData> {
        self.agents.get(&agent_id)
    }

    pub fn get_monster(&self, monster_id: u32) -> Option<&SimpleMonster> {
        self.monsters.get(&monster_id)
    }

    // Setters for individual Agent and Monster
    pub fn set_agent(&mut self, agent_id: u32, agent_data: AgentData) {
        self.agents.insert(agent_id, agent_data);
    }

    pub fn set_monster(&mut self, monster_id: u32, monster_data: SimpleMonster) {
        self.monsters.insert(monster_id, monster_data);
    }

    // Getters and Setters for the entire HashMaps
    pub fn get_agents(&self) -> &HashMap<u32, AgentData> {
        &self.agents
    }

    pub fn get_monsters(&self) -> &HashMap<u32, SimpleMonster> {
        &self.monsters
    }

    pub fn get_treasures(&self) -> &HashMap<u32, SimpleTreasure> {
        &self.treasures
    }

    pub fn set_agents(&mut self, agents: HashMap<u32, AgentData>) {
        self.agents = agents;
    }

    pub fn set_monsters(&mut self, monsters: HashMap<u32, SimpleMonster>) {
        self.monsters = monsters;
    }

    pub fn set_treasures(&mut self, treasures: HashMap<u32, SimpleTreasure>) {
        self.treasures = treasures;
    }

}


pub fn run_simulation(
    mut world: ResMut<World>,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
){
    //Create MCST
    let tree = mcst::MCTSTree::new();
    


}

