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
//use debug::DebugPlugin;
use world::WorldGrid;
use world::Agents;
use crate::tile::TileType;
use crate::components::Position;
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use bevy::asset::AssetServer;
use crate::npc::Agent;

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

         // Insert a WorldGrid resource that contains the game world's grid.
         .insert_resource(WorldGrid {grid: world::create_world_grid(),})
        
        
         .insert_resource(Agents { vec: Vec::new() })

         
         // Add a system that handles camera drag functionality.
         .add_system(camera_drag_system.system())

        // Add a startup system that sets up the initial state of the game (e.g., camera, entities, etc.).
        .add_startup_system(world::setup.system())
        
        // Add a system that moves agents to a village.
        .add_startup_system(agent_test.system())
        //.add_startup_system(test_create.system())

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

fn move_sprite_handle(
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) -> Handle<ColorMaterial> {
    
    materials.add(asset_server.load("textures/agent.png").into())
}

fn agent_test(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn an agent using the `new_random` function
    let mut agent = Agent::new_random(5.0 * 32.0, 5.0 * 32.0, &mut commands, &mut materials, &asset_server);
    
    let agent2 = Agent::new_random(5.0 * 32.0, 5.0 * 32.0, &mut commands, &mut materials, &asset_server);
    let new_transform = Transform::from_translation(Vec3::new(5.0* 32.0, 6.0, 1.0));
    update_agent_transform(&mut agent, new_transform, &mut commands);
}

//Update based on the agent
//Remember to multiply by 32
fn update_agent_transform(
    agent: &mut Agent,
    new_transform: Transform,
    commands: &mut Commands,
) {
    agent.transform = new_transform;
    commands.entity(agent.entity).insert(agent.transform.clone());
}

