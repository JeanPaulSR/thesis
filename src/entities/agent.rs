use std::fmt;

use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::errors::MyError;
use crate::mcst::NpcAction;
use crate::movement::find_path;
use crate::World;

static mut A_COUNTER: u32 = 0;


#[derive(Clone, Debug)]
pub enum Status {
    Idle,
    Finished,
    Working,
    Moving,
    Dead,
}

#[derive(Clone, Copy, Debug)]
pub enum Target{
    Agent,
    Monster, 
    None,
    Tile,
    Treasure,
}

// Define the Genes struct
#[derive(Clone, Debug)]
pub struct Genes {
    pub greed: f32,
    pub aggression: f32,
    pub social: f32,
    pub self_preservation: f32,
    pub vision: f32,
}

impl Genes{
    pub fn generate() -> Self{
    
        // Initialize a random number generator
        let mut rng = rand::thread_rng();
    
        // Define distribution ranges for agent attributes
        let greed_distribution = Uniform::new(0.5, 1.0);
        let aggression_distribution = Uniform::new(0.3, 0.8);
        let common_distribution = Uniform::new(0.0, 1.0);
        let vision_distribution = Uniform::new(3.0, 8.0);
        
        Genes {
            greed: greed_distribution.sample(&mut rng),
            aggression: aggression_distribution.sample(&mut rng),
            social: common_distribution.sample(&mut rng),
            self_preservation: common_distribution.sample(&mut rng),
            vision: vision_distribution.sample(&mut rng),
        }
    }

}

// Modify the Agent struct to include the Genes field
#[derive(Clone)]
pub struct Agent {
    pub entity: Entity,
    pub genes: Genes,
    pub energy: u8,
    pub max_energy: u8,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
    pub action: Option<NpcAction>,
    pub id: u32,
    pub status: Status,
    pub target: Target,
    pub monster_target_id: Option<u32>,
    pub agent_target_id: Option<u32>,
    pub treasure_target_id: Option<u32>,
    pub tile_target: Option<(u32, u32)>,
    pub path: Option<Vec<(i32, i32)>>,
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
    ) -> Self {
        // Convert x and y to world coordinates
        x = x * 32.0;
        y = y * 32.0;
    
        // Define the size of the agent's sprite
        let sprite_size = Vec2::new(32.0, 32.0); 
    
        // Load the agent's sprite texture from the asset server
        let texture_handle = asset_server.load("textures/agent.png");
    
        // Spawn the agent's sprite using the Commands resource and get its entity ID
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
    
        // Create a new instance of the Genes struct with random attribute values
        
    
        // Create and return a new instance of the Agent struct
        Agent {
            genes : Genes::generate(),
            id: unsafe { A_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x , y , 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
            energy: 100,
            max_energy: 100,
            action: None::<NpcAction>,
            status: Status::Idle,
            target: Target::None,
            monster_target_id: None::<u32>,
            agent_target_id: None::<u32>,
            treasure_target_id: None::<u32>,
            tile_target: None::<(u32, u32)>,
            path: None::<Vec<(i32, i32)>>,
        }
    }

    pub fn print(&self) {
        println!("Agent ID: {}", self.id);
        println!("Greed: {}", self.genes.greed);
        println!("Aggression: {}", self.genes.aggression);
        println!("Social: {}", self.genes.social);
        println!("Self Preservation: {}", self.genes.self_preservation);
        println!("Vision: {}", self.genes.vision);
        println!("Position: x={}, y={}", self.transform.translation.x/32.0, self.transform.translation.y/32.0);
    }

    // Function to move the agent to a specific position
    pub fn move_to(
        &mut self,
        y: f32,
        x: f32,
        commands: &mut Commands,
    ) {
        let new_transform = Transform::from_translation(Vec3::new(y * 32.0, x * 32.0, 1.0));
        self.transform = new_transform;
        commands.entity(self.entity).insert(self.transform.clone());
    }

    // Updated travel function to use the path
    pub fn travel(
        &mut self,
        world: ResMut<World>,
        commands: &mut Commands,
    ) -> Result<(), MyError> {
        // Check if the agent's current position is equal to the tile target
        if self.get_position() == self.tile_target.unwrap_or_default() {
            // Agent is already at the target tile
            return Ok(());
        }
    
        // Create the path if it's missing or empty
        if self.path.is_none() || self.path.as_ref().unwrap().is_empty() {
            self.path = find_path(
                &world,
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
    
        // Check if there is a path available
        if let Some(path) = &mut self.path {
            if let Some((x, y)) = path.pop() {
                // Get the agent's ID
                let agent_id = self.id;
    
                // Call the move_between_tiles function to move the agent to the next position in the path
                world.move_agent(agent_id, x as usize, y as usize, commands)?;
            }
            Ok(())
        } else {
            // If there is no path, return an error
            Err(MyError::PathNotFound)
        }
    }

    pub fn get_position(&self) -> (u32, u32) {
        (
            (self.transform.translation.x / 32.0) as u32,
            (self.transform.translation.y / 32.0) as u32,
        )
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

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn _set_action(&mut self, action: NpcAction) {
        self.action = Some(action);
    }

    //Add error handling if the target is gone/dead
    fn _perform_action(&mut self, world: ResMut<World>, commands: &mut Commands) -> Result<(), MyError> {
        let current_target = self.target;
        match current_target {
            Target::Agent => {
                if let Some(agent_id) = self.agent_target_id {
                    match world.get_agent_position(agent_id) {
                        Ok(agent_position) => {
                            let (x, y) = agent_position;
                            self.tile_target = Some((x as u32, y as u32));
                        }
                        Err(MyError::AgentNotFound) => {
                            return Err(MyError::AgentNotFound);
                        }
                        _ => {} // Handle other errors if needed
                    }
                }
            }
            Target::Monster => {
                if let Some(monster_id) = self.monster_target_id {
                    match world.get_monster_position(monster_id) {
                        Ok(monster_position) => {
                            let (x, y) = monster_position;
                            self.tile_target = Some((x as u32, y as u32));
                        }
                        Err(MyError::MonsterNotFound) => {
                            return Err(MyError::MonsterNotFound);
                        }
                        _ => {} // Handle other errors if needed
                    }
                }
            }
            Target::Treasure => {
                if let Some(treasure_id) = self.treasure_target_id {
                    match world.get_treasure_position(treasure_id) {
                        Ok(treasure_position) => {
                            let (x, y) = treasure_position;
                            self.tile_target = Some((x as u32, y as u32));
                        }
                        Err(MyError::TreasureNotFound) => {
                            return Err(MyError::TreasureNotFound);
                        }
                        _ => {} // Handle other errors if needed
                    }
                }
            }
            Target::None => {
                return Err(MyError::InvalidTarget);
            }
            Target::Tile => {
                // Do nothing for Target::Tile
            }
        }
    
        // Check if the agent's current position is equal to the tile target
        if self.get_position() == self.tile_target.unwrap_or_default() {
            // Continue with action logic
            if let Some(ref action) = self.action {
                match action {
                    NpcAction::Attack => {
                        match current_target{
                            Target::Agent => todo!(),
                            Target::Monster => todo!(),
                            Target::None => todo!(),
                            Target::Tile => todo!(),
                            Target::Treasure => todo!(),
                        }
                        // Attack formula
                        // Agents have 3 lives
                        // Every time an agent attacks something they lose a life
                    }
                    NpcAction::Steal => {
                        // Logic for moving to a treasure
                    }
                    NpcAction::Rest => {
                        // Logic for moving to a monster
                    }
                    NpcAction::Talk => todo!(),
                }
                // Clear the action after performing it
                self.status = Status::Idle;
            }
            Ok(()) // Return Ok to indicate success
        } else {
            // If the agent is not at the target position, initiate travel
            self.travel(world, commands)?; 
            self.set_status(Status::Moving);
            Ok(()) // Return Ok to indicate success
        }
    }


}

