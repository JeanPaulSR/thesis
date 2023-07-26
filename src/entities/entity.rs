use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::mcst::NpcAction;

static mut NPC_COUNTER: u32 = 0;

pub struct Npc {
    pub npc: Npc,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
    pub id: u32,
}

impl Npc {
    pub fn new(
        x: f32,
        y: f32,
        texture_handle: Handle<ColorMaterial>,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let sprite_size = Vec2::new(32.0, 32.0); // Adjust to your sprite size

        let npc = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();

        unsafe {
            NPC_COUNTER += 1;
        }

        Npc {
            npc,
            id: unsafe { NPC_COUNTER },
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust position in relation to the npc transform
                ..Default::default()
            },
        }
    }

    pub fn travel(&mut self, x: f32, y: f32, commands: &mut Commands) {
        let new_transform = Transform::from_translation(Vec3::new(x * 32.0, y * 32.0, 1.0));
        self.transform = new_transform;
        commands.npc(self.npc).insert(self.transform.clone());
    }

    pub fn print(&self) {
        println!("Npc ID: {}", self.id);
        println!("Position: x={}, y={}", self.transform.translation.x / 32.0, self.transform.translation.y / 32.0);
    }
}