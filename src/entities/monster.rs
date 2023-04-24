use bevy::prelude::*;
use crate::components::{Position};
use crate::tile::NPC;
use crate::tile::NPCComponent;
use crate::tile::NPC;

#[derive(Debug, Clone)]
pub struct Monster {
    pub aggression: f32,
    pub health: f32,
    pub damage: f32,
    pub reward: f32,
}

pub fn spawn_monster(
    commands: &mut Commands,
    monster_material: &Handle<ColorMaterial>,
    monster: NPC,
    position: Position,
) {
    let monster_sprite_bundle = SpriteBundle {
        material: monster_material.clone(),
        transform: Transform::from_xyz(
            (position.x as f32) * 32.0,
            (position.y as f32) * 32.0,
            1.0,
        ),
        sprite: Sprite::new(Vec2::new(32.0, 32.0)),
        ..Default::default()
    };
    commands
        .spawn_bundle(monster_sprite_bundle)
        .insert(position)
        .insert(NPCComponent { npc: monster });
}