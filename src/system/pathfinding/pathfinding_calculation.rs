use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use crate::gameworld::tile_types::TileType;
use crate::gameworld::world::GameWorld;
use crate::gameworld::position::Position;

/// A* pathfinding algorithm
pub fn a_star_pathfinding(
    world: &GameWorld,
    start: Position,
    goal: Position,
) -> Vec<Position> {
    // Priority queue for open nodes
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        position: start,
        cost: 0.0,
        estimated_total_cost: heuristic(start, goal),
    });

    // Maps to store the cost of reaching a node and the path to it
    let mut g_score: HashMap<Position, f32> = HashMap::new();
    g_score.insert(start, 0.0);

    let mut came_from: HashMap<Position, Position> = HashMap::new();

    while let Some(current) = open_set.pop() {
        // If we reached the goal, reconstruct the path
        if current.position == goal {
            return reconstruct_path(came_from, current.position);
        }

        // Get neighbors of the current position
        for neighbor in get_neighbors(world, current.position) {
            // Extract x and y from the neighbor position
            let (x, y) = (neighbor.x, neighbor.y);

            // Get the tile type for the neighbor
            if let Some(tile_type) = world.get_tile_type(x, y) {
                let travel_weight = tile_type.get_travel_weight();

                // Skip impassable tiles
                if travel_weight == 0.0 {
                    continue;
                }

                let tentative_g_score = g_score.get(&current.position).unwrap_or(&f32::INFINITY)
                    + travel_weight;

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                    // Update the path and cost
                    came_from.insert(neighbor, current.position);
                    g_score.insert(neighbor, tentative_g_score);

                    open_set.push(Node {
                        position: neighbor,
                        cost: tentative_g_score,
                        estimated_total_cost: tentative_g_score + heuristic(neighbor, goal),
                    });
                }
            }
        }
    }

    // Return an empty path if no path is found
    Vec::new()
}

/// A* pathfinding algorithm that reuses a current path
pub fn a_star_with_current_path(
    world: &GameWorld,
    current_path: Vec<Position>,
    goal: Position,
) -> Vec<Position> {
    // Use the first position in the current path as the starting point
    let start = match current_path.first() {
        Some(&position) => position,
        None => return Vec::new(),
    };

    // Check if the current path is valid and can be reused
    let mut reusable_path = Vec::new();
    if let Some(index) = current_path.iter().position(|&pos| pos == start) {
        reusable_path = current_path[index..].to_vec();
        if let Some(last_position) = reusable_path.last() {
            if *last_position == goal {
                // If the current path already reaches the goal, return it
                return reusable_path;
            }
        }
    }

    // If the path is invalid or incomplete, recalculate from the last valid position
    let recalculate_start = reusable_path.last().cloned().unwrap_or(start);

    // Priority queue for open nodes
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        position: recalculate_start,
        cost: 0.0,
        estimated_total_cost: heuristic(recalculate_start, goal),
    });

    // Maps to store the cost of reaching a node and the path to it
    let mut g_score: HashMap<Position, f32> = HashMap::new();
    g_score.insert(recalculate_start, 0.0);

    let mut came_from: HashMap<Position, Position> = HashMap::new();

    while let Some(current) = open_set.pop() {
        // If we reached the goal, reconstruct the path
        if current.position == goal {
            let recalculated_path = reconstruct_path(came_from, current.position);
            reusable_path.pop(); // Remove the last position to avoid duplication
            reusable_path.extend(recalculated_path);
            return reusable_path;
        }

        // Get neighbors of the current position
        for neighbor in get_neighbors(world, current.position) {
            let travel_weight = world.get_tile_type(neighbor.x, neighbor.y)
                .map_or(f32::INFINITY, |tile| tile.get_travel_weight());

            // Skip impassable tiles
            if travel_weight == 0.0 {
                continue;
            }

            let tentative_g_score = g_score.get(&current.position).unwrap_or(&f32::INFINITY)
                + travel_weight;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                // Update the path and cost
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);

                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    estimated_total_cost: tentative_g_score + heuristic(neighbor, goal),
                });
            }
        }
    }

    // Return an empty path if no path is found
    Vec::new()
}

/// Recalculate path around a position
/// Important issue with moving around multiple monsters in a row
pub fn recalculate_path_around_position(
    world: &GameWorld,
    current_path: Vec<Position>,
    avoid_position: Position,
    avoid_radius: i32,
    goal: Position,
) -> Vec<Position> {
    // Use the first position in the current path as the starting point
    let start = match current_path.first() {
        Some(&position) => position,
        None => return Vec::new(), // Return an empty path if the current path is empty
    };

    // Create a set of positions to avoid
    let mut avoid_positions = HashSet::new();
    for dx in -avoid_radius..=avoid_radius {
        for dy in -avoid_radius..=avoid_radius {
            let neighbor = Position {
                x: avoid_position.x + dx,
                y: avoid_position.y + dy,
            };
            if world.is_within_bounds(neighbor) {
                avoid_positions.insert(neighbor);
            }
        }
    }

    // If the starting position is within the avoid area, find a valid position outside
    let mut adjusted_start = start;
    if avoid_positions.contains(&start) {
        let mut radius = 1;
        let mut found_valid_position = false;

        while !found_valid_position {
            let mut candidates = Vec::new();

            // Check all positions in the current radius
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let candidate = Position {
                        x: start.x + dx,
                        y: start.y + dy,
                    };

                    // Ensure the candidate is valid, outside the avoid area, and moves away from the center
                    if world.is_within_bounds(candidate)
                        && !avoid_positions.contains(&candidate)
                        && (candidate.x - avoid_position.x).abs() >= (start.x - avoid_position.x).abs()
                        && (candidate.y - avoid_position.y).abs() >= (start.y - avoid_position.y).abs()
                    {
                        candidates.push(candidate);
                    }
                }
            }

            // If valid candidates are found, select the one closest to the start
            if !candidates.is_empty() {
                adjusted_start = candidates
                    .into_iter()
                    .min_by_key(|pos| {
                        let dx = (pos.x - start.x).abs();
                        let dy = (pos.y - start.y).abs();
                        dx + dy
                    })
                    .unwrap();
                found_valid_position = true;
            } else {
                radius += 1; // Increase the radius and try again
            }

            // Safety check to prevent infinite loops
            if radius > avoid_radius * 2 {
                return Vec::new(); // Return an empty path if no valid position is found
            }
        }
    }

    // If the adjusted start is still within the avoid area, calculate a path out of the area
    if avoid_positions.contains(&adjusted_start) {
        let path_out = a_star_pathfinding(world, start, adjusted_start);
        if path_out.is_empty() {
            return Vec::new(); // Return an empty path if no valid path out of the area is found
        }

        // Start A* from the last position in the path out to the goal
        let final_path = a_star_pathfinding(world, *path_out.last().unwrap(), goal);
        return [path_out, final_path].concat();
    }

    // If the adjusted start is valid, calculate the path to the goal
    a_star_pathfinding(world, adjusted_start, goal)
}


//WHAT HAPPENS IF TARGET IS IN AREA
pub fn recalculate_path_around_multiple_positions(
    world: &GameWorld,
    current_path: Vec<Position>,
    avoid_positions: Vec<(Position, i32)>, // List of positions with their respective avoid radii
    goal: Position,
) -> Vec<Position> {
    // Use the first position in the current path as the starting point
    let start = match current_path.first() {
        Some(&position) => position,
        None => return Vec::new(), // Return an empty path if the current path is empty
    };

    // Create a set of all positions to avoid
    let mut avoid_set = HashSet::new();
    for (avoid_position, avoid_radius) in &avoid_positions {
        for dx in -avoid_radius..=*avoid_radius {
            for dy in -avoid_radius..=*avoid_radius {
                let neighbor = Position {
                    x: avoid_position.x + dx,
                    y: avoid_position.y + dy,
                };
                if world.is_within_bounds(neighbor) {
                    avoid_set.insert(neighbor);
                }
            }
        }
    }

    // If the starting position is within the avoid area, find a valid position outside
    let mut adjusted_start = start;
    if avoid_set.contains(&start) {
        let mut radius = 1;
        let mut found_valid_position = false;

        while !found_valid_position {
            let mut candidates = Vec::new();

            // Check all positions in the current radius
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let candidate = Position {
                        x: start.x + dx,
                        y: start.y + dy,
                    };

                    // Ensure the candidate is valid, outside the avoid area, and moves away from all avoid centers
                    if world.is_within_bounds(candidate)
                        && !avoid_set.contains(&candidate)
                        && avoid_positions.iter().all(|(avoid_position, _)| {
                            (candidate.x - avoid_position.x).abs() >= (start.x - avoid_position.x).abs()
                                && (candidate.y - avoid_position.y).abs() >= (start.y - avoid_position.y).abs()
                        })
                    {
                        candidates.push(candidate);
                    }
                }
            }

            // If valid candidates are found, select the one closest to the start
            if !candidates.is_empty() {
                adjusted_start = candidates
                    .into_iter()
                    .min_by_key(|pos| {
                        let dx = (pos.x - start.x).abs();
                        let dy = (pos.y - start.y).abs();
                        dx + dy
                    })
                    .unwrap();
                found_valid_position = true;
            } else {
                radius += 1; // Increase the radius and try again
            }

            // Safety check to prevent infinite loops
            if radius > avoid_positions.iter().map(|(_, r)| r).max().unwrap_or(&1) * 2 {
                return Vec::new(); // Return an empty path if no valid position is found
            }
        }
    }

    // If the adjusted start is still within the avoid area, calculate a path out of the area
    if avoid_set.contains(&adjusted_start) {
        let path_out = a_star_pathfinding(world, start, adjusted_start);
        if path_out.is_empty() {
            return Vec::new(); // Return an empty path if no valid path out of the area is found
        }

        // Start A* from the last position in the path out to the goal
        let final_path = a_star_pathfinding(world, *path_out.last().unwrap(), goal);
        return [path_out, final_path].concat();
    }

    // If the adjusted start is valid, calculate the path to the goal
    a_star_pathfinding(world, adjusted_start, goal)
}

/// Heuristic function for A* (Manhattan distance)
fn heuristic(a: Position, b: Position) -> f32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32
}

/// Reconstruct the path from the `came_from` map
fn reconstruct_path(
    mut came_from: HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = Vec::new();
    while let Some(&prev) = came_from.get(&current) {
        path.push(current); // Add the current position to the path
        current = prev;
    }
    path.reverse(); // Reverse the path to get it from start to goal
    path
}

/// Get valid neighbors of a position, including diagonals
fn get_neighbors(world: &GameWorld, position: Position) -> Vec<Position> {
    let directions = [
        Position { x: 0, y: 1 },   // Up
        Position { x: 1, y: 0 },   // Right
        Position { x: 0, y: -1 },  // Down
        Position { x: -1, y: 0 },  // Left
        Position { x: 1, y: 1 },   // Up-Right (Diagonal)
        Position { x: 1, y: -1 },  // Down-Right (Diagonal)
        Position { x: -1, y: -1 }, // Down-Left (Diagonal)
        Position { x: -1, y: 1 },  // Up-Left (Diagonal)
    ];

    directions
        .iter()
        .map(|dir| Position {
            x: position.x + dir.x,
            y: position.y + dir.y,
        })
        .filter(|neighbor| world.is_within_bounds(*neighbor))
        .collect()
}

/// Node structure for the priority queue
#[derive(Debug, Clone, PartialEq)]
struct Node {
    position: Position,
    cost: f32,
    estimated_total_cost: f32,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_total_cost
            .partial_cmp(&self.estimated_total_cost)
            .unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}