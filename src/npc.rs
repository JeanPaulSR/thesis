use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};



#[derive(Clone)]
pub struct Monster {
    pub entity: Entity,
    pub id: u32,
    pub vision: f32,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
}

static mut M_COUNTER: u32 = 0;
static mut A_COUNTER: u32 = 0;

impl Monster {
    pub fn new_monster(
        x: f32,
        y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let vision_distribution = Uniform::new(2.0, 4.0);

        let sprite_size = Vec2::new(32.0, 32.0); // Adjust to your sprite size

        let texture_handle = asset_server.load("textures/enemy.png");

        let entity = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();

        // Increment the static counter variable after creating a new instance
        unsafe {
            M_COUNTER += 1;
        }

        Monster {
            vision: vision_distribution.sample(&mut rng),
            // Set the id of the monster to the current value of the counter
            id: unsafe { M_COUNTER },
            entity: entity,
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
        }
    }

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
    pub id: u32,
}

impl Agent {
    pub fn new_agent(
        x: f32,
        y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
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
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
        }
    }

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

    
}

pub fn agent_test(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn an agent using the `new_random` function
    let mut agent = Agent::new_agent(5.0 * 32.0, 5.0 * 32.0, &mut commands, &mut materials, &asset_server);
    
    let agent2 = Agent::new_agent(5.0 * 32.0, 6.0 * 32.0, &mut commands, &mut materials, &asset_server);
    agent.travel(2.0, 3.0, &mut commands);
    //update_agent_transform(&mut agent, new_transform, &mut commands);
    
    let mut monster = Monster::new_monster(3.0 * 32.0, 3.0 * 32.0, &mut commands, &mut materials, &asset_server);
    monster.travel(7.0, 1.0, &mut commands);
}
