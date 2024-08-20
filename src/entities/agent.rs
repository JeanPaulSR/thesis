use crate::errors::MyError;
use crate::mcst_system::mcst::NpcAction;
use crate::movement::find_path;
use crate::tile::Tile;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

static A_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Idle,
    Finished,
    Working,
    Moving,
    Dead,
    Following,
}

#[derive(Clone, Copy, Debug)]
pub enum Target {
    Agent,
    Monster,
    None,
    Tile,
    Treasure,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GeneType {
    Greed,
    Aggression,
    Social,
    SelfPreservation,
    Vision,
}

#[derive(Clone, Debug)]
pub struct Genes {
    pub gene_scores: Arc<Mutex<HashMap<GeneType, f32>>>,
}
impl Genes {
    pub fn new(gene_scores: HashMap<GeneType, f32>) -> Self {
        Genes {
            gene_scores: Arc::new(Mutex::new(gene_scores)),
        }
    }

    pub fn return_type_score(&self, gene_type: GeneType) -> f32 {
        let gene_scores = self.gene_scores.lock().unwrap();
        match gene_scores.get(&gene_type) {
            Some(result) => *result,
            None => todo!(),
        }
    }

    pub fn generate() -> Self {
        use rand::distributions::{Distribution, Uniform};

        // Initialize a random number generator
        let mut rng = rand::thread_rng();

        // Define distribution ranges for agent attributes
        let greed_distribution = Uniform::new(0.5, 1.0);
        let aggression_distribution = Uniform::new(0.3, 0.8);
        let common_distribution = Uniform::new(0.0, 1.0);
        let vision_distribution = Uniform::new(3.0, 8.0);

        // Generate random values for each attribute
        let mut gene_scores = HashMap::new();
        gene_scores.insert(GeneType::Greed, greed_distribution.sample(&mut rng));
        gene_scores.insert(
            GeneType::Aggression,
            aggression_distribution.sample(&mut rng),
        );
        gene_scores.insert(GeneType::Social, common_distribution.sample(&mut rng));
        gene_scores.insert(
            GeneType::SelfPreservation,
            common_distribution.sample(&mut rng),
        );
        gene_scores.insert(GeneType::Vision, vision_distribution.sample(&mut rng));

        Genes {
            gene_scores: Arc::new(Mutex::new(gene_scores)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Opinions {
    pub opinion_scores: Arc<Mutex<HashMap<u32, f32>>>,
}

#[derive(Clone, Component, Resource)]
#[allow(dead_code)]
pub struct Agent {
    entity: Entity,
    genes: Genes,
    opinions: Opinions,
    energy: u8,
    max_energy: u8,
    transform: Transform,
    sprite_bundle: SpriteBundle,
    action: NpcAction,
    id: u32,
    reward: u32,
    status: Status,
    target: Target,
    monster_target_id: u32,
    agent_target_id: u32,
    treasure_target_id: u32,
    tile_target: Option<(u32, u32)>,
    path: Option<Vec<(i32, i32)>>,
    leader: bool,
    follower: bool,
    leader_id: u32,
    followers: Vec<u32>,
}

impl fmt::Debug for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Agent")
            .field("id", &self.id)
            .field("genes", &self.genes)
            .field("energy", &self.energy)
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

    pub fn new_agent(
        x: f32,
        y: f32,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>, // Use TextureAtlas
    ) -> Self {
        let x = x * 32.0;
        let y = y * 32.0;

        let sprite_size = Vec2::new(32.0, 32.0);

        let texture_handle = asset_server.load("textures/agent.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle.clone(), // Texture handle
            sprite_size,            // Size of each sprite
            1,                      // Number of columns
            1,                      // Number of rows
            None,                   // Optional padding
            None,                   // Optional spacing
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let entity = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                ..Default::default()
            })
            .id();

        let id = A_COUNTER.fetch_add(1, Ordering::SeqCst);

        Agent {
            genes: Genes::generate(),
            opinions: Opinions {
                opinion_scores: Arc::new(Mutex::new(HashMap::new())),
            },
            id,
            entity,
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite_bundle: SpriteBundle {
                texture: texture_handle.clone(), // Use texture instead of material
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            },
            energy: 100,
            max_energy: 100,
            reward: 0,
            action: NpcAction::None,
            status: Status::Idle,
            target: Target::None,
            monster_target_id: u32::MAX,
            agent_target_id: u32::MAX,
            treasure_target_id: u32::MAX,
            tile_target: None,
            path: None,
            leader: false,
            follower: false,
            leader_id: 0,
            followers: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Agent {
            genes: Genes::generate(),
            opinions: Opinions {
                opinion_scores: Arc::new(Mutex::new(HashMap::new())), // Wrap HashMap in Arc<Mutex<>>
            },
            id: 0,
            energy: 100,
            max_energy: 100,
            action: NpcAction::None,
            status: Status::Idle,
            target: Target::None,
            reward: 0,
            monster_target_id: u32::MAX,
            agent_target_id: u32::MAX,
            treasure_target_id: u32::MAX,
            tile_target: None,
            path: None,
            entity: Entity::PLACEHOLDER, // Use placeholder or default value
            transform: Transform::default(),
            sprite_bundle: SpriteBundle::default(),
            leader: false,
            follower: false,
            leader_id: 0,
            followers: Vec::new(),
        }
    }

    //   ________        __        /\   _________       __
    //  /  _____/  _____/  |_     / /  /   _____/ _____/  |_
    // /   \  ____/ __ \   __\   / /   \_____  \_/ __ \   __\
    // \    \_\  \  ___/|  |    / /     ______\ \  ___/|  |
    //  \______  /\___  >__|   / /    /_______  /\___  >__|
    //         \/     \/       \/             \/     \/

    pub fn get_entity(&self) -> Entity {
        self.entity
    }
    pub fn get_position(&self) -> (u32, u32) {
        (
            (self.transform.translation.x / 32.0) as u32,
            (self.transform.translation.y / 32.0) as u32,
        )
    }

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

    pub fn get_energy(&self) -> u8 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u8) {
        self.energy = energy;
    }

    pub fn add_energy(&mut self, energy: u8) {
        let new_energy = self.energy.saturating_add(energy);
        self.energy = new_energy.min(self.max_energy);
    }

    pub fn remove_energy(&mut self, energy: u8) {
        self.energy = self.energy.saturating_sub(energy);

        if self.energy <= 0 {
            self.set_status(Status::Dead);
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

    pub fn get_max_energy(&self) -> u8 {
        self.max_energy
    }

    pub fn set_max_energy(&mut self, max_energy: u8) {
        self.max_energy = max_energy;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_target(&self) -> Target {
        self.target
    }

    pub fn set_target(&mut self, target: Target) {
        self.target = target;
    }

    pub fn get_agent_target_id(&self) -> u32 {
        self.agent_target_id
    }

    pub fn set_agent_target_id(&mut self, agent_target_id: u32) {
        self.agent_target_id = agent_target_id;
    }

    pub fn get_monster_target_id(&self) -> u32 {
        self.monster_target_id
    }

    pub fn set_monster_target_id(&mut self, monster_target_id: u32) {
        self.monster_target_id = monster_target_id;
    }

    pub fn get_treasure_target_id(&self) -> u32 {
        self.treasure_target_id
    }

    pub fn set_treasure_target_id(&mut self, treasure_target_id: u32) {
        self.treasure_target_id = treasure_target_id;
    }

    pub fn get_tile_target(&self) -> Option<(u32, u32)> {
        self.tile_target
    }

    pub fn set_tile_target(&mut self, tile_target: Option<(u32, u32)>) {
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

    pub fn get_path(&self) -> Option<Vec<(i32, i32)>> {
        self.path.clone()
    }

    pub fn set_path(&mut self, new_path: Vec<(i32, i32)>) {
        self.path = Some(new_path);
    }

    pub fn is_leader(&self) -> bool {
        self.leader
    }

    pub fn set_is_leader(&mut self, is_leader: bool) {
        self.leader = is_leader;
    }

    pub fn get_leader_id(&self) -> u32 {
        self.leader_id
    }

    pub fn set_leader_id(&mut self, leader_id: u32) {
        self.leader_id = leader_id;
    }

    pub fn is_follower(&self) -> bool {
        self.follower
    }

    pub fn set_is_follower(&mut self, is_follower: bool) {
        self.follower = is_follower;
    }

    pub fn add_follower(&mut self, followers: Vec<u32>) {
        self.followers.extend(followers.clone());
    }

    pub fn remove_follower(&mut self, follower_id: u32) {
        if let Some(index) = self.followers.iter().position(|&id| id == follower_id) {
            self.followers.remove(index);
        }
    }

    pub fn get_followers(&self) -> Vec<u32> {
        self.followers.clone()
    }

    pub fn get_group_size(&self) -> u32 {
        self.get_followers().len() as u32
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

        println!(
            "Position: x={}, y={}",
            self.transform.translation.x / 32.0,
            self.transform.translation.y / 32.0
        );
    }

    pub fn print_status(&self) {
        match self.status {
            Status::Idle => println!("Agent {} is idle.", self.id),
            Status::Finished => println!("Agent {} has finished its task.", self.id),
            Status::Working => println!("Agent {} is currently working.", self.id),
            Status::Moving => println!("Agent {} is moving.", self.id),
            Status::Dead => println!("Agent {} is dead.", self.id),
            Status::Following => println!("Agent {} is following.", self.id),
        }
    }

    pub fn print_target(&self) {
        match self.target {
            Target::Agent => println!("Agent {} is targeting another agent.", self.id),
            Target::Monster => println!("Agent {} is targeting a monster.", self.id),
            Target::None => println!("Agent {} has no specific target.", self.id),
            Target::Tile => {
                if let Some((x, y)) = self.tile_target {
                    println!(
                        "Agent {} is targeting a tile at coordinates ({}, {}).",
                        self.id, x, y
                    );
                } else {
                    println!("Agent {} is not targeting any specific tile.", self.id);
                }
            }
            Target::Treasure => println!("Agent {} is targeting treasure.", self.id),
        }
    }

    // Function to move the agent to a specific position
    pub fn move_to(&mut self, x: f32, y: f32, commands: &mut Commands) {
        let new_transform = Transform::from_translation(Vec3::new(x * 32.0, y * 32.0, 1.0));
        self.transform = new_transform;
        commands.entity(self.entity).insert(self.transform.clone());
    }

    // Updated travel function to use the path
    pub fn travel(&mut self, grid: Vec<Vec<Tile>>, commands: &mut Commands) -> Result<(), MyError> {
        if self.get_position() == self.tile_target.unwrap_or_default() {
            return Ok(());
        }

        if self.path.is_none() || self.path.as_ref().unwrap().is_empty() {
            self.path = find_path(
                grid,
                (self.get_position().0 as i32, self.get_position().1 as i32),
                (
                    self.tile_target.unwrap_or_default().0 as i32,
                    self.tile_target.unwrap_or_default().1 as i32,
                ),
            );
        }

        if let Some(path) = &mut self.path {
            if let Some((x, y)) = path.pop() {
                self.move_to(x as f32, y as f32, commands);
            }
            Ok(())
        } else {
            Err(MyError::PathNotFound)
        }
    }

    pub fn print_agent_properties(query: Query<&Agent>) {
        for agent in query.iter() {
            println!("Agent ID: {}", agent.id);
            println!("Genes: {:?}", agent.genes);
            println!("Energy: {}", agent.energy);
        }
    }

    pub fn calculate_best_agent(&self, action: NpcAction, agent_ids: &Vec<u32>) -> u32 {
        match action {
            NpcAction::AttackAgent | NpcAction::Steal | NpcAction::Talk => {
                let mut best_score = f32::MIN;
                let mut best_agent_id = u32::MAX;

                // Lock the Mutex to access the HashMap
                let opinion_scores = self.opinions.opinion_scores.lock().unwrap(); // Handle the possibility of poisoning in real code

                for &agent_id in agent_ids {
                    let opinion_score = opinion_scores.get(&agent_id).cloned().unwrap_or(1.0);

                    let total_score = match action {
                        NpcAction::AttackAgent => self.calculate_attack_score(
                            opinion_score,
                            self.genes.return_type_score(GeneType::Aggression),
                        ),
                        NpcAction::Steal => self.calculate_steal_score(
                            opinion_score,
                            self.genes.return_type_score(GeneType::Greed),
                        ),
                        NpcAction::Talk => self.calculate_talk_score(
                            opinion_score,
                            self.genes.return_type_score(GeneType::Social),
                        ),
                        _ => 0.0,
                    };

                    if total_score > best_score {
                        best_score = total_score;
                        best_agent_id = agent_id;
                    }
                }

                best_agent_id
            }
            _ => u32::MAX,
        }
    }

    fn calculate_gene_score(&self, genes: &Genes) -> f32 {
        // Placeholder for gene score calculation
        1.0
    }

    fn calculate_attack_score(&self, opinion_score: f32, gene_score: f32) -> f32 {
        // Placeholder for attack score calculation formula
        opinion_score * gene_score
    }

    fn calculate_steal_score(&self, opinion_score: f32, gene_score: f32) -> f32 {
        // Placeholder for steal score calculation formula
        opinion_score * gene_score
    }

    fn calculate_talk_score(&self, opinion_score: f32, gene_score: f32) -> f32 {
        // Placeholder for talk score calculation formula
        opinion_score * gene_score
    }
}
