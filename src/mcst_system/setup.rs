use crate::components::Position;
use crate::components::TileComponent;
use crate::entities::agent::Agent;
use crate::IterationCount;
use crate::WorldRandom;
use bevy::app::AppExit;
use bevy::prelude::*;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::mcst_system::mcst_tree::mcst_tree::MCTSTree;
use crate::gameworld::tile_types::TileType;
use crate::AgentList;
use crate::FinishedSelectionPhase;
use crate::ScoreTracker;
use crate::WorldSim;
use crate::{
    GameWorld, IterationCurrent, IterationTotal, MCSTCurrent, MCSTFlag, MCSTTotal, RunningFlag,
};

use super::mcst;

const START_AGENT_COUNT: usize = 10;
const START_MONSTER_COUNT: usize = 5;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut world: ResMut<GameWorld>,
    mut iteration_total: ResMut<IterationCount>,
    mut world_sim: ResMut<WorldSim>,
    mut world_random: ResMut<WorldRandom>,
) {
    // Load the individual textures
    let forest_texture_handle = asset_server.load("textures/forest.png");
    let mountain_texture_handle = asset_server.load("textures/mountain.png");
    let lake_texture_handle = asset_server.load("textures/water.png");
    let village_texture_handle = asset_server.load("textures/village.png");
    let dungeon_texture_handle = asset_server.load("textures/dungeon.png");

    for (y, column) in world.grid.iter_mut().enumerate() {
        for (x, tile) in column.iter_mut().enumerate() {
            let texture_handle = match tile.lock().unwrap().get_tile_type() {
                TileType::Forest => forest_texture_handle.clone(),
                TileType::Mountain => mountain_texture_handle.clone(),
                TileType::Lake => lake_texture_handle.clone(),
                TileType::Village => village_texture_handle.clone(),
                TileType::Dungeon => dungeon_texture_handle.clone(),
            };

            let sprite_bundle = SpriteBundle {
                texture: texture_handle,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz((x as f32) * 32.0, (y as f32) * 32.0, 0.0),
                ..Default::default()
            };

            let mut tile_entity = commands.spawn(sprite_bundle);
            tile_entity.insert(Position {
                x: x as i32,
                y: y as i32,
            });
            tile_entity.insert(TileComponent {
                tile_type: tile.lock().unwrap().get_tile_type().clone(),
            });
        }
    }

    // Calculate the center of the grid
    let grid_width = world.grid[0].len() as f32;
    let grid_height = world.grid.len() as f32;
    let half_grid_width = grid_width * 16.0;
    let half_grid_height = grid_height * 16.0;

    // Set up the 2D camera at the center of the grid
    commands
        .spawn(Camera2dBundle::default())
        .insert(Transform::from_xyz(
            half_grid_width,
            half_grid_height,
            1000.0,
        ));

    // Pass `texture_atlases` instead of `textures` to `populate_agents`
    world.populate_agents(
        START_AGENT_COUNT,
        &mut commands,
        &mut texture_atlases,
        &asset_server,
    );

    world.set_valid_monster_spawns();
    let valid_monster_spawns = world.get_valid_monster_spawns();
    world.populate_monsters(
        valid_monster_spawns,
        &mut world_random.0,
        START_MONSTER_COUNT,
        &mut commands,
        &mut texture_atlases,
        &asset_server,
    );
    iteration_total.0 = 50; // Update the custom resource
    world_sim.0 = world.clone();
}

pub fn setup_tree(
    mut tree: ResMut<mcst::SimulationTree>,
    mut mcst_total: ResMut<MCSTTotal>,
    mut agent_query: Query<&mut Agent>,
    mut score_tracker_res: ResMut<ScoreTracker>,
    
    //mut commands: Commands,
) {
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

            //commands.entity(agent.get_entity()).insert(Transform::from_translation(Vec3::new(1.0 * 32.0, 1.0 * 32.0, 1.0)));

        }
    }
}

pub fn check_end(
    iteration_total: ResMut<IterationTotal>,
    iteration_counter: ResMut<IterationCurrent>,
    mcst_current: ResMut<MCSTCurrent>,
    mcst_total: ResMut<MCSTTotal>,
    mcst_flag: ResMut<MCSTFlag>,
    running_flag: ResMut<RunningFlag>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    agent_query: Query<&Agent>,
    tree: ResMut<mcst::SimulationTree>,
) {
    //Neither running simulation or in mcst phase
    if !mcst_flag.0 && !running_flag.0 {
        //If the current mcst loop is less than the total mcst loops allowed
        if mcst_current.0 >= mcst_total.0 {
            //If the total number of mcst simulations is less than the total iterations allowed
            if iteration_counter.0 >= iteration_total.0 {
                for agent in agent_query.iter() {
                    println!("Result for agent {}: Score finish 'check_end '{}", agent.get_id(), agent.get_reward());
                }
                let data = tree.to_string();
                let file_path = Path::new("results/result.txt");
            
                if let Some(parent) = file_path.parent() {
                    create_dir_all(parent).expect("Unable to create directories");
                }
            
                let file = File::create(&file_path).expect("Unable to create file");
                let mut writer = BufWriter::new(file);
                writer.write_all(data.as_bytes()).expect("Unable to write data");
                writer.flush().expect("Failed to flush data to file");

                app_exit_events.send(AppExit);
                std::process::exit(0);
            }
        }
    }
}

pub fn change_state(
    world: ResMut<GameWorld>,
    world_sim: ResMut<WorldSim>,
    mut agent_query: Query<&mut Agent>,
    mut agent_copy_res: ResMut<AgentList>,
    mut mcst_flag: ResMut<MCSTFlag>,
    mut selection_flag: ResMut<FinishedSelectionPhase>,
    mut running_flag: ResMut<RunningFlag>,
    mut mcst_current: ResMut<MCSTCurrent>,
    mcst_total: ResMut<MCSTTotal>,
    mut iteration_counter: ResMut<IterationCurrent>,
) {
    if !mcst_flag.0 && !running_flag.0 {
        let agent_copy = save_agents_to_vector(&mut agent_query);
        agent_copy_res.0 = agent_copy;
        if mcst_current.0 < mcst_total.0 {
            world_sim.copy_world(&world);
            mcst_flag.0 = true;
            mcst_current.0 += 1;
            selection_flag.0 = false;
        } else {
            running_flag.0 = true;
            mcst_current.0 = 0;
            iteration_counter.0 += 1;
            println!("Entered regular simulation {} ", iteration_counter.0);
        }
    }
}

// Function to save the state of agents from a query into a vector
fn save_agents_to_vector(query: &mut Query<&mut Agent>) -> Vec<Agent> {
    let mut agent_backup = Vec::new();
    for agent in query.iter_mut() {
        agent_backup.push(agent.clone());
    }
    agent_backup
}
