use bevy::prelude::Component;

use crate::entities::treasure::Treasure;
use crate::gameworld::tile_types::TileType;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct TileComponent {
    pub tile_type: TileType,
}
pub struct TreasureComponent {
    pub treasure: Treasure,
}
