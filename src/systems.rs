use bevy::prelude::*;
use crate::components::{Position};
use crate::debug::debug;
use std::collections::HashSet;
use crate::World;
    

pub fn debug_system(_time: Res<Time>) {
    // Call the debug function with a command string
    debug("test_movement");
}

pub fn _get_tiles_in_range2(
    x: i32,
    y: i32,
    vision_range: i32,
    world: &Res<World>,
) -> Vec<Position> {
    let mut tiles_in_range = HashSet::new();
    let max_distance = vision_range as f32 * 32.0;
    for (ty, row) in world.grid.iter().enumerate() {
        for (tx, _) in row.iter().enumerate() {
            let distance = ((tx as i32 - x).pow(2) + (ty as i32 - y).pow(2)) as f32;
            if distance <= max_distance.powi(2) {
                tiles_in_range.insert(Position { x: tx as i32, y: ty as i32 });
            }
        }
    }
    tiles_in_range.into_iter().collect()
}
