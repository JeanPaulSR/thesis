use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

use super::agent::Status;

#[derive(Clone)]
pub struct Monster {
    entity: Entity,
    id: u32,
    vision: u8,
    energy: u8,
    max_energy: u8,
    reward: u32,
    status: Status,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
}

static mut M_COUNTER: u32 = 0;

impl Monster {
    // Function to create the monster
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct SimpleMonster {
    id: u32,
    vision: u8,
    energy: u8,
    max_energy: u8,
    reward: u32,
    status: Status,
    transform: Transform,
}

impl From<&Monster> for SimpleMonster {
    fn from(monster: &Monster) -> Self {
        SimpleMonster {
            id: monster.id,
            vision: monster.vision,
            energy: monster.energy,
            max_energy: monster.max_energy,
            reward: monster.reward,
            status: monster.status.clone(),
            transform: monster.transform.clone(),
        }
    }
}

impl SimpleMonster {
    pub fn new(monster: &Monster) -> Self {
        SimpleMonster::from(monster)
    }

    
    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
    }

    pub fn get_energy(&self) -> u8 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u8) {
        self.energy = energy;
    }

    pub fn get_vision(&self) -> u8 {
        self.vision
    }

    pub fn set_vision(&mut self, vision: u8) {
        self.vision = vision;
    }


    pub fn add_energy(&mut self, energy: u8) {
        let new_energy = self.energy.saturating_add(energy); 
        self.energy = new_energy.min(self.max_energy); 
    }

    pub fn remove_energy(&mut self, energy: u8) {
        self.energy = self.energy.saturating_sub(energy);

        if self.energy == 0 {
            self.set_status(Status::Dead);
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_max_energy(&self) -> u8 {
        self.max_energy
    }

    pub fn set_max_energy(&mut self, max_energy: u8) {
        self.max_energy = max_energy;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn set_reward(&mut self, reward: u32) {
        self.reward = reward;
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
}