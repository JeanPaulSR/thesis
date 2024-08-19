#![allow(dead_code)]
use bevy::prelude::*;
use camera::camera_drag_system;
use camera::CameraDragging;
mod tile;
mod components;
mod camera;
mod world;
mod debug;
use mcst_system::backpropogate::backpropgate;
use mcst_system::backpropogate::check_simulation_finish;
use mcst_system::simulation::run_actual;
use mcst_system::simulation::{run_simulation, set_simulation_actions};
use mcst_system::systems::agent_message_system;
use mcst_system::systems::cleanup_system;
use mcst_system::systems::monster_message_system;
use mcst_system::systems::treasure_message_system;
use world::World;
mod movement; 
mod errors;
use std::iter::empty;
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
use mcst_system::selection_expansion::select_expansion;

use mcst_system::setup::setup_tree;
use mcst_system::setup::check_end;
use mcst_system::setup::change_state;
use mcst_system::setup::setup;
use crate::mcst_system::mcst::NpcAction;
use crate::mcst_system::mcst;
use crate::mcst_system::mcst::SimulationTree;
use crate::entities::agent::Agent;



//Flag that tells if the program is in the mcst phase
pub struct MCSTFlag(bool);
//Flag that tells if the program is in the execute found action phase
pub struct RunningFlag(bool);
//Flag that tells selection phase of MCST has been completed
pub struct FinishedSelectionPhase(bool);
//Flag that tells if all
pub struct FinishedSelectingActions(bool);
//Flag that marks that the program has finished running all phases
pub struct FinishedAllFlag(bool);
//Flag that marks that the program has finished running execution phases
pub struct FinishedRunningFlag(bool);
//Flag that marks that the program has finished running simulation phases
pub struct FinishedSimulationFlag(bool);
pub struct Backpropogate(bool);
//Current mcst simulation count
pub struct MCSTCurrent(i32);
//Total number of mcst simulations
pub struct MCSTTotal(i32);
//Current execution iteration count
pub struct IterationCurrent(i32);
//Total number of execution iterations
pub struct IterationTotal(i32);
pub struct WorldSim(World);
pub struct NpcActions(Vec<(u32, VecDeque<mcst::NpcAction>)>);
pub struct NpcActionsCopy(Vec<(u32, VecDeque<mcst::NpcAction>)>);
pub struct ScoreTracker(Vec<(u32, i32)>);

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
        //Insert the world tree
        .insert_resource(SimulationTree::new_empty())
        // Insert a CameraDragging resource to track the camera dragging state.
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        
        
        .insert_resource(IterationTotal(0))
        // Add a system that handles camera drag functionality.
        .add_system(camera_drag_system.system())
        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(setup.system())
        
        

        // // Insert AgentMessages resource with an empty vector.
        // .insert_resource(AgentMessages::new())
        // // Insert MonsterMessages resource with an empty vector.
        // .insert_resource(MonsterMessages::new())
        // // Insert TreasureMessages resource with an empty vector.
        // .insert_resource(TreasureMessages::new())
        
        // //End simulation key
        
        // .insert_resource(MCSTCurrent(0))
        // .insert_resource(MCSTTotal(0))
        // .insert_resource(IterationCurrent(0))
        // .insert_resource(FinishedSelectionPhase(false))
        // .insert_resource(FinishedSelectingActions(false))
        // .insert_resource(Vec::<Agent>::new())
        // .insert_resource(MCSTFlag(false))
        // .insert_resource(RunningFlag(false))
        // .insert_resource(FinishedRunningFlag(false))
        // .insert_resource(FinishedSimulationFlag(false))
        // .insert_resource(Backpropogate(false))
        // .insert_resource(NpcActions(Vec::new()))
        // .insert_resource(NpcActionsCopy(Vec::new()))
        // .insert_resource(ScoreTracker(Vec::new()))
        // .add_system(toggle_flag_system.system())
        
        
        // Setup System
        //.add_system(setup_tree.system().label("setup_tree"))
        //.add_system(check_end.system().after("setup_tree").label("check_end"))
        //.add_system(change_state.system().after("check_end").label("change_state"))
        //.add_system(select_expansion.system().after("change_state").label("selection_expansion_phase"))
        //Simulation phase
        //.add_system(set_simulation_actions.system().after("selection_expansion_phase").label("set_simulation_actions"))
        //.add_system(run_simulation.system().after("set_simulation_actions").label("simulation_actions"))

        //.add_system(check_simulation_finish.system().after("simulation_actions").label("check_simulation_end"))
        //.add_system(backpropgate.system().after("check_simulation_end").label("backpropegate"))

        //Running Phase
        //.add_system(run_actual.system().after("backpropegate").label("running_actions"))


        // // Add the agent message system to handle messages after actions.
        // .add_system(treasure_message_system.system().after("action").label("message"))
        // .add_system(monster_message_system.system().after("action").label("message"))
        // .add_system(agent_message_system.system().after("action").label("message"))
        // //Add the despawn handler after all message systems
        // .add_system(cleanup_system.system().after("message"))
        
        //.add_system(debug_system.system().after("running_actions").label("debug"))
        //)
        .run();
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
            //agent.set_agent_target_id(2);
            //agent.set_target(entities::agent::Target::Agent);
            //agent.set_action(NpcAction::AttackMonster);
            //agent.perform_action(world, commands, agent_messages);
        }
    }

}

fn debug_system(
    mut running_flag: ResMut<RunningFlag>,
    mut agent_query: Query<&mut Agent>, 
) {
    //running_flag.0 = false;    
    // for mut agent in agent_query.iter_mut(){
    //     agent.set_action(NpcAction::None);
    // }

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
