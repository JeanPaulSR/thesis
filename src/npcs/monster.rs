use bevy::prelude::*;

use crate::{gameworld::position::Position, npcs::npc_components::npc_status::Status};

#[derive(Clone, Component)]
pub struct Monster {
    id: i32,
    start_position: Position,
    current_position: Position,
    target_id: i32,
    reward: u32,
    status: Status,
}

static mut M_COUNTER: i32 = 0;

impl Monster {
    pub fn new_monster(start_position: Position) -> Self {

        Monster {
            id: unsafe { M_COUNTER },
            reward: 0,
            status: Status::Idle,
            start_position,
            current_position: start_position,
            target_id: i32::MAX,
        }
    }


    // Function to get the id of the monster
    pub fn get_id(&self) -> i32 {
        self.id
    }

    // Function to set the status of the monster
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    // Function to get the status of the monster
    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    // Function to set the reward of the monster
    pub fn set_reward(&mut self, reward: u32) {
        self.reward = reward;
    }

    // Function to get the reward of the monster
    pub fn get_reward(&self) -> u32 {
        self.reward
    }

    // Function to add reward to the monster
    pub fn add_reward(&mut self, reward: u32) {
        self.reward = self.reward + reward;
    }

    // Function to remove reward to the monster
    pub fn remove_reward(&mut self, reward: u32) {
        self.reward = self.reward.saturating_sub(reward);
    }

    pub fn set_target_id(&mut self, target_id: i32) {
        self.target_id = target_id;
    }

    pub fn get_target_id(&self) -> i32 {
        self.target_id
    }
}
