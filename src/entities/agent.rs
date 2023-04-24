use bevy::prelude::*;
use crate::components::{Position};
use crate::tile::NPC;
use crate::tile::NPCComponent;
use crate::tile::NPC;

#[derive(Debug, Clone)]
pub struct Agent {
    pub greed: f32,
    pub preservation: f32,
    pub social: f32,
    pub reproduction: f32,
    pub hate: f32,
    pub exploration: f32,
    pub aggression: f32,
    pub adaptability: f32,
    pub health: f32,
    pub fame: f32,
}

pub fn spawn_agent(
    commands: &mut Commands,
    agent_material: &Handle<ColorMaterial>,
    agent: NPC,
    position: Position,
) {
    let agent_sprite_bundle = SpriteBundle {
        material: agent_material.clone(),
        transform: Transform::from_xyz(
            (position.x as f32) * 32.0,
            (position.y as f32) * 32.0,
            1.0,
        ),
        sprite: Sprite::new(Vec2::new(32.0, 32.0)),
        ..Default::default()
    };
    commands
        .spawn_bundle(agent_sprite_bundle)
        .insert(position)
        .insert(NPCComponent { npc: agent });
}