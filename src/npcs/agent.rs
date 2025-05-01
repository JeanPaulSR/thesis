use crate::gameworld::position::Position;
use crate::npcs::npc_components::npc_action::NpcAction;
use bevy::prelude::*;
use std::collections::HashMap;
use std::{fmt, u32};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use super::npc_components::gene_type::GeneType;
use super::npc_components::genes::Genes;
use super::npc_components::npc_status::Status;
use super::npc_components::npc_type::NPCType;
use super::npc_components::opinions::Opinions;
use super::npc_components::target::Target;

static A_COUNTER: AtomicI32 = AtomicI32::new(0);


#[derive(Clone, Component, Resource)]
#[allow(dead_code)]
pub struct Agent {
    genes: Genes,
    opinions: Opinions,
    action: NpcAction,
    id: i32,
    reward: u32,
    status: Status,
    target: Target,
    retaliation_target: Target,
    retaliation_target_id: i32,
    monster_target_id: i32,
    agent_target_id: i32,
    treasure_target_id: i32,
    tile_target: Option<Position>,
    path: Option<Vec<Position>>,
    leader: bool,
    follower: bool,
    leader_id: i32,
    followers: Vec<i32>,
    memory: Vec<(i32, NpcAction)>, // New memory field
}

impl fmt::Debug for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Agent")
            .field("id", &self.id)
            .field("genes", &self.genes)
            .field("action", &self.action)
            .field("target", &self.target)
            .field("tile_target", &self.tile_target)
            .finish()
    }
}

impl Agent {
    //     _____                _                   _
    //     / ____|              | |                 | |
    //    | |     ___  _ __  ___| |_ _ __ _   _  ___| |_ ___  _ __
    //    | |    / _ \| '_ \/ __| __| '__| | | |/ __| __/ _ \| '__|
    //    | |___| (_) | | | \__ \ |_| |  | |_| | (__| || (_) | |
    //     \_____\___/|_| |_|___/\__|_|   \__,_|\___|\__\___/|_|

    pub fn new_agent() -> Agent {

        // Create the Agent component
        let agent = Agent {
            genes: Genes::generate(),
            opinions: Opinions {
                opinion_scores: Arc::new(Mutex::new(HashMap::new())),
            },
            id: A_COUNTER.fetch_add(1, Ordering::SeqCst),
            reward: 0,
            action: NpcAction::None,
            status: Status::Idle,
            target: Target::None,
            retaliation_target: Target::None,
            retaliation_target_id: i32::MAX,
            monster_target_id: i32::MAX,
            agent_target_id: i32::MAX,
            treasure_target_id: i32::MAX,
            tile_target: None,
            path: None,
            leader: true,
            follower: false,
            leader_id: i32::MAX,
            followers: Vec::new(),
            memory: Vec::new(), // Initialize memory field
        };

        agent
    }

    #[allow(dead_code)]
    pub fn default() -> Self {

        Agent {
            genes: Genes::generate(),
            opinions: Opinions {
                opinion_scores: Arc::new(Mutex::new(HashMap::new())), // Wrap HashMap in Arc<Mutex<>>
            },
            id: 0,
            reward: 0,
            action: NpcAction::None,
            status: Status::Idle,
            target: Target::None,
            retaliation_target: Target::None,
            retaliation_target_id: i32::MAX,
            monster_target_id: i32::MAX,
            agent_target_id: i32::MAX,
            treasure_target_id: i32::MAX,
            tile_target: None,
            path: None,
            leader: false,
            follower: false,
            leader_id: 0,
            followers: Vec::new(),
            memory: Vec::new(), // Initialize memory field
        }
    }

    //   ________        __        /\   _________       __
    //  /  _____/  _____/  |_     / /  /   _____/ _____/  |_
    // /   \  ____/ __ \   __\   / /   \_____  \_/ __ \   __\
    // \    \_\  \  ___/|  |    / /     ______\ \  ___/|  |
    //  \______  /\___  >__|   / /    /_______  /\___  >__|
    //         \/     \/       \/             \/     \/



    pub fn get_genes(&self) -> &Genes {
        &self.genes
    }

    pub fn set_genes(&mut self, genes: Genes) {
        self.genes = genes;
    }

    pub fn get_opinions(&self) -> &Opinions {
        &self.opinions
    }

    pub fn set_opinions(&mut self, opinions: Opinions) {
        self.opinions = opinions;
    }

    pub fn modify_opinion(&mut self, id: i32, amount: f32) {
        let mut opinion_scores = self.opinions.opinion_scores.lock().unwrap();
        let current_opinion = opinion_scores.entry(id).or_insert(0.5);
        *current_opinion += amount;
        if *current_opinion > 1.0 {
            *current_opinion = 1.0;
        } else if *current_opinion < 0.0 {
            *current_opinion = 0.0;
        }
    }

    pub fn get_agent_opinion(&self, id: i32) -> f32 {
        let opinion_scores = self.opinions.opinion_scores.lock().unwrap();
        *opinion_scores.get(&id).unwrap_or(&0.5)
    }

    pub fn influence_opinions(&mut self, influencing_opinions: Opinions) {
        let influencing_opinion_scores = influencing_opinions.opinion_scores.lock().unwrap();
        let mut current_opinion_scores = self.opinions.opinion_scores.lock().unwrap();
        for (&id, &influencing_opinion) in influencing_opinion_scores.iter() {
            let influence = if influencing_opinion > 0.5 {
                0.1 * (influencing_opinion - 0.5)
            } else {
                -0.1 * (0.5 - influencing_opinion)
            };
            let current_opinion = current_opinion_scores.entry(id).or_insert(0.5);
            *current_opinion = (*current_opinion + influence).clamp(0.0, 1.0);
        }
    }


    pub fn get_reward(&self) -> u32 {
        self.reward
    }

    pub fn add_reward(&mut self, reward: u32) {
        self.reward = self.reward + reward;
    }

    pub fn remove_reward(&mut self, reward: u32) {
        self.reward = self.reward.saturating_sub(reward);
    }


    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_target(&self) -> Target {
        self.target
    }

    pub fn set_target(&mut self, target: Target) {
        self.target = target;
    }

    pub fn get_retaliation_target(&self) -> Target {
        self.retaliation_target
    }

    pub fn set_retaliation_target(&mut self, target: Target) {
        self.retaliation_target = target;
    }

    pub fn get_retaliation_target_id(&self) -> i32 {
        self.retaliation_target_id
    }

    pub fn set_retaliation_target_id(&mut self, retaliation_target_id: i32) {
        self.retaliation_target_id = retaliation_target_id;
    }

    pub fn get_agent_target_id(&self) -> i32 {
        self.agent_target_id
    }

    pub fn set_agent_target_id(&mut self, agent_target_id: i32) {
        self.agent_target_id = agent_target_id;
    }

    pub fn get_monster_target_id(&self) -> i32 {
        self.monster_target_id
    }

    pub fn set_monster_target_id(&mut self, monster_target_id: i32) {
        self.monster_target_id = monster_target_id;
    }

    pub fn get_treasure_target_id(&self) -> i32 {
        self.treasure_target_id
    }

    pub fn set_treasure_target_id(&mut self, treasure_target_id: i32) {
        self.treasure_target_id = treasure_target_id;
    }

    pub fn get_tile_target(&self) -> Option<Position> {
        self.tile_target
    }

    pub fn set_tile_target(&mut self, tile_target: Option<Position>) {
        self.tile_target = tile_target;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn get_action(&self) -> NpcAction {
        self.action.clone()
    }

    pub fn set_action(&mut self, action: NpcAction) {
        self.action = action;
    }

    pub fn get_path(&self) -> Option<Vec<Position>> {
        self.path.clone()
    }

    pub fn set_path(&mut self, new_path: Vec<Position>) {
        self.path = Some(new_path);
    }

    pub fn is_leader(&self) -> bool {
        self.leader
    }

    pub fn set_is_leader(&mut self, is_leader: bool) {
        self.leader = is_leader;
    }

    pub fn get_leader_id(&self) -> i32 {
        self.leader_id
    }

    pub fn set_leader_id(&mut self, leader_id: i32) {
        self.leader_id = leader_id;
    }

    pub fn is_follower(&self) -> bool {
        self.follower
    }

    pub fn set_is_follower(&mut self, is_follower: bool) {
        self.follower = is_follower;
    }

    pub fn add_follower(&mut self, followers: Vec<i32>) {
        self.followers.extend(followers.clone());
    }

    pub fn remove_follower(&mut self, follower_id: i32) {
        if let Some(index) = self.followers.iter().position(|&id| id == follower_id) {
            self.followers.remove(index);
        }
    }

    pub fn has_followers(&self) -> bool {
        !self.followers.is_empty()
    }

    pub fn get_followers(&self) -> Vec<i32> {
        self.followers.clone()
    }

    pub fn get_group_size(&self) -> i32 {
        self.get_followers().len() as i32
    }

    // ______      _     _ _
    // | ___ \    | |   | (_)
    // | |_/ /   _| |__ | |_  ___
    // |  __/ | | | '_ \| | |/ __|
    // | |  | |_| | |_) | | | (__
    // \_|   \__,_|_.__/|_|_|\___|

    pub fn print_information(&self) {
        println!("Agent ID: {}", self.id);

        // Lock the Mutex to access the HashMap
        let gene_scores = self.genes.gene_scores.lock().unwrap();

        if let Some(greed) = gene_scores.get(&GeneType::Greed) {
            println!("Greed: {}", greed);
        }
        if let Some(aggression) = gene_scores.get(&GeneType::Aggression) {
            println!("Aggression: {}", aggression);
        }
        if let Some(social) = gene_scores.get(&GeneType::Social) {
            println!("Social: {}", social);
        }
        if let Some(self_preservation) = gene_scores.get(&GeneType::SelfPreservation) {
            println!("Self Preservation: {}", self_preservation);
        }
        if let Some(vision) = gene_scores.get(&GeneType::Vision) {
            println!("Vision: {}", vision);
        }

    }

    pub fn print_status(&self) {
        match self.status {
            Status::Idle => println!("Agent {} is idle.", self.id),
            Status::Finished => println!("Agent {} has finished its task.", self.id),
            Status::Working => println!("Agent {} is currently working.", self.id),
            Status::Moving => println!("Agent {} is moving.", self.id),
            Status::Dead => println!("Agent {} is dead.", self.id),
            Status::Following => println!("Agent {} is following.", self.id),
            Status::Retaliating => println!("Agent {} is Retaliating.", self.id),
            Status::Fleeing => println!("Agent {} is Fleeing.", self.id),
            Status::Recovering => println!("Agent {} is Recovering.", self.id),
            Status::Attacking => println!("Agent {} is Attacking.", self.id),
            Status::RequiresInstruction => println!("Agent {} is Awaiting Instructions.", self.id),
            Status::Talking => println!("Agent {} is Talking.", self.id),
        }
    }

    pub fn print_target(&self) {
        match self.target {
            Target::Agent => println!("Agent {} is targeting another agent.", self.id),
            Target::Monster => println!("Agent {} is targeting a monster.", self.id),
            Target::None => println!("Agent {} has no specific target.", self.id),
            Target::Tile => {
                if let Some(tile_target) = self.tile_target {
                    println!(
                        "Agent {} is targeting a tile at coordinates ({}, {}).",
                        self.id, tile_target.x, tile_target.y
                    );
                } else {
                    println!("Agent {} is not targeting any specific tile.", self.id);
                }
            }
            Target::Treasure => println!("Agent {} is targeting treasure.", self.id),
        }
    }
    
    pub fn print_agent_properties(query: Query<&Agent>) {
        for agent in query.iter() {
            println!("Agent ID: {}", agent.id);
            println!("Genes: {:?}", agent.genes);
        }
    }

    pub fn find_best_agent(&self) -> Option<i32> {
        let opinion_scores = self.opinions.opinion_scores.lock().unwrap();
        opinion_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&id, _)| id as i32)
    }

    pub fn find_worst_agent(&self) -> Option<i32> {
        let opinion_scores = self.opinions.opinion_scores.lock().unwrap();
        opinion_scores
            .iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&id, _)| id as i32)
    }

    // Add a new entry to the memory
    pub fn add_to_memory(&mut self, agent_id: i32, action: NpcAction) {
        // Add the new entry to the front of the vector
        self.memory.insert(0, (agent_id, action));

        // Ensure the memory does not exceed 50 items
        if self.memory.len() > 50 {
            self.memory.pop(); // Remove the last item if the limit is exceeded
        }
    }

    // Retrieve the memory
    pub fn get_memory(&self) -> &Vec<(i32, NpcAction)> {
        &self.memory
    }
    
}

// True means flight, false means fight
pub fn flight_or_fight(agent: &Agent, npc_type: NPCType, target_id: Option<i32>) -> bool {
    let genes = agent.get_genes();
    let gene_scores = genes.gene_scores.lock().unwrap();
    let aggression = *gene_scores.get(&GeneType::Aggression).unwrap_or(&0.0);
    let self_preservation = *gene_scores.get(&GeneType::SelfPreservation).unwrap_or(&0.0);
    let flight = aggression * self_preservation;
    let followers_opinion = if agent.has_followers() {
        let followers = agent.get_followers();
        let total_opinion: f32 = followers
            .iter()
            .map(|follower_id| agent.get_agent_opinion(*follower_id))
            .sum();
        total_opinion / followers.len() as f32
    } else {
        1.0
    };
    let target_opinion = if npc_type == NPCType::Agent {
        target_id.map_or(1.0, |id| agent.get_agent_opinion(id))
    } else {
        1.0
    };

    let fight = ((aggression * target_opinion) + followers_opinion) / 2.0;
    flight > fight
}
