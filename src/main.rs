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
use world::World;
mod movement; 
mod mcst;
mod entities {
    pub mod monster;
    pub mod agent;
}

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
         .insert_resource(World {agents: Vec::new(), grid: world::create_world(),})
        

         
         // Add a system that handles camera drag functionality.
         .add_system(camera_drag_system.system())

        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(world::setup.system())
        
        // Add a system that moves agents to a village.
        .add_startup_system(npc::debug.system())

        // Add the DebugPlugin to the app.
        //.add_plugin(DebugPlugin)
        
       
        // Insert a CameraDragging resource to track the camera dragging state.
        .insert_resource(CameraDragging {
            is_dragging: false,
            previous_mouse_position: None,
        })
        // Run the app. This starts the game loop and executes all systems in the proper order.
        .run();
}


