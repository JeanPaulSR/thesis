use std::fmt;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::errors::MyError;
use crate::mcst::NpcAction;
use crate::movement::find_path;
use crate::tile::Tile;
use std::collections::HashMap;

static mut A_COUNTER: u32 = 0;


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
pub enum Target{
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
    pub gene_scores: HashMap<GeneType, f32>,
}

impl Genes {
    pub fn new(gene_scores: HashMap<GeneType, f32>) -> Self {
        Genes { gene_scores }
    }

    pub fn return_type_score(&self, gene_type : GeneType) -> f32{
        match self.gene_scores.get(&gene_type){
            Some(result) => *result,
            None => todo!(),
        }
    }

    pub fn generate() -> Self {
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
        gene_scores.insert(GeneType::Aggression, aggression_distribution.sample(&mut rng));
        gene_scores.insert(GeneType::Social, common_distribution.sample(&mut rng));
        gene_scores.insert(GeneType::SelfPreservation, common_distribution.sample(&mut rng));
        gene_scores.insert(GeneType::Vision, vision_distribution.sample(&mut rng));

        Genes { gene_scores }
    }
}


#[derive(Clone, Bundle)]
#[allow(dead_code)]
pub struct Agent {
    entity: Entity,
    genes: Genes,
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
        mut x: f32,
        mut y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self{
        x = x * 32.0;
        y = y * 32.0;
    
        let sprite_size = Vec2::new(32.0, 32.0); 
    
        let texture_handle = asset_server.load("textures/agent.png");
    
        let entity = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();
    
        unsafe {
            A_COUNTER += 1;
        }
        
        let tile_target = (10 as u32, 10 as u32);
        Agent {
            genes : Genes::generate(),
            id: unsafe { A_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x , y , 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
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
            tile_target: None::<(u32, u32)>,
            path: None::<Vec<(i32, i32)>>,
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
            entity: Entity::new(0),
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
        self.reward  = self.reward + reward; 
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

    pub fn get_path(&self) -> Option<Vec<(i32, i32)>>{
        self.path.clone()
    }
    
    pub fn is_leader(&self) -> bool{
        self.leader
    }

    pub fn set_is_leader(&mut self, is_leader: bool){
        self.leader = is_leader;
    }

    pub fn get_leader_id(&self) -> u32{
        self.leader_id
    }

    pub fn set_leader_id(&mut self, leader_id: u32){
        self.leader_id = leader_id;
    }

    pub fn is_follower(&self) -> bool{
        self.follower
    }

    pub fn set_is_follower(&mut self, is_follower: bool){
        self.follower = is_follower;
    }

    pub fn add_follower(
        &mut self,
        followers: Vec<u32>,
    ) {
        self.followers.extend(followers.clone());
    }

    pub fn remove_follower(
        &mut self,
        follower_id: u32,
    ) {
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
        if let Some(greed) = self.genes.gene_scores.get(&GeneType::Greed) {
            println!("Greed: {}", greed);
        }
        if let Some(aggression) = self.genes.gene_scores.get(&GeneType::Aggression) {
            println!("Aggression: {}", aggression);
        }
        if let Some(social) = self.genes.gene_scores.get(&GeneType::Social) {
            println!("Social: {}", social);
        }
        if let Some(self_preservation) = self.genes.gene_scores.get(&GeneType::SelfPreservation) {
            println!("Self Preservation: {}", self_preservation);
        }
        if let Some(vision) = self.genes.gene_scores.get(&GeneType::Vision) {
            println!("Vision: {}", vision);
        }
        println!("Position: x={}, y={}", self.transform.translation.x / 32.0, self.transform.translation.y / 32.0);
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
                    println!("Agent {} is targeting a tile at coordinates ({}, {}).", self.id, x, y);
                } else {
                    println!("Agent {} is not targeting any specific tile.", self.id);
                }
            }
            Target::Treasure => println!("Agent {} is targeting treasure.", self.id),
        }
    }

    // Function to move the agent to a specific position
    pub fn move_to(
        &mut self,
        x: f32,
        y: f32,
        commands: &mut Commands,
    ) {
        let new_transform = Transform::from_translation(Vec3::new(x * 32.0, y * 32.0, 1.0));
        self.transform = new_transform;
        commands.entity(self.entity).insert(self.transform.clone());
    }

    // Updated travel function to use the path
    pub fn travel(
        &mut self,
        grid: Vec<Vec<Tile>>,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        if self.get_position() == self.tile_target.unwrap_or_default() {
            return Ok(());
        }
    
        if self.path.is_none() || self.path.as_ref().unwrap().is_empty() {
            self.path = find_path(
                grid,
                (
                    self.get_position().0 as i32,
                    self.get_position().1 as i32,
                ),
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

}