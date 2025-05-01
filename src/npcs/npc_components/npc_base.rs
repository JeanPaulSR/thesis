use bevy::prelude::*;
use crate::{gameworld::position::Position, npcs::{agent::Agent, monster::Monster, treasure::Treasure}};

use super::npc_type::NPCType;

#[derive(Clone, Component, Resource)]
pub struct NPCBase {
    pub entity: Entity,
    pub npc_type: NPCType,
    pub position: Position,
    pub energy: u32,
    pub max_energy: u32,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
}

impl NPCBase {
    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }

    pub fn get_energy(&self) -> u32 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u32) {
        self.energy = energy;
    }

    pub fn add_energy(&mut self, energy: u32) {
        self.energy = (self.energy + energy).min(self.max_energy);
    }

    pub fn remove_energy(&mut self, energy: u32) {
        self.energy = self.energy.saturating_sub(energy);
    }

    pub fn get_max_energy(&self) -> u32 {
        self.max_energy
    }

    pub fn set_max_energy(&mut self, max_energy: u32) {
        self.max_energy = max_energy;
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn get_sprite_bundle(&self) -> &SpriteBundle {
        &self.sprite_bundle
    }

    pub fn set_sprite_bundle(&mut self, sprite_bundle: SpriteBundle) {
        self.sprite_bundle = sprite_bundle;
    }

    pub fn new(
        x: i32,
        y: i32,
        npc_type: NPCType,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let sprite_size = Vec2::new(32.0, 32.0);

        let texture_path = format!("textures/{}.png", npc_type.to_string());
        let texture_handle = asset_server.load(texture_path.as_str());
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle.clone(),
            sprite_size,
            1,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let x_f32 = x as f32 * 32.0;
        let y_f32 = y as f32 * 32.0;

        let entity = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(x_f32, y_f32, 1.0)),
                sprite: TextureAtlasSprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        println!(
            "Created NPCBase Entity: {:?}, Type: {:?}, Position: ({}, {})",
            entity, npc_type, x, y
        );

        NPCBase {
            entity,
            npc_type,
            position: Position::new(x, y),
            energy: 100,
            max_energy: 100,
            transform: Transform::from_translation(Vec3::new(x_f32, y_f32, 1.0)),
            sprite_bundle: SpriteBundle {
                texture: texture_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x_f32, y_f32, 1.0)),
                ..Default::default()
            },
        }
    }

    /// Moves the NPC to a new position and updates the sprite's Transform.
    pub fn move_to(&mut self, new_x: i32, new_y: i32, commands: &mut Commands) {
        // Update the position
        self.position.x = new_x;
        self.position.y = new_y;

        // Update the Transform to match the new position
        let new_transform = Transform::from_translation(Vec3::new(
            new_x as f32 * 32.0,
            new_y as f32 * 32.0,
            1.0,
        ));
        self.transform = new_transform;

        // Update the entity's Transform in the ECS
        commands.entity(self.entity).insert(self.transform.clone());
    }

    /// Retrieves the position of the NPC.
    pub fn get_position(&self) -> Position {
        self.position
    }

    /// Retrieves the associated agent if the NPC is of type `Agent`.
    pub fn get_agent<'a>(
        &self,
        agent_query: &'a Query<&Agent>,
    ) -> Option<&'a Agent> {
        if self.npc_type == NPCType::Agent {
            agent_query.get(self.entity).ok()
        } else {
            None
        }
    }

    /// Retrieves the associated monster if the NPC is of type `Monster`.
    pub fn get_monster<'a>(
        &self,
        monster_query: &'a Query<&Monster>,
    ) -> Option<&'a Monster> {
        if self.npc_type == NPCType::Monster {
            monster_query.get(self.entity).ok()
        } else {
            None
        }
    }

    /// Retrieves the associated treasure if the NPC is of type `Treasure`.
    pub fn get_treasure<'a>(
        &self,
        treasure_query: &'a Query<&Treasure>,
    ) -> Option<&'a Treasure> {
        if self.npc_type == NPCType::Treasure {
            treasure_query.get(self.entity).ok()
        } else {
            None
        }
    }
}
