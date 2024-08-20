#![allow(dead_code)]

use bevy::prelude::*;
use camera::camera_drag_system;
use camera::CameraDragging;

// Module imports
mod camera;
mod components;
mod debug;
mod errors;
mod movement;
mod tile;
mod world;
mod entities {
    pub mod agent;
    pub mod monster;
    pub mod treasure;
}
mod mcst_system {
    mod mcst_tree {
        pub mod mcst_node;
        pub mod mcst_tree;
    }
    pub mod backpropogate;
    pub mod mcst;
    pub mod selection_expansion;
    pub mod setup;
    pub mod simulation;
    pub mod systems;
}
mod tests {
    pub mod mcst_tests;
    pub mod simple_agent;
}

// Use statements from mcst_system and standard libraries
use crate::entities::agent::Agent;
use crate::mcst_system::mcst::{NpcAction, SimulationTree};
use crate::tile::Tile;
use mcst_system::backpropogate::backpropgate;
use mcst_system::backpropogate::check_simulation_finish;
use mcst_system::selection_expansion::select_expansion;
use mcst_system::setup::IterationCount;
use mcst_system::setup::{change_state, check_end, setup, setup_tree};
use mcst_system::simulation::{run_actual, run_simulation, set_simulation_actions};
use mcst_system::systems::AgentMessages;
use mcst_system::systems::MonsterMessages;
use mcst_system::systems::TreasureMessages;
use mcst_system::systems::{
    agent_message_system, cleanup_system, monster_message_system, perform_action,
    treasure_message_system,
};
use std::collections::VecDeque;
use std::iter::empty;
use std::sync::{Arc, Mutex};
use world::GameWorld;

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

#[derive(Resource)]
pub struct WorldSim(pub GameWorld);

#[derive(Resource)]
pub struct AgentList(pub Vec<Agent>);

#[derive(Resource)]
pub struct NpcActions(pub Vec<(u32, VecDeque<mcst::NpcAction>)>);

#[derive(Resource)]
pub struct NpcActionsCopy(pub Vec<(u32, VecDeque<mcst::NpcAction>)>);

#[derive(Resource)]
pub struct ScoreTracker(pub Vec<(u32, i32)>);
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
        let cloned_grid: Vec<Vec<Arc<Mutex<Tile>>>> = world
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| Arc::new(Mutex::new(tile.lock().unwrap().clone())))
                    .collect()
            })
            .collect();

        // Create a new WorldSim instance with the cloned contents
        WorldSim(GameWorld {
            agents: cloned_agents,
            monsters: cloned_monsters,
            treasures: cloned_treasures,
            grid: cloned_grid,
        })
    }
}

#[allow(dead_code)]
fn main() {
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
        // Add default Bevy plugins to the app. This includes basic functionality like rendering, input handling, etc.
        .add_plugins(DefaultPlugins)
        // Insert a GameWorld resource that contains the game world's grid.
        .insert_resource(world::create_world())
        // Insert a WorldSim resource that can be modified for simulations.
        .insert_resource(WorldSim(GameWorld::new()))
        // Insert the simulation tree.
        .insert_resource(SimulationTree::new_empty())
        // Insert a CameraDragging resource to track the camera dragging state.
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
        .insert_resource(IterationTotal(0))
        // Insert resources for messages and other systems.
        .insert_resource(AgentMessages::new())
        .insert_resource(MonsterMessages::new())
        .insert_resource(TreasureMessages::new())
        .insert_resource(IterationCount(0))
        // Add a system that handles camera drag functionality.
        .add_system(camera_drag_system)
        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(setup)
        // Setup Phase Systems
        .add_system(setup_tree.label("setup_tree"))
        .add_system(check_end.after("setup_tree").label("check_end"))
        .add_system(change_state.after("check_end").label("change_state"))
        .add_system(
            select_expansion
                .after("change_state")
                .label("selection_expansion_phase"),
        )
        // Simulation Phase Systems
        .add_system(
            set_simulation_actions
                .after("selection_expansion_phase")
                .label("set_simulation_actions"),
        )
        .add_system(
            run_simulation
                .after("set_simulation_actions")
                .label("simulation_actions"),
        )
        .add_system(
            check_simulation_finish
                .after("simulation_actions")
                .label("check_simulation_end"),
        )
        .add_system(
            backpropgate
                .after("check_simulation_end")
                .label("backpropegate"),
        )
        // Running Phase System
        .add_system(run_actual.after("backpropegate").label("running_actions"))
        // Run the app
        .run();
}

fn debug(
    mut query: Query<&mut Agent>,
    //world: ResMut<GameWorld>,
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
            //agent.set_agent_target_id(2);
            //agent.set_target(entities::agent::Target::Agent);
            //agent.set_action(NpcAction::AttackMonster);
            //agent.perform_action(world, commands, agent_messages);
        }
    }
}

fn toggle_flag_system(keyboard_input: Res<Input<KeyCode>>, mut toggle_flag: ResMut<MCSTFlag>) {
    if keyboard_input.just_pressed(KeyCode::X) {
        // Toggle the flag to true when X key is pressed
        toggle_flag.0 = !toggle_flag.0;
        println!("Flag toggled to: {}", toggle_flag.0);
    }
}
