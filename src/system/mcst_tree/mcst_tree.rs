use std::sync::{Arc, Mutex};

use super::mcst_node::{Node, NodeType};

pub struct MCTSTree {
    pub root: Option<Arc<Mutex<Node>>>,
    pub current_node: Option<Arc<Mutex<Node>>>,
    pub in_selection_phase: bool,
    pub ready_for_expansion: bool,
}

impl Clone for MCTSTree {
    fn clone(&self) -> Self {
        MCTSTree {
            root: self.root.as_ref().map(|node| Arc::clone(node)),
            current_node: self.current_node.as_ref().map(|node| Arc::clone(node)),
            in_selection_phase: self.in_selection_phase,
            ready_for_expansion: self.ready_for_expansion,
        }
    }
}

impl MCTSTree {
    /// Creates a new MCTSTree with a "null" root node.
    pub fn new() -> Self {
        let null_node = Arc::new(Mutex::new(Node::new(
            NodeType::NullNode,
            0,
            None,
        )));
        MCTSTree {
            root: Some(null_node.clone()),
            current_node: Some(null_node),
            in_selection_phase: true, // Start in the selection phase
            ready_for_expansion: false, // Initially not ready for expansion
        }
    }

    /// Sets the root node of the tree and removes its parent reference.
    /// Recursively updates the depth of the tree starting from the root.
    pub fn set_root(&mut self, node: Arc<Mutex<Node>>) {
        let mut root_lock = node.lock().unwrap();

        // Ensure the root node is not an InformationNode.
        if let NodeType::InformationNode { .. } = root_lock.node_type {
            panic!("The root node cannot be an InformationNode.");
        }

        // Remove the parent reference and set the root.
        root_lock.remove_parent();
        root_lock.update_depth(1); // Start depth from 1.
        drop(root_lock);

        self.root = Some(node.clone());
        self.current_node = Some(node);
    }

    /// Returns a reference to the root node of the tree.
    pub fn get_root(&self) -> Option<&Arc<Mutex<Node>>> {
        self.root.as_ref()
    }

    /// Returns a reference to the current node of the tree.
    pub fn get_current_node(&self) -> Option<&Arc<Mutex<Node>>> {
        self.current_node.as_ref()
    }

    /// Sets the current node of the tree.
    pub fn set_current_node(&mut self, node: Arc<Mutex<Node>>) {
        self.current_node = Some(node);
    }

    /// Resets the current node to the root node.
    pub fn reset_current_node_to_root(&mut self) {
        if let Some(root) = &self.root {
            self.current_node = Some(root.clone());
        }
    }

    /// Checks if the tree has a root node.
    pub fn has_root(&self) -> bool {
        self.root.is_some()
    }

    /// Calculates the height of the tree starting from the root node.
    pub fn get_height(&self) -> u32 {
        if let Some(root) = &self.root {
            let root_lock = root.lock().unwrap();
            root_lock.calculate_height()
        } else {
            0
        }
    }

    /// Initializes the tree with a "null" root node.
    pub fn initialize_tree(&mut self) {
        let null_node = Arc::new(Mutex::new(Node::new(
            NodeType::NullNode, 
            0,
            None,
        )));
        self.set_root(null_node);
    }

    /// Wipes the reference to the current node, effectively removing it.
    pub fn wipe_current_node(&mut self) {
        self.current_node = None;
    }

    /// Sets the root node as the current node.
    pub fn set_root_current_node(&mut self) {
        if let Some(root) = &self.root {
            self.current_node = Some(root.clone());
        }
    }

    /// Checks if the tree is in the selection phase.
    pub fn is_in_selection_phase(&self) -> bool {
        self.in_selection_phase
    }

    /// Sets the selection phase flag.
    pub fn set_in_selection_phase(&mut self, in_selection_phase: bool) {
        self.in_selection_phase = in_selection_phase;
    }

    /// Checks if the tree is ready for expansion.
    pub fn is_ready_for_expansion(&self) -> bool {
        self.ready_for_expansion
    }

    /// Sets the ready-for-expansion flag.
    pub fn set_ready_for_expansion(&mut self, ready: bool) {
        self.ready_for_expansion = ready;
    }
}
