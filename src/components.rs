use crate::tile::TileType;
use crate::entities::treasure::Treasure;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


pub struct TileComponent {
    pub tile_type: TileType,
}
pub struct TreasureComponent {
    pub treasure: Treasure,
}