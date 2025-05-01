use crate::components::TileComponent;
use crate::gameworld::highlight::Highlight;
use crate::gameworld::position::Position;
use crate::gameworld::tile_types::TileType;
use crate::npcs::agent::Agent;
use crate::npcs::monster::Monster;
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::npc_components::npc_type::NPCType;
use crate::npcs::player::Player;
use crate::npcs::treasure::Treasure;
use crate::GameWorld;
use crate::IterationCount;
use crate::WorldRandom;
use crate::WorldSim;
use bevy::prelude::*;
use rand::seq::SliceRandom;

// use super::mcst;
use std::collections::HashMap;

use bevy::ecs::bundle::Bundle;

use super::mcst_tree::mcst_tree::MCTSTree;
use super::mcst_tree::simulation_tree::SimulationTree;

const START_AGENT_COUNT: usize = 10;
const START_MONSTER_COUNT: usize = 5;
const START_TREASURE_COUNT: usize = 5;

#[derive(Bundle)]
pub struct TileBundle {
    pub sprite_bundle: SpriteBundle,
    pub position: Position,
    pub tile_component: TileComponent,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    world: ResMut<GameWorld>,
    mut iteration_total: ResMut<IterationCount>,
    mut world_sim: ResMut<WorldSim>,
    mut world_random: ResMut<WorldRandom>,
    mut simulation_tree: ResMut<SimulationTree>,
    npc_query: Query<(Entity, &NPCBase)>, // Add the NPC query here
) {
    // 1. Load textures dynamically based on tile names
    let mut tile_textures: HashMap<TileType, Handle<Image>> = HashMap::new();
    for tile_type in [
        TileType::Forest,
        TileType::Mountain,
        TileType::Lake,
        TileType::Village,
        TileType::Dungeon,
        TileType::Farm,
        TileType::Mine,
    ] {
        let texture_path = format!(
            "textures/tiles/{}.png",
            tile_type.to_string().to_lowercase()
        );
        tile_textures.insert(tile_type, asset_server.load(texture_path.as_str()));
    }

    // 2. Spawn tiles
    for (position, tile) in world.tiles.iter() {
        let tile_type = tile.lock().unwrap().get_tile_type();
        let texture_handle = tile_textures.get(&tile_type).unwrap().clone();

        commands.spawn(TileBundle {
            sprite_bundle: SpriteBundle {
                texture: texture_handle,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (position.x as f32) * 32.0,
                    (position.y as f32) * 32.0,
                    0.0,
                ),
                ..Default::default()
            },
            position: Position {
                x: position.x,
                y: position.y,
            },
            tile_component: TileComponent { tile_type },
        });
    }

    // 3. Get SPAWN_LOCATION (villages)
    let spawn_locations: Vec<Position> = world
        .tiles
        .iter()
        .filter_map(|(position, tile)| {
            if tile.lock().unwrap().get_tile_type() == TileType::Village {
                Some(*position) // Ensure this uses the correct Position type
            } else {
                None
            }
        })
        .collect();

    if let Some(spawn_position) = spawn_locations.first() {
        add_player(
            *spawn_position,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
        );
    }

    populate_agents(
        START_AGENT_COUNT,
        &spawn_locations,
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut simulation_tree,
    );

    // 5. Spawn monsters and treasures
    let valid_monster_treasure_spawns: Vec<Position> = world
        .tiles
        .iter()
        .filter_map(|(position, tile)| {
            let tile_type = tile.lock().unwrap().get_tile_type();
            if tile_type != TileType::Village
                && tile_type != TileType::Mountain
                && tile_type != TileType::Lake
            {
                let is_far_enough = spawn_locations.iter().all(|spawn| {
                    let dx = (spawn.x - position.x).abs();
                    let dy = (spawn.y - position.y).abs();
                    dx > 5 || dy > 5
                });
                if is_far_enough {
                    return Some(*position);
                }
            }
            None
        })
        .collect();

    // Populate monsters
    populate_monsters(
        START_MONSTER_COUNT,
        &valid_monster_treasure_spawns,
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut world_random, // Pass WorldRandom here
    );

    // Populate treasures
    populate_treasures(
        START_TREASURE_COUNT,
        &valid_monster_treasure_spawns,
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut world_random, // Pass WorldRandom here
    );

    // 6. Set iteration_total and world_sim
    iteration_total.0 = 50;
    world_sim.0 = world.clone();
}

pub fn add_player(
    spawn: Position,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    // Create the NPCBase component
    let npc_base = NPCBase::new(
        spawn.x,
        spawn.y,
        NPCType::Player,
        commands,
        asset_server,
        texture_atlases,
    );

    println!(
        "Created NPCBase for Player at position: ({}, {}), Type: {:?}",
        spawn.x,
        spawn.y,
        NPCType::Player
    );

    // Create the Player component
    let player = Player::new();
    println!("Created Player component.");

    // Spawn the entity with NPCBase and Player components
    let entity = commands.spawn((npc_base, player)).id();
    println!("Spawned Player entity: {:?}", entity);
}

fn populate_agents(
    count: usize,
    spawn_locations: &[Position],
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    simulation_tree: &mut ResMut<SimulationTree>,
) {
    for i in 0..count {
        let spawn = spawn_locations[i % spawn_locations.len()];

        // Add the agent and assign it an MCTS tree
        add_agent(
            spawn,
            commands,
            asset_server,
            texture_atlases,
            simulation_tree,
        );

        println!("Spawned Agent at position: {:?}", spawn);
    }
}

fn populate_monsters(
    count: usize,
    valid_spawns: &[Position],
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    world_random: &mut ResMut<WorldRandom>,
) {
    let mut rng = &mut world_random.0;

    for _ in 0..count {
        // Randomly select a spawn position from valid_spawns
        if let Some(spawn) = valid_spawns.choose(&mut rng) {
            let npc_base = NPCBase::new(
                spawn.x,
                spawn.y,
                NPCType::Monster,
                commands,
                asset_server,
                texture_atlases,
            );

            let monster = Monster::new_monster(*spawn);

            commands.spawn((monster, npc_base));
        }
    }
}

fn populate_treasures(
    count: usize,
    valid_spawns: &[Position],
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    world_random: &mut ResMut<WorldRandom>,
) {
    let mut rng = &mut world_random.0;

    for _ in 0..count {
        // Randomly select a spawn position from valid_spawns
        if let Some(spawn) = valid_spawns.choose(&mut rng) {
            let npc_base = NPCBase::new(
                spawn.x,
                spawn.y,
                NPCType::Treasure,
                commands,
                asset_server,
                texture_atlases,
            );

            let treasure = Treasure::new_treasure();

            commands.spawn((treasure, npc_base));
        }
    }
}

fn assign_tree(agent: &Agent, simulation_tree: &mut ResMut<SimulationTree>) {
    // Create a new MCTS tree for the agent
    let mut tree = MCTSTree::new();

    // Initialize the tree with the agent's data
    tree.initialize_tree();

    // Add the tree to the SimulationTree resource
    simulation_tree.add_tree(agent.get_id(), tree);
}

fn add_agent(
    spawn: Position,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    simulation_tree: &mut ResMut<SimulationTree>,
) {
    // Create the NPCBase component
    let npc_base = NPCBase::new(
        spawn.x,
        spawn.y,
        NPCType::Agent,
        commands,
        asset_server,
        texture_atlases,
    );

    println!(
        "Created NPCBase for Agent at position: ({}, {}), Type: {:?}",
        spawn.x,
        spawn.y,
        NPCType::Agent
    );

    // Create the Agent component
    let agent = Agent::new_agent();
    println!("Created Agent with ID: {}", agent.get_id());

    // Spawn the entity with NPCBase and Agent components
    let entity = commands.spawn((npc_base, agent.clone())).id();
    println!("Spawned Agent entity: {:?}", entity);

    // Assign the agent an MCTS tree
    assign_tree(&agent, simulation_tree);
    println!("Assigned MCTS tree to Agent with ID: {}", agent.get_id());
}

pub fn check_npc_count(npc_query: Query<(Entity, &NPCBase)>) {
    println!("Number of NPCs queried: {}", npc_query.iter().count());
    for (entity, npc_base) in npc_query.iter() {
        println!(
            "Queried NPC - Entity: {:?}, Type: {:?}, Position: {:?}",
            entity, npc_base.npc_type, npc_base.position
        );
    }
}

