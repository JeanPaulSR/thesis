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
    // Function to create a treasure
    pub fn new_treasure(
        mut x: f32,
        mut y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
        _reward: u32,
    ) -> Self {
        // Convert x and y to world coordinates
        x = x * 32.0;
        y = y * 32.0;

        let sprite_size = Vec2::new(32.0, 32.0);
        let texture_handle = asset_server.load("textures/treasure.png");

        let entity = commands
            .spawn(SpriteBundle {
                texture: texture_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                ..Default::default()
            })
            .id();

        unsafe {
            T_COUNTER += 1;
        }
        Treasure {
            id: unsafe { T_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite_bundle: SpriteBundle {
                texture: texture_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            },
            reward: 0,
        }
    }

    // Function to get the treasures entity
    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    // Function to get the position of the treasure
    pub fn get_position(&self) -> (f32, f32) {
        (
            self.transform.translation.x / 32.0,
            self.transform.translation.y / 32.0,
        )
    }

    // Function to set the reward of the treasure
    pub fn set_reward(&mut self, reward: u32) {
        self.reward = reward;
    }

    // Function to get the reward of the treasure
    pub fn get_reward(&self) -> u32 {
        self.reward
    }

    // Function to add reward to the treasure
    pub fn add_reward(&mut self, reward: u32) {
        self.reward = self.reward + reward;
    }

    // Function to remove reward from the treasure
    pub fn remove_reward(&mut self, reward: u32) {
        self.reward = self.reward.saturating_sub(reward);
    }

    // Function to get the id of the treasure
    pub fn get_id(&self) -> u32 {
        self.id
    }

    // ______      _     _ _
    // | ___ \    | |   | (_)
    // | |_/ /   _| |__ | |_  ___
    // |  __/ | | | '_ \| | |/ __|
    // | |  | |_| | |_) | | | (__
    // \_|   \__,_|_.__/|_|_|\___|

    // Function to print the treasure
    pub fn print(&self) {
        println!("Treasure ID: {}", self.id);
        println!(
            "Position: x={}, y={}",
            self.transform.translation.x / 32.0,
            self.transform.translation.y / 32.0
        );
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
        (
            self.transform.translation.x / 32.0,
            self.transform.translation.y / 32.0,
        )
    }

    pub fn set_reward(&mut self, reward: u32) {
        self.reward = reward;
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

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
