#![allow(dead_code)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use system::mcst_system::expansion::expansion_system;
use system::mcst_system::selection::selection_system;
use system::player_system::handle_player_movement::move_player;
use system::player_system::highlight_squares::highlight_moveable_player_squares;
use system::simulation::handle_npc_movement::handle_agent_movement;
use system::simulation::handle_selected_action::handle_selected_action_system;
use ui::mcst_tree_display::mcst_tree_display::agent_action_button_system;
use ui::mcst_tree_display::mcst_tree_display::update_agent_action_button_visibility;
use ui::mcst_tree_display::mcst_tree_display::DisplayTreeWindowState;
use ui::mcst_tree_display::tree_app::display_tree_window_system;

// Module imports
mod components;
mod ui {
    pub(crate) mod player_ui {
        pub mod player_health_bar;
    }
    pub mod mcst_tree_display{
        pub mod mcst_tree_display;
        pub mod tree_app;
    }
    pub mod camera;
    pub mod npc_click;
    pub mod setup_ui;
}
mod gameworld {
    pub mod highlight;
    pub mod position;
    pub mod tile;
    pub mod tile_types;
    pub mod world;
}
mod debug;
mod errors;
mod npcs {
    pub(crate) mod npc_components {
        pub mod action_rating;
        pub mod gene_type;
        pub mod genes;
        pub mod npc_action;
        pub mod npc_base;
        pub mod npc_status;
        pub mod npc_type;
        pub mod opinions;
        pub mod target;
    }
    pub mod agent;
    pub mod monster;
    pub mod player;
    pub mod treasure;
}
mod system {
    pub(crate) mod player_system {
        pub mod handle_player_movement;
        pub mod highlight_squares;
    }
    pub(crate) mod mcst_tree {
        pub mod mcst_node;
        pub mod mcst_tree;
        pub mod simulation_tree;
    }
    pub(crate) mod mcst_system {
        pub mod backpropegation;
        pub mod expansion;
        pub mod selection;
        pub mod simulation;
    }
    pub(crate) mod pathfinding {
        pub mod pathfinding_calculation;
    }
    pub mod setup;

    pub(crate) mod simulation {
        pub mod handle_npc_movement;
        pub mod handle_selected_action;
    }
}
mod tests {

    pub mod check_mcst_trees_system;
}

use crate::npcs::agent::Agent;
use clap::Parser;
use gameworld::position::Position;
use gameworld::world;
use npcs::npc_components::npc_action::NpcAction;
use npcs::npc_components::npc_base::NPCBase;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::VecDeque;
use system::mcst_tree::simulation_tree::SimulationTree;
use system::pathfinding::pathfinding_calculation::a_star_pathfinding;
use system::setup::check_npc_count;
use system::setup::setup;
use ui::camera::{camera_drag_system, setup_camera, CameraDragging};
use ui::npc_click::npc_click_system;
use ui::npc_click::update_selected_npc_text;
use ui::player_ui::player_health_bar::setup_player_health_ui;
use ui::setup_ui::end_turn_button_system;
use ui::setup_ui::setup_ui;
use ui::setup_ui::PanelState;
use world::GameWorld;

#[derive(Resource, Default)]
pub struct PlayerMoved(pub bool);

#[derive(Resource)]
pub struct MCSTFlag(pub bool);

#[derive(Resource)]
pub struct RunningFlag(pub bool);

#[derive(Resource)]
pub struct FinishedSelectionPhase(pub bool);

#[derive(Resource)]
pub struct FinishedSelectingActions(pub bool);

#[derive(Resource)]
pub struct FinishedRunningFlag(pub bool);

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
pub struct NpcActions(pub Vec<(u32, VecDeque<NpcAction>)>);

#[derive(Resource)]
pub struct NpcActionsCopy(pub Vec<(u32, VecDeque<NpcAction>)>);

#[derive(Resource)]
pub struct ScoreTracker(pub Vec<(u32, i32)>);

#[derive(Resource)]
struct WorldRandom(StdRng);

#[derive(Default, Resource)]
pub struct IterationCount(pub i32);
#[derive(Resource, Default)]
pub struct SelectedNPC(pub Option<Entity>);

#[derive(Resource)]
pub struct EndTurn(pub bool);

#[derive(Resource, Default)]
pub struct SystemMove(pub bool);

#[derive(Resource, Default)]
pub struct HighlightMovement(pub bool);

impl WorldSim {
    pub fn get_world(&self) -> &GameWorld {
        &self.0
    }

    pub fn copy_world(&self, world: &GameWorld) -> WorldSim {
        WorldSim(GameWorld {
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
    #[arg(short, long, default_value_t = 0)]
    seed: u64,
}

/// A timer resource for querying NPCs
#[derive(Resource, Default)]
pub struct QueryTimer(pub Timer);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    PlayerTurn,
    MCTSPhase,
    ExecutionPhase,
    EndTurn,
}

#[allow(dead_code)]
fn main() {
    let game_world = world::initialize("test").expect("Failed to initialize the game world");
    let start = Position { x: 0, y: 1 };
    let goal = Position { x: 0, y: 3 };

    let path = a_star_pathfinding(&game_world, start, goal);

    if path.is_empty() {
        println!("No path found!");
    } else {
        println!("Path found: {:?}", path);
    }

    // Parse the command-line arguments
    let args = Args::parse();

    // Use the seed from the command-line or generate a random seed
    let seed = if args.seed != 0 {
        args.seed
    } else {
        rand::thread_rng().gen()
    };
    println!("Using seed: {}", seed);

    // Initialize a seeded RNG
    let rng = StdRng::seed_from_u64(seed);

    // Begin building the Bevy app using App::new().
    App::new()
        // Set the window properties, such as title, width, and height.
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Thesis".to_string(),
                    resolution: (800., 600.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            EguiPlugin,
        ))
        // Insert various resources
        .insert_resource(game_world)
        .insert_resource(WorldSim(GameWorld::new()))
        .insert_resource(SimulationTree::default())
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        .insert_resource(MCSTFlag(true))
        .insert_resource(RunningFlag(false))
        .insert_resource(SelectedNPC(None))
        .insert_resource(PanelState {
            is_collapsed: false,
        })
        .insert_resource(MCSTCurrent(0))
        .insert_resource(MCSTTotal(0))
        .insert_resource(IterationCurrent(0))
        .insert_resource(FinishedSelectionPhase(false))
        .insert_resource(FinishedSelectingActions(false))
        .insert_resource(AgentList(Vec::new()))
        .insert_resource(RunningFlag(false))
        .insert_resource(FinishedRunningFlag(false))
        .insert_resource(Backpropogate(false))
        .insert_resource(NpcActions(Vec::new()))
        .insert_resource(NpcActionsCopy(Vec::new()))
        .insert_resource(ScoreTracker(Vec::new()))
        .insert_resource(IterationTotal(3))
        .insert_resource(IterationCount(0))
        .insert_resource(WorldRandom(rng))
        .insert_resource(QueryTimer(Timer::from_seconds(0.1, TimerMode::Once))) // Delay by 0.1 seconds
        .insert_resource(EndTurn(false)) // Add the EndTurn resource
        .insert_resource(SystemMove(false)) // Initialize the flag as false
        .insert_resource(HighlightMovement(true))
        .insert_resource(DisplayTreeWindowState::default()) 
        // .insert_resource(DisplayTreeWindowState::default()) // Initialize HighlightMovement as true
        // Add systems using system sets for labels and ordering
        .add_systems(
            Startup,
            (
                setup,
                |mut commands: Commands| {
                    setup_camera(&mut commands, 16.0, 16.0);
                },
                setup_ui,
                setup_player_health_ui,
                check_npc_count.after(setup),
            ),
        )
        .add_systems(
            Update,
            (
                // UI Elements
                camera_drag_system,
                npc_click_system,
                update_selected_npc_text,
                end_turn_button_system,
                highlight_moveable_player_squares,
                move_player,
                selection_system,
                expansion_system,
                handle_selected_action_system,
                handle_agent_movement,
                update_agent_action_button_visibility,
                agent_action_button_system,
                display_tree_window_system,
                // movement_system
                // action_system
                // backpropegate_system,
                //
            ),
        )
        // .add_systems(Update, (
        //     // MCTS Phase
        //
        //     expansion_system,
        //     backpropegate_system,
        //     // delayed_check_npc_count,
        // ))
        .run();
}

pub fn delayed_check_npc_count(
    time: Res<Time>,
    mut timer: ResMut<QueryTimer>,
    npc_query: Query<(Entity, &NPCBase)>,
) {
    if timer.0.tick(time.delta()).finished() {
        println!("Number of NPCs queried: {}", npc_query.iter().count());
        for (entity, npc_base) in npc_query.iter() {
            println!(
                "Queried NPC - Entity: {:?}, Type: {:?}, Position: {:?}",
                entity, npc_base.npc_type, npc_base.position
            );
        }
    }
}
// Start Selection Phase
