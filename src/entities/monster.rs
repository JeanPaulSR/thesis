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

    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
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