#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Forest,
    Mountain,
    Lake,
    Village,
    Dungeon,
}

impl Tile {
    // Function to create a new instance of Tile with the specified tile type
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
        }
    }

    // Function to get the type of the current tile
    pub fn get_tile_type(&self) -> TileType {
        self.tile_type
    }

    // Function to update the type of the current tile
    pub fn update_tile_type(&mut self, new_tile_type: TileType) {
        self.tile_type = new_tile_type;
    }

}


