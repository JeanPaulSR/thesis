

use crate::mcst_system::mcst::{ActionRating, NpcAction};


use std::sync::{Arc, Mutex};
use std::collections::{VecDeque};
use crate::entities::agent::Agent;
use crate::entities::agent::Genes;



use super::mcst_node::MCTSNode;

pub struct MCTSTree {
    root: Option<Arc<Mutex<MCTSNode>>>,
    current_node: Option<Arc<Mutex<MCTSNode>>>,
    genes: Option<Genes>,
    action_rating: Option<ActionRating>,
    exploration_constant: f64,
}

impl MCTSTree {
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    pub fn insert_root(&mut self, node: MCTSNode) {
        let node_arc = Arc::new(Mutex::new(node));
        self.root = Some(node_arc);
    }
    
    pub fn set_genes(&mut self, genes: Genes){
        self.genes = Some(genes);
    }


    pub fn new_empty() -> Self {
        MCTSTree {
            root: None,
            current_node: None,
            genes: None,
            action_rating: None,
            exploration_constant: 1.0,
        }
    }

    pub fn initialize_tree(&mut self, agent: Agent){
        self.genes = Some(agent.get_genes().clone());
        let mut action_rating = ActionRating::new();
        action_rating.generate_ratings(agent.get_genes().clone());
        
        let node = MCTSNode::new(None, action_rating);
        self.insert_root(node);
    } 

     pub fn select_child(&mut self) -> &mut Arc<Mutex<MCTSNode>> {
        let returning_node = &mut self.root;
        
        
        match returning_node {
            Some(node) => node,
            None => !unreachable!(),
        }
    }

    pub fn selection_phase(&mut self) -> VecDeque<NpcAction> {
        let mut root = self.root.as_ref().unwrap().lock().unwrap();
        root.select()

    }

    pub fn expand(&mut self) -> Arc<Mutex<MCTSNode>> {
        unimplemented!()
    }

    //Try recursive instead
    pub fn backpropegate(&mut self, actions: VecDeque<NpcAction>, score: u32){
        if actions.is_empty() {
            panic!("Passed empty actions to backpropegate")
        }
        
        let mut root = self.root.as_ref().unwrap().lock().unwrap();
        
        root.backpropagate(actions, score);
    }
}