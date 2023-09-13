use std::collections::HashMap;
use crate::tile::{TileType, Tile};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub fn find_path(grid: Vec<Vec<Tile>>, start_pos: (i32, i32), end_pos: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    open_set.push(Node {
        position: start_pos,
        g_score: 0,
        h_score: heuristic_cost_estimate(start_pos, end_pos),
    });

    g_score.insert(start_pos, 0);

    while let Some(current_node) = open_set.pop() {
        if current_node.position == end_pos {
            return Some(reconstruct_path(came_from, end_pos));
        }

        let (x, y) = current_node.position;

        for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_x >= grid[0].len() as i32 || new_y < 0 || new_y >= grid.len() as i32 {
                continue;
            }

            let new_tile = &grid[new_y as usize][new_x as usize];
            let new_tile_type = new_tile.get_tile_type();
            let new_weight = match new_tile_type {
                TileType::Forest | TileType::Village | TileType::Dungeon => 1,
                TileType::Lake | TileType::Mountain => 10000,
            };

            let tentative_g_score = current_node.g_score + new_weight;

            if let Some(&g) = g_score.get(&(new_x, new_y)) {
                if tentative_g_score >= g {
                    continue;
                }
            }

            came_from.insert((new_x, new_y), current_node.position);
            g_score.insert((new_x, new_y), tentative_g_score);

            open_set.push(Node {
                position: (new_x, new_y),
                g_score: tentative_g_score,
                h_score: heuristic_cost_estimate((new_x, new_y), end_pos),
            });
        }
    }

    None
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    position: (i32, i32),
    g_score: i32,
    h_score: i32,
}

impl Node {
    fn f_score(&self) -> i32 {
        self.g_score + self.h_score
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score().cmp(&self.f_score())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic_cost_estimate(start: (i32, i32), end: (i32, i32)) -> i32 {
    let dx = (start.0 - end.0).abs();
    let dy = (start.1 - end.1).abs();
    dx + dy
}

fn reconstruct_path(came_from: HashMap<(i32, i32), (i32, i32)>, current: (i32, i32)) -> Vec<(i32, i32)> {
    let mut path = vec![current];
    let mut current = current;
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path.remove(0);
    path.reverse();
    path
}
