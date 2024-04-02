use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

use super::agent::Status;

#[derive(Clone)]
pub struct Monster {
    entity: Entity,
    id: u32,
    start_point: (usize, usize),
    current_point: (usize, usize),
    vision: u8,
    target_id: u32,
    reward: u32,
    status: Status,
    energy: u8,
    max_energy: u8,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
}

static mut M_COUNTER: u32 = 0;

impl Monster {
    pub fn new_monster(
        x: f32,
        y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let vision_distribution = Uniform::new(2, 4);
        let sprite_size = Vec2::new(32.0, 32.0);

        let texture_handle = asset_server.load("textures/enemy.png");

        let entity = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();
        
        unsafe {
            M_COUNTER += 1;
        }

        Monster {
            vision: vision_distribution.sample(&mut rng),
            id: unsafe { M_COUNTER },
            energy: 10,
            max_energy: 10,
            entity,
            reward: 0,
            status: Status::Idle,
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            },
            start_point: (usize::MAX,usize::MAX),
            current_point: (usize::MAX,usize::MAX),
            target_id: u32::MAX,
        }
    }

    // Function to get the entity of the monster
    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    // Function to get the position of the monster
    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
    }
    
    // Function to move the monster to the given position
    pub fn travel(
        &mut self,
        x: f32,
        y: f32,
        commands: &mut Commands,
    ) {
        let new_transform = Transform::from_translation(Vec3::new(x * 32.0, y * 32.0, 1.0));
        self.transform = new_transform;
        commands.entity(self.entity).insert(self.transform.clone());
    }

    // Function to get the energy of the monster
    pub fn get_energy(&self) -> u8 {
        self.energy
    }

    // Function to set the energy of the monster
    pub fn set_energy(&mut self, energy: u8) {
        self.energy = energy;
    }

    // Function to get the vision of the monster
    pub fn get_vision(&self) -> u8 {
        self.vision
    }

    // Function to set the vision of the monster
    pub fn set_vision(&mut self, vision: u8) {
        self.vision = vision;
    }

    // Function to add energy to the monster
    pub fn add_energy(&mut self, energy: u8) {
        let new_energy = self.energy.saturating_add(energy); 
        self.energy = new_energy.min(self.max_energy); 
    }

    // Function to remove energy to the monster
    pub fn remove_energy(&mut self, energy: u8) {
        self.energy = self.energy.saturating_sub(energy);

        if self.energy == 0 {
            self.set_status(Status::Dead);
        }
    }

    // Function to get the id of the monster
    pub fn get_id(&self) -> u32 {
        self.id
    }

    // Function to get the max energy of the monster
    pub fn get_max_energy(&self) -> u8 {
        self.max_energy
    }

    // Function to set the max energy of the monster
    pub fn set_max_energy(&mut self, max_energy: u8) {
        self.max_energy = max_energy;
    }

    // Function to set the status of the monster
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    // Function to get the status of the monster
    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    // Function to set the reward of the monster
    pub fn set_reward(&mut self, reward: u32) {
        self.reward = reward;
    }

    // Function to get the reward of the monster
    pub fn get_reward(&self) -> u32 {
        self.reward
    }

    // Function to add reward to the monster
    pub fn add_reward(&mut self, reward: u32) {
        self.reward  = self.reward + reward; 
    }

    // Function to remove reward to the monster
    pub fn remove_reward(&mut self, reward: u32) {
        self.reward = self.reward.saturating_sub(reward);
    }
    
    
}