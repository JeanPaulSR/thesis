use bevy::ecs::system::{Res, ResMut};
use std::{thread, time::Duration};

use crate::system::mcst_tree::simulation_tree::SimulationTree;

pub fn check_mcst_trees_system(simulation_tree: Res<SimulationTree>) {
    // Check if all MCTS trees are no longer in the selection phase.
    let all_trees_done = simulation_tree
        .trees
        .values()
        .all(|tree| !tree.is_in_selection_phase());

    if all_trees_done {
        println!("All MCTS trees are no longer in the selection phase. Pausing for 10 seconds...");
        thread::sleep(Duration::from_secs(10)); // Pause for 10 seconds
    }
}