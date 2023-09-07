use bevy::prelude::*;
use camera::camera_drag_system;
use camera::CameraDragging;
mod tile;
mod components;
mod systems;
mod camera;
mod world;
mod npc;
mod debug;
mod behavior;
use errors::MyError;
use world::World;
mod movement; 
mod mcst;
mod errors;
mod entities {
    pub mod monster;
    pub mod agent;
    pub mod treasure;
}
use crate::components::{Position, TileComponent, TreasureComponent};
use crate::entities::agent::Agent;
use crate::tile::TileType;

const START_AGENT_COUNT: usize = 5;

#[allow(dead_code)]
fn main() {
    // Begin building the Bevy app.
    App::build()
        // Set the window properties, such as title, width, and height.
        .insert_resource(WindowDescriptor {
            title: "Thesis".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        // Add default Bevy plugins to the app. This includes basic functionality like rendering, input handling, etc.
        .add_plugins(DefaultPlugins)
        // Insert a World resource that contains the game world's grid.
        .insert_resource(world::create_world())
        // Add a system that handles camera drag functionality.
        .add_system(camera_drag_system.system())
        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(setup.system())
        // Add a system that moves agents to a village.
        .add_startup_system(npc::debug.system())
        // Insert a CameraDragging resource to track the camera dragging state.
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        // Custom systems here
        .run();
}

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

    for (y, column) in world.grid.iter_mut().enumerate() {
        for (x, tile) in column.iter_mut().enumerate() {
            let treasure = None;
            let material_handle = match tile.lock().unwrap().get_tile_type() {
                TileType::Forest => forest_material.clone(),
                TileType::Mountain => mountain_material.clone(),
                TileType::Lake => lake_material.clone(),
                TileType::Village => village_material.clone(),
                TileType::Dungeon => dungeon_material.clone(),
            };

            let sprite_bundle = SpriteBundle {
                material: material_handle,
                transform: Transform::from_xyz((x as f32) * 32.0, (y as f32) * 32.0, 0.0),
                sprite: Sprite::new(Vec2::new(32.0, 32.0)),
                ..Default::default()
            };

            let mut tile_entity = commands.spawn_bundle(sprite_bundle);
            tile_entity.insert(Position { x: x as i32, y: y as i32 });
            tile_entity.insert(TileComponent { tile_type: tile.lock().unwrap().get_tile_type().clone() });
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
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(half_grid_width, half_grid_height, 1000.0));

    let mut villages: Vec<(usize, usize)> = Vec::new();
    for (y, column) in world.grid.iter().enumerate() {
        for (x, tile_mutex) in column.iter().enumerate() {
            let tile = tile_mutex.lock().unwrap();
            if tile.get_tile_type() == TileType::Village {
                villages.push((x, y));
            }
        }
    }
    
    for i in 0..START_AGENT_COUNT {
        let village = villages[i % villages.len()];
    
        let agent = Agent::new_agent(
            village.0 as f32,
            village.1 as f32,
            &mut commands,
            &mut materials,
            &asset_server,
        );
    

        // Try to add the agent to the world
        if let Err(err) = world.add_agent(agent.clone()) {
            // Handle the error here, e.g. print an error message
            match err {
                MyError::TileNotFound => {
                    println!("Failed to add agent: Tile not found.");
                }
                // Handle other error cases if needed
                _ => {
                    println!("Failed to add agent: Unknown error.");
                }
            }
        } 
    }
}

