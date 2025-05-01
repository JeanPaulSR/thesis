#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TileType {
    Forest,
    Mountain,
    Lake,
    Village,
    Dungeon,
    Farm,
    Mine,
}

impl TileType {
    
    ///To string of each tile type

    pub fn to_string(&self) -> &'static str {
        match self {
            TileType::Forest => "Forest",
            TileType::Mountain => "Mountain",
            TileType::Lake => "Water",
            TileType::Village => "Village",
            TileType::Dungeon => "Dungeon",
            TileType::Farm => "Farm",
            TileType::Mine => "Mine",
        }
    }
    pub fn get_travel_weight(&self) -> f32 {
        match self {
            TileType::Forest => 1.25,
            TileType::Village => 1.0,
            TileType::Dungeon => 1.0,
            TileType::Mountain => 1.25,
            TileType::Lake => 0.0,
            TileType::Farm => 1.0,
            TileType::Mine => 1.0,
        }
    }
    
}