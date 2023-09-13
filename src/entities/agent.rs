use std::fmt;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::errors::MyError;
use crate::mcst::NpcAction;
use crate::movement::find_path;
use crate::{World, AgentMessages, AgentMessage, MessageType};

static mut A_COUNTER: u32 = 0;


#[derive(Clone, Debug, PartialEq)]
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
    status: Status,
    target: Target,
    monster_target_id: u32,
    agent_target_id: u32,
    treasure_target_id: u32,
    tile_target: Option<(u32, u32)>,
    path: Option<Vec<(i32, i32)>>,
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
            action: NpcAction::None,
            status: Status::Idle,
            target: Target::None,
            monster_target_id: u32::MAX,
            agent_target_id: u32::MAX,
            treasure_target_id: u32::MAX,
            tile_target: None::<(u32, u32)>,
            path: None::<Vec<(i32, i32)>>,
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
            monster_target_id: u32::MAX,
            agent_target_id: u32::MAX,
            treasure_target_id: u32::MAX,
            tile_target: None,
            path: None,
            entity: Entity::new(0), // Initialize with a default entity ID
            transform: Transform::default(),
            sprite_bundle: SpriteBundle::default(),
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

        if self.energy == 0 {
            self.set_status(Status::Dead);
        }
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

    // ______      _     _ _      
    // | ___ \    | |   | (_)     
    // | |_/ /   _| |__ | |_  ___ 
    // |  __/ | | | '_ \| | |/ __|
    // | |  | |_| | |_) | | | (__ 
    // \_|   \__,_|_.__/|_|_|\___|

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
                println!("Testing");
                self.move_to(x as f32, y as f32, commands);
                // Call the move_between_tiles function to move the agent to the next position in the path
                world.move_agent(agent_id, x as usize, y as usize, commands)?;
            }
            Ok(())
        } else {
            // If there is no path, return an error
            Err(MyError::PathNotFound)
        }
    }

   

    //Add error handling if the target is gone/dead
    pub fn perform_action(&mut self,
        world: ResMut<World>,
        commands: &mut Commands,
        mut agent_messages: ResMut<AgentMessages>,) -> Result<(), MyError> {
        let current_target = self.target;
        match current_target {
            Target::Agent => {
                match world.get_agent_position(self.agent_target_id) {
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
            Target::Monster => {
                match world.get_monster_position(self.monster_target_id) {
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
            Target::Treasure => {
                match world.get_treasure_position(self.treasure_target_id) {
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
            Target::None => {
                return Err(MyError::InvalidTarget);
            }
            Target::Tile => {
                todo!()
            }
        }
    
        // Check if the agent's current position is equal to the tile target
        let (x, y) = self.get_position();
        if (x, y) == self.tile_target.unwrap_or_default() {
        //     // Continue with action logic
            let action = &self.action;
            //Match the type of action
            match action {
                NpcAction::Attack => {
                    //Match the current target for the Attack action
                    match current_target{
                        //For the target Agent of the Attack action
                        Target::Agent => {
                            let id = self.agent_target_id;
                            self.send_message(
                                id,
                                MessageType::Attack(10),
                                &mut agent_messages,
                            )
                        },
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
                NpcAction::None => todo!(),
            }
            // Clear the action after performing it
            self.status = Status::Idle;
            
            return Ok(()) // Return Ok to indicate success
        } else {
            // If the agent is not at the target position, initiate travel
            self.travel(world, commands)?; 
            self.set_status(Status::Moving);
            return Ok(()) // Return Ok to indicate success
        }
    }


    pub fn print_agent_properties(query: Query<&Agent>) {
        for agent in query.iter() {
            println!("Agent ID: {}", agent.id);
            println!("Genes: {:?}", agent.genes);
            println!("Energy: {}", agent.energy);
            // Print other properties as needed
        }
    }

    pub fn send_message(
        &mut self,
        receiver_id: u32,
        message_content: MessageType,
        agent_messages: &mut AgentMessages,
    ) {
        let message = AgentMessage {
            sender_id: self.id,
            receiver_id,
            message_type: message_content,
        };
        agent_messages.messages.push(message);
    }
    
  
}

