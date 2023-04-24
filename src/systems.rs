use bevy::prelude::*;
use crate::components::{Position};
use crate::tile::TileType;
use crate::debug::debug;
    

pub fn debug_system(_time: Res<Time>) {
    // Call the debug function with a command string
    debug("test_movement");
}

