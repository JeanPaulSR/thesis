use std::error::Error;
use std::fmt;

// Enum to encompass all possible errors
#[derive(Debug)]
pub enum MyError {
    PositionError,
    TileNotFound,
    AgentNotFound,
    MonsterNotFound,
    TreasureNotFound,
    OutOfBounds,
    PathNotFound,
    UnecessaryMove,
    InvalidTarget,
    OtherError,
}

// Implement the Error trait for the custom error enum
impl Error for MyError {}

// Implement the Display trait for the custom error enum
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::PositionError => write!(f, "Position Error"),
            MyError::TileNotFound => write!(f, "Tile not found"),
            MyError::AgentNotFound => write!(f, "Agent not found"),
            MyError::MonsterNotFound => write!(f, "Monster not found"),
            MyError::TreasureNotFound => write!(f, "Treasure not found"),
            MyError::OutOfBounds => write!(f, "Treasure not found"),
            MyError::PathNotFound => write!(f, "Path not found"),
            MyError::UnecessaryMove => write!(f, "Unecessary move"),
            MyError::InvalidTarget=> write!(f, "Invalid Target"),
            MyError::OtherError => write!(f, "Other Error"),
        }
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct PositionError;

// Implement the Error trait for the custom error type
impl Error for PositionError {}

// Implement the Display trait for the custom error type
impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position is out of bounds")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct TileNotFound;

// Implement the Error trait for the custom error type
impl Error for TileNotFound {}

// Implement the Display trait for the custom error type
impl fmt::Display for TileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile not found")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct AgentNotFound;

// Implement the Error trait for the custom error type
impl Error for AgentNotFound {}

// Implement the Display trait for the custom error type
impl fmt::Display for AgentNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Agent not found")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct MonsterNotFound;

// Implement the Error trait for the custom error type
impl Error for MonsterNotFound {}

// Implement the Display trait for the custom error type
impl fmt::Display for MonsterNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monster not found")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct TreasureNotFound;

// Implement the Error trait for the custom error type
impl Error for TreasureNotFound {}

// Implement the Display trait for the custom error type
impl fmt::Display for TreasureNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Treasure not found")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct OutOfBounds;

// Implement the Error trait for the custom error type
impl Error for OutOfBounds {}

// Implement the Display trait for the custom error type
impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out of bounds")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct PathNotFound;

// Implement the Error trait for the custom error type
impl Error for PathNotFound {}

// Implement the Display trait for the custom error type
impl fmt::Display for PathNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct UnecessaryMove;

// Implement the Error trait for the custom error type
impl Error for UnecessaryMove {}

// Implement the Display trait for the custom error type
impl fmt::Display for UnecessaryMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No need to move")
    }
}

// Define a custom error type for out-of-bounds positions
#[derive(Debug)]
pub struct InvalidTarget;

// Implement the Error trait for the custom error type
impl Error for InvalidTarget {}

// Implement the Display trait for the custom error type
impl fmt::Display for InvalidTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Target is invalid")
    }
}