use bevy::prelude::*;
use crate::components::{Position, TileComponent, TreasureComponent};
use crate::npc::{Agent};
use crate::tile::TileType;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_grid: Res<WorldGrid>,
) {
// Load the individual textures

let forest_texture = asset_server.load("textures/forest.png");
let mountain_texture = asset_server.load("textures/mountain.png");
let lake_texture = asset_server.load("textures/water.png");
let village_texture = asset_server.load("textures/village.png");
let dungeon_texture = asset_server.load("textures/dungeon.png");
let agent_texture = asset_server.load("textures/agent.png");
let monster_texture = asset_server.load("textures/enemy.png");
let materials = [
    materials.add(forest_texture.into()),
    materials.add(mountain_texture.into()),
    materials.add(lake_texture.into()),
    materials.add(village_texture.into()),
    materials.add(dungeon_texture.into()),
    materials.add(agent_texture.into()),
    materials.add(monster_texture.into()),
];
/*
let materials = [
    materials.add(Color::NONE.into()),
    materials.add(Color::NONE.into()),
    materials.add(Color::NONE.into()),
    materials.add(Color::NONE.into()),
    materials.add(Color::NONE.into()),
    materials.add(Color::NONE.into()),
    materials.add(Color::NONE.into()),
];
*/
for (y, row) in world_grid.grid.iter().rev().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            let treasure = None; // Replace with actual treasure if desired
            let material_index = match tile_type {
                TileType::Forest => 0,
                TileType::Mountain => 1,
                TileType::Lake => 2,
                TileType::Village => 3,
                TileType::Dungeon => 4,
            };
            let sprite_bundle = SpriteBundle {
                material: materials[material_index].clone(),
                transform: Transform::from_xyz(
                    (x as f32)   * 32.0,
                    (y as f32) * 32.0,
                    0.0,
                    ),
                    sprite: Sprite::new(Vec2::new(32.0, 32.0)),
                    ..Default::default()
            };
    
            
            let mut tile_entity = commands.spawn_bundle(sprite_bundle);
            tile_entity.insert(Position { x: x as i32, y: y as i32 });
            tile_entity.insert(TileComponent { tile_type: tile_type.clone() });
            tile_entity.insert(Position { x: x as i32, y: y as i32 });
    
    
            if let Some(treasure) = treasure {
                tile_entity.insert(TreasureComponent { treasure });
            }
    
            
        }
    }
    
    // Calculate the center of the grid
    let grid_width = world_grid.grid[0].len() as f32;
    let grid_height = world_grid.grid.len() as f32;
    let half_grid_width = grid_width * 16.0;
    let half_grid_height = grid_height * 16.0;
    

    // Set up the 2D camera at the center of the grid
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(Transform::from_xyz(half_grid_width, half_grid_height, 1000.0));

    if let Some(tile_type) = world_grid.get(5, 5) {
        println!("TileType at position (5, 5): {:?}", tile_type);
    } else {
        println!("Invalid position (5, 5)");
    }
}



pub fn create_world_grid() -> Vec<Vec<TileType>> {
    let map_data: Vec<&str> = vec![
        "fffffffffffffffffffm",
        "fffffffffffffffffflm",
        "fffffffvfffffffffllm",
        "ffffffffffffffffllfm",
        "fffffffffffffffllffm",
        "ffffffffffffffllfffm",
        "fffffffffffffllffffm",
        "ffffffffffffffmffllm",
        "fffffffffffffffmlllm",
        "ffffmfffffffffmllllm",
        "fffffmffffffffffmllm",
        "fffffffmfffffmmmmllm",
        "mmmmmmmmmmmmmmmmlllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
    ];
    let world_grid: Vec<Vec<TileType>> = map_data
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'm' => TileType::Mountain,
                    'l' => TileType::Lake,
                    'v' => TileType::Village,
                    'd' => TileType::Dungeon,
                    'f' => TileType::Forest,
                    _ => panic!("Invalid tile character: {}", c),
                })
                .collect()
        })
        .collect();
    world_grid
}
pub struct WorldGrid {
    pub grid: Vec<Vec<TileType>>,
}

impl WorldGrid {
    pub fn get(&self, x: usize, y: usize) -> Option<&TileType> {
        self.grid.get(y)?.get(x)
    }
}

pub struct Agents {
    pub vec: Vec<Agent>,
}
