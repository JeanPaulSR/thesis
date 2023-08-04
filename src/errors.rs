use std::error::Error;
use std::fmt;



// Enum to encompass all possible errors
#[derive(Debug)]
pub enum MyError {
    PositionError,
    OtherError,
    // Add more error variants here
}

// Implement the Error trait for the custom error enum
impl Error for MyError {}

// Implement the Display trait for the custom error enum
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::PositionError => write!(f, "Position Error"),
            MyError::OtherError => write!(f, "Other Error"),
            // Add more cases for other error variants
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