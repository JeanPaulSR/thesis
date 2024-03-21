use bevy::prelude::*;
use camera::camera_drag_system;
use camera::CameraDragging;
mod tile;
mod components;
mod camera;
mod world;
mod debug;
use world::World;
mod movement; 
mod errors;
use std::sync::Arc;
use std::sync::Mutex;
use crate::tile::Tile;
mod tests{
    pub mod mcst_tests;
    pub mod simple_agent;
}
mod entities {
    pub mod monster;
    pub mod agent;
    pub mod treasure;
}
mod mcst_system{
    mod mcst_tree{
        pub mod mcst_node;
        pub mod mcst_tree;
    }
    pub mod backpropogate;
    pub mod mcst;
    pub mod setup;
    pub mod simulation;
    pub mod selection_expansion;
    pub mod systems;
}
use std::collections::VecDeque;
use mcst_system::systems::AgentMessages;
use mcst_system::systems::MonsterMessages;
use mcst_system::systems::TreasureMessages;



use mcst_system::systems::perform_action;

use mcst_system::simulation::setup_simulation;
use crate::mcst_system::mcst::NpcAction;
use crate::mcst_system::mcst;
use crate::mcst_system::mcst::SimulationTree;
use crate::components::{Position, TileComponent};
use crate::entities::agent::Agent;
use crate::tile::TileType;
use crate::errors::MyError;

const START_AGENT_COUNT: usize = 5;
struct SimulationCompleteEvent;

pub struct SimulationFlag(bool);
pub struct RunningFlag(bool);
pub struct Backpropogate(bool);
pub struct SimulationTotal(i32);
pub struct MCSTCurrent(i32);
pub struct MCSTTotal(i32);
pub struct WorldSim(World);
pub struct NpcActions(Vec<(u32, VecDeque<mcst::NpcAction>)>);
pub struct NpcActionsCopy(Vec<(u32, VecDeque<mcst::NpcAction>)>);
pub struct ScoreTracker(Vec<(u32, u32)>);

impl WorldSim {
    pub fn get_world(&self) -> &World {
        &self.0
    }

    pub fn copy_world(&self, world: &World) -> WorldSim {
        // Clone the contents of the Arc<Mutex<_>> fields
        let cloned_agents = Arc::new(Mutex::new(world.agents.lock().unwrap().clone()));
        let cloned_monsters = Arc::new(Mutex::new(world.monsters.lock().unwrap().clone()));
        let cloned_treasures = Arc::new(Mutex::new(world.treasures.lock().unwrap().clone()));

        // Clone the grid if necessary (assuming Tile implements Clone)
        let cloned_grid: Vec<Vec<Arc<Mutex<Tile>>>> = world.grid.iter()
            .map(|row| row.iter()
                .map(|tile| Arc::new(Mutex::new(tile.lock().unwrap().clone())))
                .collect())
            .collect();

        // Create a new WorldSim instance with the cloned contents
        let world_sim = WorldSim(World {
            agents: cloned_agents,
            monsters: cloned_monsters,
            treasures: cloned_treasures,
            grid: cloned_grid,
        });

        // Return the copied WorldSim instance
        world_sim
    }
}

#[allow(dead_code)]
fn main() {
    //let simulation_message = AgentMessages::new();
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
        // Insert a World resource that can be modifed for simulations.
        .insert_resource(WorldSim(World::new()))
        // Add a system that handles camera drag functionality.
        .add_system(camera_drag_system.system())
        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(setup.system())
        
        // Insert a CameraDragging resource to track the camera dragging state.
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        
        //Insert the world tree
        .insert_resource(SimulationTree::new_empty())
        //Simulation Event
        .add_event::<SimulationCompleteEvent>()
        

        // Insert AgentMessages resource with an empty vector.
        .insert_resource(AgentMessages::new())
        // Insert MonsterMessages resource with an empty vector.
        .insert_resource(MonsterMessages::new())
        // Insert TreasureMessages resource with an empty vector.
        .insert_resource(TreasureMessages::new())
        
        //End simulation key
        
        .insert_resource(SimulationTotal(0))
        .insert_resource(MCSTCurrent(0))
        .insert_resource(MCSTTotal(0))
        .insert_resource(Vec::<Agent>::new())
        .insert_resource(SimulationFlag(false))
        .insert_resource(RunningFlag(false))
        .insert_resource(Backpropogate(false))
        .insert_resource(NpcActions(Vec::new()))
        .insert_resource(NpcActionsCopy(Vec::new()))
        .insert_resource(ScoreTracker(Vec::new()))
        .add_system(toggle_flag_system.system())
        
        
        // Add the simulation
        .add_system(setup_simulation.system())
        // Add the agent action handling
        .add_system(perform_action.system().label("action"))

        // Add the agent message system to handle messages after actions.
        //.add_system(treasure_message_system.system().after("action").label("message"))
        //.add_system(monster_message_system.system().after("action").label("message"))
        //.add_system(agent_message_system.system().after("action").label("message"))
        // Add the despawn handler after all message systems
        //.add_system(cleanup_system.system().after("message"))
        
        //.add_system(debug.system())
        //)
        .run();
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world: ResMut<World>,
    mut iteration_total: ResMut<SimulationTotal>,
    mut mcst_total: ResMut<MCSTTotal>,
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
    iteration_total.0 = 3;
    mcst_total.0 = 10;
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
            agent.set_action(NpcAction::Attack);
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
    mut toggle_flag: ResMut<SimulationFlag>,
) {
    if keyboard_input.just_pressed(KeyCode::X) {
        // Toggle the flag to true when X key is pressed
        toggle_flag.0 = !toggle_flag.0;
        println!("Flag toggled to: {}", toggle_flag.0);
    }
}
