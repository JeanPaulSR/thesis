//CURRENT GOAL IS TO SEPERATE LEADER MESSAGES AND FOLLOWER MESSAGES IN SOCIAL ORGANIZER

#![allow(dead_code)]

use bevy::prelude::*;
use camera::camera_drag_system;
use camera::CameraDragging;

// Module imports
mod components;
mod camera;
mod gameworld {
    pub mod world;
    pub mod position;
    pub mod tile;
    pub mod tile_types;
}
mod debug;
mod movement;
mod errors;
mod entities {
    pub mod monster;
    pub mod agent;
    pub mod treasure;
}
mod mcst_system {
    mod mcst_tree {
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
mod tests {
    pub mod mcst_tests;
    pub mod simple_agent;
}

use clap::command;
use clap::Parser;
use gameworld::world;
use mcst_system::backpropogate::backpropogate;
use mcst_system::backpropogate::check_simulation_finish;
use mcst_system::mcst;
use mcst_system::simulation::check_actual_finish;
use mcst_system::simulation::check_cooperation;
use mcst_system::simulation::handle_current_agent_status;
use mcst_system::simulation::set_actual_simulation_actions;
use mcst_system::simulation::run_actual;
use mcst_system::simulation::set_mcst_actions;
use mcst_system::systems::agent_message_system_social;
use mcst_system::systems::agent_movement_system;
use mcst_system::systems::AgentMessages;
use mcst_system::systems::MonsterMessages;
use mcst_system::systems::TreasureMessages;
use mcst_system::systems::{agent_message_system, perform_action};
use mcst_system::selection_expansion::select_expansion;
use mcst_system::setup::{setup_tree, check_end, change_state, setup};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use world::GameWorld;
use crate::mcst_system::mcst::SimulationTree;
use crate::entities::agent::Agent;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::gameworld::tile::Tile;

// Define resource structs
#[derive(Resource)]
pub struct MCSTFlag(pub bool);

#[derive(Resource)]
pub struct RunningFlag(pub bool);

#[derive(Resource)]
pub struct FinishedSelectionPhase(pub bool);

#[derive(Resource)]
pub struct FinishedSelectingActions(pub bool);

#[derive(Resource)]
pub struct FinishedAllFlag(pub bool);

#[derive(Resource)]
pub struct FinishedRunningFlag(pub bool);

#[derive(Resource)]
pub struct FinishedSimulationFlag(pub bool);

#[derive(Resource)]
pub struct Backpropogate(pub bool);

#[derive(Resource)]
pub struct MCSTCurrent(pub i32);

#[derive(Resource)]
pub struct MCSTTotal(pub i32);

#[derive(Resource)]
pub struct IterationCurrent(pub i32);

#[derive(Resource)]
pub struct IterationTotal(pub i32);

///Simulation world
#[derive(Resource)]
pub struct WorldSim(pub GameWorld);

#[derive(Resource)]
pub struct AgentList(pub Vec<Agent>);

#[derive(Resource)]
pub struct NpcActions(pub Vec<(u32, VecDeque<mcst::NpcAction>)>);

///Used to save actions when updating simulation tree
#[derive(Resource)]
pub struct NpcActionsCopy(pub Vec<(u32, VecDeque<mcst::NpcAction>)>);

#[derive(Resource)]
pub struct ScoreTracker(pub Vec<(u32, i32)>);

#[derive(Resource)]
struct WorldRandom(StdRng);

#[derive(Default, Resource)]
pub struct IterationCount(pub i32);
impl WorldSim {
    pub fn get_world(&self) -> &GameWorld {
        &self.0
    }

    pub fn copy_world(&self, world: &GameWorld) -> WorldSim {
        // Safely clone the contents of the Arc<Mutex<_>> fields
        let cloned_agents = Arc::new(Mutex::new(world.agents.lock().unwrap().clone()));
        let cloned_monsters = Arc::new(Mutex::new(world.monsters.lock().unwrap().clone()));
        let cloned_treasures = Arc::new(Mutex::new(world.treasures.lock().unwrap().clone()));

        // Clone the grid, ensuring that each Tile is properly cloned and the Arc<Mutex<Tile>> structure is maintained
        let cloned_grid: Vec<Vec<Arc<Mutex<Tile>>>> = world.grid.iter()
            .map(|row| row.iter()
                .map(|tile| Arc::new(Mutex::new(tile.lock().unwrap().clone())))
                .collect())
            .collect();

        // Create a new WorldSim instance with the cloned contents
        WorldSim(GameWorld {
            agents: cloned_agents,
            monsters: cloned_monsters,
            treasures: cloned_treasures,
            grid: cloned_grid,
            tiles: world.get_tiles().clone(),
            width_mind: world.get_width_mind(),
            height_min: world.get_height_min(),
            width_max: world.get_width_max(),
            height_max: world.get_height_max(),
        })
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed for the random number generator
    #[arg(short, long, default_value_t = 0)]
    seed: u64,
}

#[allow(dead_code)]
fn main() {

    // Parse the command-line arguments
    let args = Args::parse();

    // Use the seed from the command-line or generate a random seed
    let seed = if args.seed != 0 { args.seed } else { rand::thread_rng().gen() };
    println!("Using seed: {}", seed);

    // Initialize a seeded RNG
    let rng = StdRng::seed_from_u64(seed);
    let game_world = world::initialize("test").expect("Failed to initialize the game world");
    
    // Begin building the Bevy app using App::new().
    App::new()
        // Set the window properties, such as title, width, and height.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Thesis".to_string(),
                resolution: (800., 600.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Insert various resources
        .insert_resource(game_world)
        .insert_resource(WorldSim(GameWorld::new()))
        .insert_resource(SimulationTree::new_empty())
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        .insert_resource(MCSTCurrent(0))
        .insert_resource(MCSTTotal(0))
        .insert_resource(IterationCurrent(0))
        .insert_resource(FinishedSelectionPhase(false))
        .insert_resource(FinishedSelectingActions(false))
        .insert_resource(AgentList(Vec::new()))
        .insert_resource(MCSTFlag(false))
        .insert_resource(RunningFlag(false))
        .insert_resource(FinishedRunningFlag(false))
        .insert_resource(FinishedSimulationFlag(false))
        .insert_resource(Backpropogate(false))
        .insert_resource(NpcActions(Vec::new()))
        .insert_resource(NpcActionsCopy(Vec::new()))
        .insert_resource(ScoreTracker(Vec::new()))
        .insert_resource(IterationTotal(3))
        .insert_resource(AgentMessages::new())
        .insert_resource(MonsterMessages::new())
        .insert_resource(TreasureMessages::new())
        .insert_resource(IterationCount(0))
        .insert_resource(WorldRandom(rng))
        // Add systems using system sets for labels and ordering
        .add_systems(Startup, (
            setup,
        )) // Startup system
        .add_systems(Update, (
            camera_drag_system,
            setup_tree.in_set(SimulationSet::Setup),
            //Will end after a certain amount of iterations (Default 3)
            check_end.after(SimulationSet::Setup).in_set(SimulationSet::CheckEnd),
            change_state.after(SimulationSet::CheckEnd).in_set(SimulationSet::ChangeState),
            //MCST Selection & Expansion Phase
            //Done
            select_expansion
                .after(SimulationSet::ChangeState)
                .in_set(SimulationSet::SelectionExpansion),

            //Sets the current actions by popping it out of the actions list for that agent
            // Sets the action, the target and the tile target
            set_mcst_actions
                .after(SimulationSet::SelectionExpansion)
                .in_set(SimulationSet::SetSimulationActions),

            //Checks which agents will work together, ensuring targets where applicable
            //Also sends group requests, where those that ask first get priority in being leaders
            check_cooperation
                .after(SimulationSet::SetSimulationActions)
                .in_set(SimulationSet::CheckCooperation),

            //Revamp the loop in this one
            //Done
            agent_message_system_social
                .after(SimulationSet::CheckCooperation)
                .in_set(SimulationSet::AgentSocialMessaging),
                
            //MCST Simulation Phase
            //Sets the status and sends the first action message
            //Handles follow up actions
            //Current plan is to move messages to perform_action
            handle_current_agent_status
                .after(SimulationSet::AgentSocialMessaging)
                .in_set(SimulationSet::RunSimulation),
                //IF AGENT MOVING, CHECK PATHFINDING
            agent_movement_system
                .after(SimulationSet::RunSimulation)
                .in_set(SimulationSet::MoveAgent),
            agent_message_system
                .after(SimulationSet::MoveAgent)
                .in_set(SimulationSet::AgentMovement),
            perform_action
                .after(SimulationSet::AgentMovement)
                .in_set(SimulationSet::PerformAction),
            check_simulation_finish
                .after(SimulationSet::PerformAction)
                .in_set(SimulationSet::CheckSimulationEnd),
            //MCST Backpropegation Phase
            backpropogate
                .after(SimulationSet::CheckSimulationEnd)
                .in_set(SimulationSet::Backpropogate),
            //Actual Simulations
            set_actual_simulation_actions
                .after(SimulationSet::Backpropogate)
                .in_set(SimulationSet::SetActualActions),
            run_actual
                .after(SimulationSet::SetActualActions)
                .in_set(SimulationSet::RunActions),
            check_actual_finish
                .after(SimulationSet::RunActions)
                .in_set(SimulationSet::CheckActualFinish),
        ))
        .run();
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SimulationSet {
    Setup,
    CheckEnd,
    ChangeState,
    SelectionExpansion,
    SetSimulationActions,
    CheckCooperation,
    AgentSocialMessaging,
    RunSimulation,
    MoveAgent,
    PerformAction,
    AgentMovement,
    CheckSimulationEnd,
    Backpropogate,
    SetActualActions,
    RunActions,
    CheckActualFinish,
}


fn debug(
    mut query: Query<&mut Agent>, 
    //world: ResMut<GameWorld>,
    //mut agent_messages: ResMut<AgentMessages>,
    //commands: &mut Commands,
) {
    println!("Debuggng");
    // Query for all mutable Agent components
    for agent in query.iter_mut() {
        if agent.get_id() == 1 {
            let (x, y) = agent.get_position();
            println!("Position for agent 1: ({},{})", x, y);
            // Found the desired agent by ID
            //agent.set_agent_target_id(2);
            //agent.set_target(entities::agent::Target::Agent);
            //agent.set_action(NpcAction::AttackMonster);
            //agent.perform_action(world, commands, agent_messages);
        }
    }

}


fn toggle_flag_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_flag: ResMut<MCSTFlag>,
) {
    if keyboard_input.just_pressed(KeyCode::X) {
        // Toggle the flag to true when X key is pressed
        toggle_flag.0 = !toggle_flag.0;
        println!("Flag toggled to: {}", toggle_flag.0);
    }
}
