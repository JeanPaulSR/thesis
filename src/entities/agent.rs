use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::mcst::NpcAction;
use crate::tile::Tile;
use crate::entities::treasure::Treasure;

use super::monster::Monster;

static mut A_COUNTER: u32 = 0;



#[derive(Clone)]
pub enum Status {
    Idle,
    Finished,
    Working,
    Dead,
}

#[derive(Clone)]
pub struct Agent {
    pub entity: Entity,
    pub greed: f32,
    pub aggression: f32,
    pub social: f32,
    pub self_preservation: f32,
    pub vision: f32,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
    pub action: Option<NpcAction>,
    pub id: u32,
    pub status: Status,
    pub monster_target: Option<Monster>,
    pub agent_target_id: Option<u32>,
    pub treasure_target: Option<Treasure>,
    pub tile_target: Option<Tile>,
    pub path: Option<Vec<(i32, i32)>>,
}


//Test
impl Agent {
    pub fn new_agent(
        mut x: f32,
        mut y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        x = x * 32.0;
        y = y * 32.0;
        let mut rng = rand::thread_rng();
        let greed_distribution = Uniform::new(0.5, 1.0);
        let aggression_distribution = Uniform::new(0.3, 0.8);
        let common_distribution = Uniform::new(0.0, 1.0);
        let vision_distribution = Uniform::new(3.0, 8.0);

        let sprite_size = Vec2::new(32.0, 32.0); // Adjust to your sprite size

        let texture_handle = asset_server.load("textures/agent.png");

        let entity = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();

        // Increment the static counter variable after creating a new instance
        unsafe {
            A_COUNTER += 1;
        }

        Agent {
            greed: greed_distribution.sample(&mut rng),
            aggression: aggression_distribution.sample(&mut rng),
            social: common_distribution.sample(&mut rng),
            self_preservation: common_distribution.sample(&mut rng),
            vision: vision_distribution.sample(&mut rng),
            id: unsafe { A_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x , y , 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
            action: None::<NpcAction>,
            status: Status::Idle,
            monster_target: None::<Monster>,
            agent_target_id: None::<u32>,
            treasure_target: None::<Treasure>,
            tile_target: None::<Tile>,
            path: None::<Vec<(i32, i32)>>,
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
        commands: &mut Commands,
    ) -> Result<(), &'static str> {
        // Check if there is a path available
        if let Some(path) = &mut self.path {
            // If the path is not empty, pop the first position and move the agent to that position
            if let Some((x, y)) = path.pop() {
                self.move_to(x as f32, y as f32, commands);
            } else {
                // If the path is empty, clear it to indicate that the agent has reached its destination
                self.path = None;
            }
            Ok(())
        } else {
            // If there is no path, return an error
            Err("Agent has no path to follow.")
        }
    }


    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
    }

    pub fn print(&self) {
        println!("Agent ID: {}", self.id);
        println!("Greed: {}", self.greed);
        println!("Aggression: {}", self.aggression);
        println!("Social: {}", self.social);
        println!("Self Preservation: {}", self.self_preservation);
        println!("Vision: {}", self.vision);
        println!("Position: x={}, y={}", self.transform.translation.x/32.0, self.transform.translation.y/32.0);
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn _set_action(&mut self, action: NpcAction) {
        self.action = Some(action);
    }

    fn _perform_action(&mut self) {
        if let Some(ref action) = self.action {
            match action {
                NpcAction::MoveToVillage => {
                    // Logic for moving to a village
                }
                NpcAction::MoveToTreasure => {
                    // Logic for moving to a treasure
                }
                NpcAction::MoveToMonster => {
                    // Logic for moving to a monster
                }
                NpcAction::MoveToSteal => {
                    // Logic for moving to steal
                }
            }
            // Clear the action after performing it
            self.action = None;
        }
    }

}
