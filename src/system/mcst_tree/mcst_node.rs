use std::sync::{Arc, Mutex};
use crate::npcs::npc_components::npc_action::NpcAction;
use crate::npcs::npc_components::npc_type::NPCType;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub enum NodeType {
    ActionNode {
        action: NpcAction,
    },
    InformationNode {
        action_taken: NpcAction,
        npc_type: NPCType,
        npc_id: u32,
    },
    NullNode,
}

impl NodeType {
    pub fn to_string(&self) -> String {
        match self {
            NodeType::ActionNode { action } => format!("ActionNode: {:?}", action),
            NodeType::InformationNode { action_taken, npc_type, npc_id } => {
                format!("InformationNode: {:?}, NPC Type: {:?}, NPC ID: {}", action_taken, npc_type, npc_id)
            }
            NodeType::NullNode => "NullNode".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub visits: usize,
    pub total_reward: u32,
    pub depth: u32,
    pub parent: Option<Arc<Mutex<Node>>>,
    pub children: Vec<Arc<Mutex<Node>>>,
}

impl Node {
    /// Creates a new node.
    pub fn new(node_type: NodeType, depth: u32, parent: Option<Arc<Mutex<Node>>>) -> Self {
        Node {
            node_type,
            visits: 0,
            total_reward: 0,
            depth,
            parent,
            children: Vec::new(),
        }
    }

    /// Returns the children of the node.
    pub fn get_children(&self) -> &Vec<Arc<Mutex<Node>>> {
        &self.children
    }

    /// Checks if the node has a specific child.
    pub fn has_child(&self, node_type: &NodeType) -> bool {
        self.children.iter().any(|child| {
            let child_lock = child.lock().unwrap();
            &child_lock.node_type == node_type
        })
    }

    /// Checks if the node has a parent.
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// Returns a reference to the parent node, if it exists.
    pub fn get_parent(&self) -> Option<&Arc<Mutex<Node>>> {
        self.parent.as_ref()
    }

    /// Removes the parent reference from the node.
    pub fn remove_parent(&mut self) {
        self.parent = None;
    }

    /// Adds a child node to the current node.
    /// Updates the depth of the child based on the parent's depth and the child type.
    /// Returns an error if a similar node type already exists.
    pub fn add_child(&mut self, child: Arc<Mutex<Node>>) -> Result<(), &'static str> {
        let mut child_lock = child.lock().unwrap();

        // Ensure no duplicate child node types exist.
        if self.has_child(&child_lock.node_type) {
            return Err("A similar node type already exists as a child.");
        }

        // Update the depth of the child node.
        if let NodeType::ActionNode { .. } = child_lock.node_type {
            child_lock.depth = self.depth + 1;
        } else {
            child_lock.depth = self.depth;
        }

        drop(child_lock);
        self.children.push(child);
        Ok(())
    }

    /// Calculates the height of the tree starting from this node.
    pub fn calculate_height(&self) -> u32 {
        if self.children.is_empty() {
            1
        } else {
            1 + self.children.iter().map(|child| {
                let child_lock = child.lock().unwrap();
                child_lock.calculate_height()
            }).max().unwrap_or(0)
        }
    }

    /// Backpropagates the reward up the tree, averaging rewards based on visits.
    pub fn backpropagate(&mut self, reward: u32) {
        self.visits += 1;
        self.total_reward += reward;

        if let Some(parent) = &self.parent {
            let mut parent_lock = parent.lock().unwrap();
            parent_lock.backpropagate(reward / self.visits as u32);
        }
    }


    /// Recursively calculates the Monte Carlo score for each action and applies UCT.
    pub fn calculate_monte_carlo(&self, exploration_constant: f64) -> Option<NpcAction> {
        // Map to store total rewards and visit counts for each action
        let mut action_rewards: HashMap<NpcAction, (u32, usize)> = HashMap::new();

        // Recursively traverse children
        for child in &self.children {
            let child_lock = child.lock().unwrap();

            // If the child is an ActionNode, propagate its reward and visits
            if let NodeType::ActionNode { action } = child_lock.node_type {
                let total_reward = child_lock.total_reward;
                let visits = child_lock.visits;

                // Update the action_rewards map
                let entry = action_rewards.entry(action).or_insert((0, 0));
                entry.0 += total_reward; // Accumulate rewards
                entry.1 += visits;       // Accumulate visits
            } else {
                // Recursively calculate rewards for non-ActionNodes
                child_lock.calculate_monte_carlo(exploration_constant);
            }
        }

        // Calculate UCT scores for each action
        let mut best_action = None;
        let mut best_score = f64::MIN;

        for (action, (total_reward, visits)) in action_rewards {
            let uct_score = if visits == 0 {
                // Assign a very high score to unexplored actions
                f64::INFINITY
            } else {
                // Calculate average reward and UCT score
                let average_reward = total_reward as f64 / visits as f64;
                let parent_visits = self.visits.max(1); // Avoid division by zero
                average_reward
                    + exploration_constant * ((parent_visits as f64).ln() / visits as f64).sqrt()
            };

            if uct_score > best_score {
                best_score = uct_score;
                best_action = Some(action);
            }
        }

        best_action
    }

    /// Selects the best action based on Monte Carlo scores and UCT, and returns the corresponding child node.
    pub fn select_action(&self, exploration_constant: f64) -> Option<Arc<Mutex<Node>>> {
        // Calculate the best action using Monte Carlo
        let best_action = self.calculate_monte_carlo(exploration_constant)?;

        // Find the child node corresponding to the best action
        for child in &self.children {
            let child_lock = child.lock().unwrap();
            if let NodeType::ActionNode { action } = &child_lock.node_type {
                if *action == best_action {
                    return Some(child.clone());
                }
            }
        }

        None
    }

    /// Expands the current node by adding a new child node with the given action.
    pub fn expand(&mut self, action: NpcAction) -> Arc<Mutex<Node>> {
        let new_node = Arc::new(Mutex::new(Node::new(
            NodeType::ActionNode { action },
            self.depth + 1,
            Some(Arc::new(Mutex::new(self.clone()))),
        )));
        self.children.push(new_node.clone());
        new_node
    }

    /// Recursively updates the depth of the current node and its children.
    /// Increments the depth only if the current node is an ActionNode.
    pub fn update_depth(&mut self, parent_depth: u32) {
        // Update the depth based on the node type.
        if let NodeType::ActionNode { .. } = self.node_type {
            self.depth = parent_depth + 1; // Increment depth for ActionNode.
        } else {
            self.depth = parent_depth; // Inherit depth for InformationNode.
        }

        // Recursively update the depth of children.
        for child in &self.children {
            let mut child_lock = child.lock().unwrap();
            child_lock.update_depth(self.depth);
        }
    }
    
    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }
    
    pub fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }
}
