use std::collections::{ HashSet, HashMap};


use crate::tile::TileType;
use crate::World;

#[derive(Eq, Hash, Debug)]
pub struct Node {
    pub pos: (i32, i32),
    pub g_score: i32,
    pub h_score: i32,
    pub f_score: i32,
    pub parent: Option<Box<Node>>,
}

impl Node {
    fn _print(&self) {
        println!("Position: {:?}", self.pos);
        println!("G Score: {:?}", self.g_score);
        println!("H Score: {:?}", self.h_score);
        println!("F Score: {:?}", self.f_score);
        match &self.parent {
            Some(parent) => println!("Parent: {:?}", parent.pos),
            None => println!("Parent: None")
        }
    }
}

impl Node {
    pub fn new(pos: (i32, i32), g_score: i32, h_score: i32, f_score: i32, parent: Option<Box<Node>>) -> Self {
        Node {
            pos,
            g_score,
            h_score,
            f_score,
            parent,
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            pos: self.pos.clone(),
            g_score: self.g_score,
            h_score: self.h_score,
            f_score: self.f_score,
            parent: self.parent.clone(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}



fn get_neighbors(grid: &World, node: &Node) -> Vec<Node> {
    let mut neighbors = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let x = node.pos.0 + i;
            let y = node.pos.1 + j;

            if x < 0 || x >= grid.grid.len() as i32 || y < 0 || y >= grid.grid[0].len() as i32 {
                continue;
            }

            let tile = &grid.grid[x as usize][y as usize];

            if *tile == TileType::Lake || *tile == TileType::Mountain{
                continue;
            }

            let weight = match tile {
                TileType::Forest | TileType::Village | TileType::Dungeon => 1,
                //TileType::Mountain => 100,
                _ => 100,
            };

            let new_g_score = node.g_score + weight;

            neighbors.push(Node {
                pos: (x, y),
                g_score: new_g_score,
                h_score: 0,
                f_score: 0,
                parent: Some(Box::new(node.clone())),
            });
        }
    }

    neighbors
}

// calculate the g score (the cost from the start node to the current node)
fn calculate_g_score(node: &Node, start: &Node) -> i32 {
    let mut g = node.g_score;

    let mut current_node = node;
    while let Some(parent) = current_node.parent.as_ref() {
        if parent.pos.0 != current_node.pos.0 && parent.pos.1 != current_node.pos.1 {
            g += 14; // diagonal move
        } else {
            g += 10; // horizontal or vertical move
        }
        if parent.pos.0 == start.pos.0 && parent.pos.1 == start.pos.1 {
            // found a path to the start node
            break;
        }
        current_node = parent.as_ref();
    }
    g
}


// calculate the h score (the heuristic estimate of the cost from the current node to the goal node)
fn calculate_h_score(pos: (i32, i32), goal_pos: (i32, i32)) -> i32 {
    let dx = (pos.0 - goal_pos.0).abs();
    let dy = (pos.1 - goal_pos.1).abs();
    let diagonal = std::cmp::min(dx, dy);
    let straight = dx + dy - diagonal;
    10 * straight + 14 * diagonal
}

// Calculates the f score of a node
fn _calculate_f_score(node: &Node, start: &Node, end: &Node) -> i32 {
    let g = calculate_g_score(node, start);
    let h = calculate_h_score(node.pos, end.pos);
    g + h
}

// Returns the node with the lowest f score from a set of nodes
fn get_lowest_f_score_node(open_set: &Vec<Node>) -> Node {
    let mut lowest_node = open_set.first().unwrap();
    let mut lowest_f_score = lowest_node.f_score;
    for node in open_set {
        if node.f_score < lowest_f_score {
            lowest_f_score = node.f_score;
            lowest_node = node;
        }
    }
    lowest_node.clone()
}

 
fn get_node_positions(node: &Node) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();
    let mut current_node = Some(node);

    while let Some(node) = current_node {
        positions.push(node.pos);
        current_node = node.parent.as_ref().map(|n| n.as_ref());
    }

    positions.reverse();
    positions
}

pub fn find_path(grid: &World, start_pos: (i32, i32), end_pos: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let start = Node::new(start_pos, 0, 0, 0, None);
    let end = Node::new(end_pos, 0, 0, 0, None);
    let mut open_set = vec![start.clone()];
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_scores = HashMap::new();
    g_scores.insert(start.clone(), 0);

    while !open_set.is_empty() {
        //Is good
        let current_node = get_lowest_f_score_node(&open_set);
        
        if current_node == end {
            //return Some(reconstruct_path(&came_from, current_node));
            return Some(get_node_positions(&current_node));
        }
        //Removes current node from open_set
        open_set.retain(|node| *node != current_node);

        closed_set.insert(current_node.clone());

        for neighbor in get_neighbors(grid, &current_node) {
            if closed_set.contains(&neighbor) {
                continue;
            }
            let tentative_g_score = calculate_g_score(&current_node, &start) + calculate_h_score(current_node.pos, neighbor.pos);
            //let tentative_g_score = *g_scores.get(&current_node).unwrap_or(&i32::MAX) + calculate_h_score(current_node.pos, neighbor.pos);

            if !open_set.contains(&neighbor) || tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.clone(), current_node.clone());
                g_scores.insert(neighbor.clone(), tentative_g_score);
                let f_score = tentative_g_score + calculate_h_score(neighbor.pos, end.pos);
                
                let neighbor_with_score = Node::new(
                    neighbor.pos,
                    tentative_g_score,
                    calculate_h_score(neighbor.pos, end_pos),
                    f_score,
                     Some(Box::new(current_node.clone())),
                );
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor_with_score);
                }
            }
        }
    }

    None
}

