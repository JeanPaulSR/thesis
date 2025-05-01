use bevy::ecs::component::Component;


#[derive(Clone, Component)]
pub struct Treasure {
    id: i32,
    reward: u32,
}

static mut T_COUNTER: i32 = 0;

impl Treasure {
    // Function to create a treasure
    pub fn new_treasure() -> Self {

        unsafe {
            T_COUNTER += 1;
        }
        Treasure {
            id: unsafe { T_COUNTER },
            reward: 100,
        }
    }

    // Function to set the reward of the treasure
    pub fn set_reward(&mut self, reward: u32) {
        self.reward = reward;
    }

    // Function to get the reward of the treasure
    pub fn get_reward(&self) -> u32 {
        self.reward
    }

    // Function to add reward to the treasure
    pub fn add_reward(&mut self, reward: u32) {
        self.reward = self.reward + reward;
    }

    // Function to remove reward from the treasure
    pub fn remove_reward(&mut self, reward: u32) {
        self.reward = self.reward.saturating_sub(reward);
    }

    // Function to get the id of the treasure
    pub fn get_id(&self) -> i32 {
        self.id
    }

    // ______      _     _ _
    // | ___ \    | |   | (_)
    // | |_/ /   _| |__ | |_  ___
    // |  __/ | | | '_ \| | |/ __|
    // | |  | |_| | |_) | | | (__
    // \_|   \__,_|_.__/|_|_|\___|

    // Function to print the treasure
    pub fn print(&self) {
        println!("Treasure ID: {}", self.id);
    }
}
