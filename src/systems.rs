use bevy::prelude::*;
use crate::components::{Position};
use crate::tile::TileType;
use crate::debug::debug;
use std::collections::HashSet;
use crate::WorldGrid;
    

pub fn debug_system(_time: Res<Time>) {
    // Call the debug function with a command string
    debug("test_movement");
}

pub fn get_tiles_in_range2(
    x: i32,
    y: i32,
    vision_range: i32,
    world_grid: &Res<WorldGrid>,
) -> Vec<Position> {
    let mut tiles_in_range = HashSet::new();
    let max_distance = vision_range as f32 * 32.0;
    for (ty, row) in world_grid.grid.iter().enumerate() {
        for (tx, _) in row.iter().enumerate() {
            let distance = ((tx as i32 - x).pow(2) + (ty as i32 - y).pow(2)) as f32;
            if distance <= max_distance.powi(2) {
                tiles_in_range.insert(Position { x: tx as i32, y: ty as i32 });
            }
        }
    }
    tiles_in_range.into_iter().collect()
}

use std::f32::consts::PI;
/*
pub fn find_tiles_in_range(x: i32, y: i32, vision_range: f32
    world_grid: Res<WorldGrid>,) -> Vec<Entity> {
    let mut tiles = Vec::new();

    // Iterate over all tiles in the grid
    for (row_index, row) in world_grid.grid.iter().enumerate() {
        for (col_index, _) in row.iter().enumerate() {
            let row = row_index as i32;
            let col = col_index as i32;

            // Calculate the distance between the current tile and the agent's position
            let distance = ((col - x).pow(2) + (row - y).pow(2)) as f32;
            let distance = distance.sqrt();

            // If the tile is within the agent's vision range, add it to the result
            if distance <= vision_range {
                let tile_entity = world_grid[row as usize][col as usize];
                tiles.push(tile_entity);
            }
        }
    }

    tiles
}*/