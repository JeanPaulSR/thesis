use crate::mcst_system::mcst::{ActionRating, NpcAction};

use crate::entities::agent::{Agent, Opinions};
use crate::entities::agent::Genes;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::mcst_node::MCTSNode;

pub struct MCTSTree {
    root: Option<Arc<Mutex<MCTSNode>>>,
    genes: Option<Genes>,
    action_rating: Option<ActionRating>,
    exploration_constant: f64,
    height: u16,
    //nodes selected
}

impl MCTSTree {
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert_root(&mut self, node: MCTSNode) {
        let node_arc = Arc::new(Mutex::new(node));
        self.root = Some(node_arc);
    }

    pub fn set_genes(&mut self, genes: Genes) {
        self.genes = Some(genes);
    }

    pub fn new_empty() -> Self {
        MCTSTree {
            root: None,
            genes: None,
            action_rating: None,
            exploration_constant: 1.0,
            height: 0,
        }
    }

    pub fn initialize_tree(&mut self, agent: Agent) {
        self.genes = Some(agent.get_genes().clone());
        let mut action_rating = ActionRating::new();
        action_rating.generate_ratings(agent.get_genes().clone());

        let node = MCTSNode::new(None, action_rating);
        self.insert_root(node);
    }

    pub fn selection_phase(&mut self, opinions: Opinions) -> VecDeque<NpcAction> {
        let mut root = self.root.as_ref().unwrap().lock().unwrap();
        root.select(opinions)
    }

    pub fn choose_action(&mut self) -> NpcAction {
        let root = self.root.as_ref().unwrap().lock().unwrap();
        root.choose_action()
    }

    //SET THE BASE REWARD HERE
    pub fn set_selected_node_as_root(&mut self, selected_action: NpcAction) {
        let mut root = self.root.as_ref().unwrap().lock().unwrap();
    
        // Find the child corresponding to the selected action
        if let Some(selected_child_index) = root.get_children().iter().position(|child| {
            let child_node = child.lock().unwrap();
            child_node.get_action() == Some(selected_action)
        }) {
            // Get the children as mutable and remove the selected child
            let selected_child = root.get_children().remove(selected_child_index);
    
            // Explicitly drop the borrow of `root`
            std::mem::drop(root);
    
            // Now modify `self.root`
            self.root = Some(selected_child);
        } else {
            //println!("Error: No child with the selected action was found.");
        }
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn calculate_height(&mut self) {
        let mut root = self.root.as_ref().unwrap().lock().unwrap();
        root.calculate_height();
        self.height = root.get_height();
    }
    pub fn expand(&mut self) -> Arc<Mutex<MCTSNode>> {
        unimplemented!()
    }

    //Try recursive instead
    pub fn backpropegate(&mut self, actions: VecDeque<NpcAction>, score: u32) {
        if actions.is_empty() {
            panic!("Passed empty actions to backpropegate")
        }

        let mut root = self.root.as_ref().unwrap().lock().unwrap();

        root.backpropagate(actions, score);
    }

    pub fn print_tree(&self) {
        if let Some(root_node) = &self.root {
            let locked_root = root_node.lock().unwrap();
            println!("Root Node:");
            println!("{}", locked_root.to_string());
            locked_root.print_children(); // Call print_children on the root node
        } else {
            println!("Tree is empty");
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(root_node) = &self.root {
            let locked_root = root_node.lock().unwrap();
            result.push_str("Root Node:\n");
            result.push_str(&locked_root.to_string());
            result.push('\n');
            result.push_str(&locked_root.children_to_string(0));
        } else {
            result.push_str("Tree is empty\n");
        }

        result
    }
}
