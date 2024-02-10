use crate::entities::agent::Target;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
pub enum NpcAction {
    Attack,
    Steal,
    Rest,
    Talk,
    None,
}

#[derive(Clone)]
pub struct ActionTaken{
    action: NpcAction,
    target: Target,
    target_id: u32,
    tile_target: Option<(u32, u32)>,
}

impl ActionTaken {
    fn new() -> Self {
        ActionTaken {
            action: NpcAction::None,
            target: Target::None,
            target_id: 0,
            tile_target: None,
        }
    }

    pub fn new_constructor(action: NpcAction, target: Target, target_id: u32, tile_target: Option<(u32, u32)>) -> Self {
        ActionTaken {
            action,
            target,
            target_id,
            tile_target,
        }
    }
}

pub struct SimpleAgent{
    agent_id: u32,
    actions_chance: Vec<ActionTaken>,
}

 #[derive(Clone)]
pub struct ActionSet{
    actions: HashMap<u32, ActionTaken>,
}

impl ActionSet {
    fn new() -> Self {
        ActionSet {
            actions: HashMap::new(),
        }
    }

    fn set_action(&mut self, agent_id: u32, action_taken: ActionTaken) -> Result<(), &'static str> {
        if self.actions.len() < self.actions.capacity() {
            self.actions.insert(agent_id, action_taken);
            Ok(())
        } else {
            Err("ActionSet hashmap is full")
        }
    }

    fn update_action(&mut self, agent_id: u32, action_taken: ActionTaken) -> Result<(), &'static str> {
        if let Some(existing_action) = self.actions.get_mut(&agent_id) {
            *existing_action = action_taken;
            Ok(())
        } else {
            Err("No action found for the provided agent ID")
        }
    }

    fn get_action(&self, agent_id: u32) -> Result<&ActionTaken, &'static str> {
        match self.actions.get(&agent_id) {
            Some(action) => Ok(action),
            None => Err("No action found for the provided agent ID"),
        }
    }
}

pub type MCTSNodeRef = Rc<RefCell<MCTSNode>>;

#[derive(Clone)]
pub struct MCTSNode {
    actions: Option<ActionSet>,
    visits: usize,
    total_reward: u32,
    parent: Option<Arc<Mutex<MCTSNode>>>,
    children: Vec<Arc<Mutex<MCTSNode>>>,
}

impl MCTSNode {
    pub fn new(actions: Option<ActionSet>) -> Self {
        MCTSNode {
            actions,
            visits: 0,
            total_reward: 0,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn depth(&self) -> usize {
        self.depth_recursive(0)
    }

    fn depth_recursive(&self, current_depth: usize) -> usize {
        match &self.parent {
            Some(parent_node) => {
                let parent_node = parent_node.lock().unwrap();
                parent_node.depth_recursive(current_depth + 1)
            }
            None => current_depth,
        }
    }

    pub fn select_child(&self) -> Option<Arc<Mutex<MCTSNode>>> {
        // Implement node selection logic here
        unimplemented!()
    }

    pub fn expand(&mut self) -> Arc<Mutex<MCTSNode>> {
        // Implement node expansion logic here
        unimplemented!()
    }

    pub fn simulate(&self) -> u32 {
        // Implement simulation logic here
        unimplemented!()
    }

    pub fn backpropagate(&mut self, reward: u32) {
        // Implement backpropagation logic here
        unimplemented!()
    }

    pub fn best_child(&self, exploration_constant: f64) -> Option<Arc<Mutex<MCTSNode>>> {
        // Implement best child selection logic here
        unimplemented!()
    }
}

pub struct MCTSTree {
    root: Option<Arc<Mutex<MCTSNode>>>,
    exploration_constant: f64,
}

impl MCTSTree {
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    pub fn insert_node(&mut self, node: MCTSNode) {
        let node_arc = Arc::new(Mutex::new(node));
        self.root = Some(node_arc);
    }
    
    pub fn new_empty() -> Self {
        MCTSTree {
            root: None,
            exploration_constant: 1.0, // Set default value for exploration constant
        }
    }

    pub fn new(root_actions: Option<ActionSet>, exploration_constant: f64) -> Self {
        let root_node = match root_actions {
            Some(actions) => MCTSNode::new(Some(actions)),
            None => MCTSNode::new(None), // Pass None to indicate no actions
        };
    
        MCTSTree {
            root: Some(Arc::new(Mutex::new(root_node))),
            exploration_constant,
        }
    }
}