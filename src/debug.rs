use bevy::prelude::*;

pub fn debug(command: &str) {
    match command {
        "test_movement" => test_movement(),
        _ => println!("Invalid debug command"),
    }
}

fn test_movement() {
    println!("Running test_movement");
}

fn debug_system() {
    debug("test_movement");
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_system);
    }
}