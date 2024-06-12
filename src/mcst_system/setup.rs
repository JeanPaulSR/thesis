use std::iter;

use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::AppExit;
use crate::entities::agent::Agent;

use crate::mcst_system::mcst_tree::mcst_tree::MCTSTree;
use crate::FinishedSelectionPhase;
use crate::ScoreTracker;
use crate::WorldSim;
use crate::{MCSTCurrent, MCSTTotal, RunningFlag, SimulationFlag, IterationCurrent, IterationTotal, World, mcst};
use crate::components::{Position, TileComponent};
use crate::tile::TileType;


const START_AGENT_COUNT: usize = 5;
const START_MONSTER_COUNT: usize = 5;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world: ResMut<World>,
    mut iteration_total: ResMut<IterationTotal>,
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

    world.populate_agents(START_AGENT_COUNT, &mut commands, &mut materials, &asset_server);

    world.set_valid_spawns();
    let valid_monster_spawns = world.find_valid_monster_spawns();
    world.populate_monsters(valid_monster_spawns, START_MONSTER_COUNT, &mut commands, &mut materials, &asset_server);
    iteration_total.0 = 3;
    
}

pub fn setup_tree(
    mut tree: ResMut<mcst::SimulationTree>,
    mut mcst_total: ResMut<MCSTTotal>,
    mut agent_query: Query<&mut Agent>, 
    mut score_tracker_res: ResMut<ScoreTracker>,
){
    //If the tree is empty. It is the first iteration. Using startup_system does not work
    if tree.is_empty() {
        mcst_total.0 = 100;
        let score_tracker = &mut score_tracker_res.0;
        for agent in agent_query.iter_mut() {
            let mut new_tree = MCTSTree::new_empty();
            new_tree.initialize_tree(agent.clone());
            tree.insert_tree(new_tree, agent.get_id());
            let tuple = (agent.get_id(), 0);
            score_tracker.push(tuple);
            println!("Finished setup for agent {}", agent.get_id());
        }
    }
}

pub fn check_end(
    iteration_total: ResMut<IterationTotal>,
    iteration_counter: ResMut<IterationCurrent>,
    mcst_current: ResMut<MCSTCurrent>,
    mcst_total: ResMut<MCSTTotal>,
    simulation_flag: ResMut<SimulationFlag>,
    running_flag: ResMut<RunningFlag>,
    mut app_exit_events: ResMut<Events<AppExit>>,
){
    if !simulation_flag.0 && !running_flag.0 {
        if mcst_current.0 == mcst_total.0 {
            if iteration_counter.0 == iteration_total.0{
                app_exit_events.send(AppExit);
                std::process::exit(0);
            }
        }     
    }   
}

pub fn change_state(
    world: ResMut<World>,
    world_sim: ResMut<WorldSim>,
    mut agent_query: Query<&mut Agent>, 
    mut agent_copy: ResMut<Vec::<Agent>>,
    mut simulation_flag: ResMut<SimulationFlag>,
    mut selection_flag: ResMut<FinishedSelectionPhase>,
    mut running_flag: ResMut<RunningFlag>,
    mut mcst_current: ResMut<MCSTCurrent>,
    mcst_total: ResMut<MCSTTotal>,
    mut iteration_counter: ResMut<IterationCurrent>,
){
    if !simulation_flag.0 && !running_flag.0 {
        if mcst_current.0 < mcst_total.0{
            *agent_copy = save_agents_to_vector(&mut agent_query);
            world_sim.copy_world(&world);
            simulation_flag.0 = true;
            mcst_current.0 = mcst_current.0 + 1;
            selection_flag.0 = false;
        } else {
            running_flag.0 = true;
            mcst_current.0 = 0;
            iteration_counter.0 = iteration_counter.0 + 1;
        }
    }
}

// Function to save the state of agents from a query into a vector
fn save_agents_to_vector(
    query: &mut Query<&mut Agent>
) -> Vec<Agent> {
    let mut agent_backup = Vec::new();
    for agent in query.iter_mut() {
        agent_backup.push(agent.clone());
    }
    agent_backup
}