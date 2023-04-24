use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};



#[derive(Debug, Clone)]
pub struct Monster {
    pub id: u32,
    pub vision: f32,
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
}

impl Agent {
    pub fn new_random(
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

        Agent {
            greed: greed_distribution.sample(&mut rng),
            aggression: aggression_distribution.sample(&mut rng),
            social: common_distribution.sample(&mut rng),
            self_preservation: common_distribution.sample(&mut rng),
            vision: vision_distribution.sample(&mut rng),
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

    pub fn set_transform(&mut self, new_transform: Transform, commands: &mut Commands) {
        self.transform = new_transform;
        commands.entity(self.entity).insert_bundle(self.sprite_bundle.clone()).insert(self.transform.clone());
    }

}
