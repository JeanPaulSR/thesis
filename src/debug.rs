use crate::debug_system;
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

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(debug_system);
    }
}
