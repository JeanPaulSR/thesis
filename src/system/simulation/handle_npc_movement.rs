use bevy::prelude::*;
use crate::gameworld::position::Position;
use crate::npcs::agent::Agent;
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::npc_components::npc_status::Status;

pub fn handle_agent_movement(
    mut query: Query<(&mut Agent, &mut NPCBase)>, // Query for agents and their NPCBase
    mut commands: Commands,
) {
    for (mut agent, mut npc_base) in query.iter_mut() {
        // Check if the agent's status is Moving
        if agent.get_status() == Status::Moving {
            // Get the agent's path
            if let Some(mut path) = agent.get_path() {
                // Pop the first position in the path
                if let Some(next_position) = path.first() {
                    // Move the NPCBase to the next position
                    npc_base.move_to(next_position.x, next_position.y, &mut commands);

                    // Remove the position from the path
                    path.remove(0);

                    // Update the agent's path
                    agent.set_path(path);

                    // If the path is empty, set the agent's status to Idle
                    if agent.get_path().is_none() || agent.get_path().unwrap().is_empty() {
                        agent.set_status(Status::Idle);
                    }
                }
            }
        }
    }
}