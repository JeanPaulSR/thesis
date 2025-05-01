use std::collections::HashMap;

use bevy::ecs::system::Resource;

use super::mcst_tree::MCTSTree;

#[derive(Resource, Default)]
pub struct SimulationTree {
    pub trees: HashMap<i32, MCTSTree>,
}

impl SimulationTree {
    pub fn add_tree(&mut self, agent_id: i32, tree: MCTSTree) {
        self.trees.insert(agent_id, tree);
    }

    pub fn get_tree(&self, agent_id: i32) -> Option<&MCTSTree> {
        self.trees.get(&agent_id)
    }

    pub fn get_tree_mut(&mut self, agent_id: i32) -> Option<&mut MCTSTree> {
        self.trees.get_mut(&agent_id)
    }

    pub fn trees_mut(&mut self) -> impl Iterator<Item = &mut MCTSTree> {
        self.trees.values_mut()
    }
}