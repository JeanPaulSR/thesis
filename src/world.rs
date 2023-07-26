use bevy::prelude::*;
use crate::components::{Position, TileComponent, TreasureComponent};
use crate::entities::agent::Agent;
use crate::tile::TileType;
use crate::tile::Tile;


const _WORLD_WIDTH: usize = 30;
const _WORLD_HEIGHT: usize = 30;
const START_AGENT_COUNT: usize = 50;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world: ResMut<World>,
) {
    // Load the individual textures
    let forest_texture = asset_server.load("textures/forest.png");
    let mountain_texture = asset_server.load("textures/mountain.png");
    let lake_texture = asset_server.load("textures/water.png");
    let village_texture = asset_server.load("textures/village.png");
    let dungeon_texture = asset_server.load("textures/dungeon.png");

    // Add the materials directly to the `materials` variable
    let forest_material = materials.add(forest_texture.into());
    let mountain_material = materials.add(mountain_texture.into());
    let lake_material = materials.add(lake_texture.into());
    let village_material = materials.add(village_texture.into());
    let dungeon_material = materials.add(dungeon_texture.into());

    for (y, row) in world.grid.iter().rev().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            let treasure = None;
            let material_handle = match tile_type {
                TileType::Forest => forest_material.clone(),
                TileType::Mountain => mountain_material.clone(),
                TileType::Lake => lake_material.clone(),
                TileType::Village => village_material.clone(),
                TileType::Dungeon => dungeon_material.clone(),
            };

            let sprite_bundle = SpriteBundle {
                material: material_handle,
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
    let grid_width = world.grid[0].len() as f32;
    let grid_height = world.grid.len() as f32;
    let half_grid_width = grid_width * 16.0;
    let half_grid_height = grid_height * 16.0;
    

    // Set up the 2D camera at the center of the grid
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(Transform::from_xyz(half_grid_width, half_grid_height, 1000.0));

    
    let mut villages: Vec<(f32, f32)> = Vec::new();
    for (y, row) in world.grid.iter().rev().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
                if let TileType::Village = tile_type {
                    villages.push((x as f32, y as f32));
                }
            }
    }
    for i in 0..START_AGENT_COUNT {
        let village = villages[i % villages.len()];

        let mut agent = Agent::new_agent(
            village.0 ,
            village.1 ,
            &mut commands,
            &mut materials,
            &asset_server,
        );
        world.agents.push(agent);
    }

    let agent1 = world.get_agent(0);
    if let Some(agent) = agent1 {
        agent.travel(2.0, 3.0, &mut commands);
    }

}



pub fn create_world() -> Vec<Vec<TileType>> {
    let map_data: Vec<&str> = vec![
        "vffffffffffffffffffm",
        "fmfffffffffffffffflm",
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
        "mmmmmmmmmmmfmmmmlllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "fffffffffffffffflllm",
        "ffffffffvffffffflllm",
        "fmfffffffffffffflllm",
        "fffffffffffffffflllm",
    ];
    let world: Vec<Vec<TileType>> = map_data
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
    world
}
pub struct World {
    pub agents: Vec<Agent>,
    pub grid: Vec<Vec<TileType>>,
}

impl World {
    
    fn _new() -> Self {
        let agents = Vec::new();
        let grid: Vec<Vec<TileType>> = vec![vec![TileType::Forest; _WORLD_HEIGHT]; _WORLD_WIDTH];
        World { agents, grid }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&TileType> {
        self.grid.get(y)?.get(x)
    }

    // Function to add an agent to the world
    pub fn add_agent(&mut self, agent: Agent) {
        // Add the agent to the world's list of agents
        self.agents.push(agent);
    }

    // Function to get a reference to an agent by ID
    pub fn get_agent(&mut self, id: u32) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|a| a.id == id)
    }

    // Function to remove an agent from the world by ID
    pub fn _remove_agent(&mut self, id: u32) -> Option<Agent> {
        // Remove the agent from the world's list of agents
        let index = self.agents.iter().position(|a| a.id == id)?;
        let removed_agent = self.agents.remove(index);
        Some(removed_agent)
    }



    pub fn find_agents_within_distance(&self, agent: &Agent, distance: f32) -> Vec<&Agent> {
        let mut nearby_agents = Vec::new();
    
        for other_agent in &self.agents {
            if agent.id != other_agent.id {
                let dx = (agent.transform.translation.x - other_agent.transform.translation.x).abs() / 32.0;
                let dy = (agent.transform.translation.y - other_agent.transform.translation.y).abs() / 32.0;
                let squared_distance = dx * dx + dy * dy;
                let calculated_distance = squared_distance.sqrt();
                if calculated_distance <= distance {
                    nearby_agents.push(other_agent);
                }
            }
        }
    
        nearby_agents
    }
}
