use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::errors::MyError;
use crate::mcst::NpcAction;
use crate::{World, movement};

static mut A_COUNTER: u32 = 0;


#[derive(Clone)]
pub enum Status {
    Idle,
    Finished,
    Working,
    Dead,
}

// Define the Genes struct
#[derive(Clone)]
pub struct Genes {
    pub greed: f32,
    pub aggression: f32,
    pub social: f32,
    pub self_preservation: f32,
    pub vision: f32,
}

// Modify the Agent struct to include the Genes field
#[derive(Clone)]
pub struct Agent {
    pub entity: Entity,
    pub genes: Genes,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
    pub action: Option<NpcAction>,
    pub id: u32,
    pub status: Status,
    pub monster_target_id: Option<u32>,
    pub agent_target_id: Option<u32>,
    pub treasure_target_id: Option<u32>,
    pub tile_target_id: Option<u32>,
    pub path: Option<Vec<(i32, i32)>>,
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
    
        // Initialize a random number generator
        let mut rng = rand::thread_rng();
    
        // Define distribution ranges for agent attributes
        let greed_distribution = Uniform::new(0.5, 1.0);
        let aggression_distribution = Uniform::new(0.3, 0.8);
        let common_distribution = Uniform::new(0.0, 1.0);
        let vision_distribution = Uniform::new(3.0, 8.0);
    
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
        let genes = Genes {
            greed: greed_distribution.sample(&mut rng),
            aggression: aggression_distribution.sample(&mut rng),
            social: common_distribution.sample(&mut rng),
            self_preservation: common_distribution.sample(&mut rng),
            vision: vision_distribution.sample(&mut rng),
        };
    
        // Create and return a new instance of the Agent struct
        Agent {
            genes,
            id: unsafe { A_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x , y , 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
            action: None::<NpcAction>,
            status: Status::Idle,
            monster_target_id: None::<u32>,
            agent_target_id: None::<u32>,
            treasure_target_id: None::<u32>,
            tile_target_id: None::<u32>,
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
        // Check if there is a path available
        if let Some(path) = &mut self.path {
            // If the path is not empty, pop the first position and move the agent to that position
            if let Some((x, y)) = path.pop() {
                // Get the agent's ID
                let agent_id = self.id;

                // Call the move_between_tiles function to move the agent to the next position in the path
                world.move_between_tiles(agent_id, x as usize, y as usize, commands)?;
            } else {
                // If the path is empty, clear it to indicate that the agent has reached its destination
                self.path = None;
            }
            Ok(())
        } else {
            // If there is no path, return an error
            Err(MyError::PathNotFound)
        }
    }
    // if let Some(path) = find_path(world, self.get_current_position(), self.get_destination_position()) {
    //     // Update the agent's path with the returned path
    //     self.path = Some(path);

    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
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
