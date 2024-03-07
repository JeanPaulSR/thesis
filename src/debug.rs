use bevy::prelude::*;
use bevy::app::{AppBuilder, Plugin};
use crate::debug_system;

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
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(debug_system.system());
    }
}

