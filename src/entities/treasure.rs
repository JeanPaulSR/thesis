use bevy::prelude::*;

#[derive(Clone)]
pub struct Treasure {
    entity: Entity,
    id: u32,
    transform: Transform,
    sprite_bundle: SpriteBundle,
    reward: u32,
}

static mut T_COUNTER: u32 = 0;

impl Treasure {
    pub fn new_treasure(
        mut x: f32,
        mut y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
        reward: u32
    ) -> Self {
        // Convert x and y to world coordinates
        x = x * 32.0;
        y = y * 32.0;

        // Define the size of the agent's sprite
        let sprite_size = Vec2::new(32.0, 32.0); 
    
        // Load the agent's sprite texture from the asset server
        let texture_handle = asset_server.load("textures/enemy.png");
    
        // Spawn the agent's sprite using the Commands resource and get its entity ID
        let entity = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();

        // Increment the static counter variable after creating a new instance
        unsafe {
            T_COUNTER += 1;
        }
        Treasure {
            id: unsafe { T_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x , y , 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
            reward: 0,
        }
    }
    
    pub fn get_entity(&self) -> Entity {
        self.entity
    }
    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
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

    pub fn get_id(&self) -> u32 {
        self.id
    }

    // ______      _     _ _      
    // | ___ \    | |   | (_)     
    // | |_/ /   _| |__ | |_  ___ 
    // |  __/ | | | '_ \| | |/ __|
    // | |  | |_| | |_) | | | (__ 
    // \_|   \__,_|_.__/|_|_|\___|

    pub fn print(&self) {
        println!("Treasure ID: {}", self.id);
        println!("Position: x={}, y={}", self.transform.translation.x/32.0, self.transform.translation.y/32.0);
    }

}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SimpleTreasure {
    id: u32,
    transform: Transform,
    reward: u32,
}

impl From<&Treasure> for SimpleTreasure {
    fn from(treasure: &Treasure) -> Self {
        SimpleTreasure {
            id: treasure.id,
            transform: treasure.transform.clone(),
            reward: treasure.reward,
        }
    }
}

impl SimpleTreasure {
    pub fn new(treasure: &Treasure) -> Self {
        SimpleTreasure::from(treasure)
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
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

    pub fn get_id(&self) -> u32 {
        self.id
    }
}