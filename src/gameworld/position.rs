use bevy::ecs::component::Component;

/// Represents a position in a 2D space with x and y coordinates.
#[derive(Clone, Component, Copy, Debug, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    /// Creates a new position with the given x and y coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    /// Returns the x-coordinate of the position.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Returns the y-coordinate of the position.
    pub fn get_y(&self) -> i32 {
        self.y
    }
    
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}