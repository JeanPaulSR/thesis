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
mod simulation;
use systems::AgentMessages;
use systems::MonsterMessages;
use systems::TreasureMessages;
use systems::agent_message_system;
use systems::cleanup_system;
use systems::monster_message_system;
use systems::perform_action;
use systems::treasure_message_system;
use simulation::run_simulation;
use world::World;
mod movement; 
mod mcst;
mod errors;
mod tests{
    pub mod mcst_tests;
    pub mod simple_agent;
}
mod entities {
    pub mod monster;
    pub mod agent;
    pub mod treasure;
}
use crate::components::{Position, TileComponent};
use crate::entities::agent::Agent;
use crate::tile::TileType;
use crate::errors::MyError;

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
        //.add_startup_stage("post_startup", SystemStage::single(debug_system.system()))
        //.add_startup_system(npc::debug.system())
        
        // Insert a CameraDragging resource to track the camera dragging state.
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        
        //Insert the world tree
        .insert_resource(mcst::SimulationTree::new_empty())
        // Add the simulation
        .add_system(run_simulation.system())
        

        // Insert AgentMessages resource with an empty vector.
        .insert_resource(AgentMessages::new())
        // Insert MonsterMessages resource with an empty vector.
        .insert_resource(MonsterMessages::new())
        // Insert TreasureMessages resource with an empty vector.
        .insert_resource(TreasureMessages::new())
        
        //End simulation key
        .insert_resource(ToggleFlag(false))
        .add_system(toggle_flag_system.system())

        // Add the despawn handler
        .add_system(cleanup_system.system())
        // Add the agent message system to handle messages between treasures.
        .add_system(treasure_message_system.system())
        // Add the agent message system to handle messages between monsters.
        .add_system(monster_message_system.system())
        // Add the agent message system to handle messages between agents.
        .add_system(agent_message_system.system())
        // Add the agent action handling
        .add_system(perform_action.system())

        
        //.add_system(debug.system())

        // Custom systems here
        .run();
}

struct ToggleFlag(bool);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world: ResMut<World>,
) {
    commands.insert_resource::<i32>(0);
    
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
            //let treasure = None;
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
    
        // // Try to add the agent to the world
        if let Err(err) = world.add_agent(agent.clone(), &mut commands) {
            match err {
                MyError::TileNotFound => {
                    println!("Failed to add agent: Tile not found.");
                }
                _ => {
                    println!("Failed to add agent: Unknown error.");
                }
            }
        } 
    }
}

fn debug(
    mut query: Query<&mut Agent>, 
    //world: ResMut<World>,
    //mut agent_messages: ResMut<AgentMessages>,
    //commands: &mut Commands,
) {
    println!("Debuggng");
    // Query for all mutable Agent components
    for mut agent in query.iter_mut() {
        if agent.get_id() == 1 {
            let (x, y) = agent.get_position();
            println!("Position for agent 1: ({},{})", x, y);
            // Found the desired agent by ID
            agent.set_agent_target_id(2);
            agent.set_target(entities::agent::Target::Agent);
            agent.set_action(mcst::NpcAction::Attack);
            //agent.perform_action(world, commands, agent_messages);
        }
    }

}

fn debug_system(
    query: Query<&mut Agent>,
    // world: ResMut<World>,
    // agent_messages: ResMut<AgentMessages>,
    // mut commands: Commands,
) {
    debug(query, 
        // world, 
        // agent_messages, 
        // &mut commands
    );
    
}


fn toggle_flag_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_flag: ResMut<ToggleFlag>,
) {
    if keyboard_input.just_pressed(KeyCode::X) {
        // Toggle the flag to true when X key is pressed
        toggle_flag.0 = !toggle_flag.0;
        println!("Flag toggled to: {}", toggle_flag.0);
    }
}


    
