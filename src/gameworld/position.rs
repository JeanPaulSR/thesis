/// Represents a position in a 2D space with x and y coordinates.
#[derive(Clone, Copy, Debug,Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}  

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}