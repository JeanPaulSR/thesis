use crate::entities::agent::Target;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::entities::agent::Agent;
use crate::entities::agent::Genes;
use rand::Rng;
use crate::errors::MyError;

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
 #[derive(Eq, Hash, PartialEq)]
pub enum NpcAction {
    Attack,
    Steal,
    Rest,
    Talk,
    None,
}

impl ToString for NpcAction {
    fn to_string(&self) -> String {
        match self {
            NpcAction::Attack => "Attack".to_string(),
            NpcAction::Steal => "Steal".to_string(),
            NpcAction::Rest => "Rest".to_string(),
            NpcAction::Talk => "Talk".to_string(),
            NpcAction::None => "None".to_string(),
        }
    }
}


#[derive(Clone)]
pub struct ActionsTaken {
    actions: ActionRating,
    actions_vec: Vec<(NpcAction, u32)>,
}

impl ActionsTaken {
    pub fn new() -> Self {
        let actions = ActionRating::default(); 
        let mut actions_vec = Vec::new();
            
        actions_vec.push((NpcAction::Attack, 0));
        actions_vec.push((NpcAction::Steal, 0));
        actions_vec.push((NpcAction::Rest, 0));
        actions_vec.push((NpcAction::Talk, 0));
        actions_vec.push((NpcAction::None, 0));
        ActionsTaken { actions, actions_vec }
    }

    pub fn new_with_rating(
        actions: ActionRating,
    ) -> Self {
        let mut actions_vec = Vec::new();
            
        actions_vec.push((NpcAction::Attack, 0));
        actions_vec.push((NpcAction::Steal, 0));
        actions_vec.push((NpcAction::Rest, 0));
        actions_vec.push((NpcAction::Talk, 0));
        actions_vec.push((NpcAction::None, 0));
        ActionsTaken {
            actions,
            actions_vec,
        }
    }

    pub fn perform_action(&mut self, action: NpcAction) {
        if let Some((_, count)) = self.actions_vec.iter_mut().find(|(a, _)| *a == action) {
            *count += 1;
        } else {
            println!("Action {:?} not found in actions_vec", action);
        }
    }

    pub fn get_action_rating(&self) -> ActionRating{
        self.actions.clone()
    }

    pub fn select_action(&self) -> Option<NpcAction> {
        let total_visits: f64 = self.actions_vec.iter().map(|(_, visits)| *visits as f64).sum();

        let mut best_action: Option<NpcAction> = None;
        let mut best_score: f64 = f64::NEG_INFINITY;

        for (action, visits) in &self.actions_vec {
            let action_rating = self.actions.actions.get(action).unwrap_or(&0.0);

            let score = if *visits == 0 {
                f64::INFINITY
            } else {
                let exploitation = *action_rating as f64;
                let exploration = (total_visits.ln() / (*visits as f64)).sqrt();
                exploitation + exploration
            };

            if score > best_score {
                best_action = Some(*action);
                best_score = score;
            }
        }

        best_action
    }
}

#[derive(Default)]
#[derive(Clone)]
pub struct ActionRating{
    actions: HashMap<NpcAction, f32>,
}


impl ActionRating {
    pub fn new() -> Self {
        let mut actions = HashMap::new();
        actions.insert(NpcAction::Attack, 0.0);
        actions.insert(NpcAction::Steal, 0.0);
        actions.insert(NpcAction::Rest, 0.0);
        actions.insert(NpcAction::Talk, 0.0);
        actions.insert(NpcAction::None, 0.0);

        ActionRating { actions }
    }

    fn generate_actions(&mut self, _genes: Genes) {
        self.actions.insert(NpcAction::Attack, 0.0);
        self.actions.insert(NpcAction::Steal, 0.0);
        self.actions.insert(NpcAction::Rest, 0.0);
        self.actions.insert(NpcAction::Talk, 0.0);
        self.actions.insert(NpcAction::None, 0.0);
    }

    pub fn calculate_total(&self) -> f32 {
        self.actions.values().sum()
    }

    pub fn get_actions(&self) -> &HashMap<NpcAction, f32> {
    &self.actions
}

    pub fn select_action(&self) -> Option<NpcAction> {
        let total = self.calculate_total();
        if total == 0.0 {
            return None;
        }

        let mut rng = rand::thread_rng();
        let mut cumulative_sum = 0.0;
        let rand_num: f32 = rng.gen_range(0.0..total);

        for (action, value) in &self.actions {
            cumulative_sum += *value;
            if cumulative_sum >= rand_num {
                return Some(*action);
            }
        }

        None
    }
}

pub type MCTSNodeRef = Rc<RefCell<MCTSNode>>;

#[derive(Clone)]
pub struct MCTSNode {
    action: Option<NpcAction>,
    action_score: ActionsTaken,
    depth: u8,
    visits: usize,
    total_reward: u32,
    parent: Option<Arc<Mutex<MCTSNode>>>,
    children: Vec<Arc<Mutex<MCTSNode>>>,
}

impl ToString for MCTSNode {
    fn to_string(&self) -> String {
        let action_str = match &self.action {
            Some(action) => action.to_string(),
            None => "None".to_string(),
        };
        format!(
            "Action: {}, Depth: {}, Visits: {}, Total Reward: {}",
            action_str, self.depth, self.visits, self.total_reward
        )
    }
}

impl MCTSNode {
    pub fn new(action: Option<NpcAction>, action_rating: ActionRating) -> Self {
        MCTSNode {
            action,
            action_score: ActionsTaken::new_with_rating(action_rating),
            depth: 0,
            visits: 0,
            total_reward: 0,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    pub fn calculate_depth(&mut self){
        match &self.parent {
            Some(parent_node) => {
                let parent_node = parent_node.lock().unwrap();
                self.depth = parent_node.get_depth() + 1;
            }
            None => {
                self.depth = 0;
            }
        }
    }

    pub fn is_valid_exploration_node(&self) -> bool {
        self.is_leaf() || self.get_depth() == 255
    }
    
    pub fn select(&mut self) -> Vec<NpcAction> {
        let depth = self.get_depth();
        if depth == 255 {
            // If depth reaches 255, return the current NPCAction
            return vec![self.action.unwrap()];
        }

        if self.is_leaf() {
            // If the current node is a leaf, return the current NPCAction
            return vec![self.action.unwrap()];
        }

        // Select an action using some selection strategy
        let selected_action = self.action_score.select_action().unwrap();

        // Check if the selected action leads to an existing child
        if let Some(child) = self.find_child(selected_action) {
            // Recursively call select on the chosen child
            child.lock().unwrap().select()
        } else {
            // If the selected action doesn't lead to an existing child, expand the node
            self.expand(selected_action);
            vec![self.action.unwrap(), selected_action]
        }
    }

    fn expand(&mut self, action: NpcAction) {
        // Placeholder implementation, replace with your actual expansion logic
        // For example, create a new child node with the given action
        let new_child = Arc::new(Mutex::new(MCTSNode {
            action: Some(action),
            action_score: ActionsTaken::new(), // Assuming ActionsTaken has a new method
            depth: self.depth + 1,
            visits: 0,
            total_reward: 0,
            parent: Some(Arc::new(Mutex::new(self.clone()))),
            children: Vec::new(),
        }));
        self.children.push(new_child);
    }

    pub fn find_child(&self, action: NpcAction) -> Option<&Arc<Mutex<MCTSNode>>> {
        // Iterate through the children of the current node
        for child in &self.children {
            let child_node = &child.lock().unwrap(); // Access the child node
            
            // Check if the child node's action matches the specified action
            if let Some(child_action) = &child_node.action {
                if *child_action == action {
                    return Some(&child);
                }
            }
        }
        
        None
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

pub struct GeneList {
    gene_list: Vec<(u32, Genes)>,
}

impl GeneList {
    pub fn new() -> Self {
        GeneList {
            gene_list: Vec::new(),
        }
    }

    pub fn add_gene(&mut self, agent_id: u32, gene: Genes) {
        self.gene_list.push((agent_id, gene));
    }

    pub fn get_genes(&self, agent_id: &u32) -> Vec<&Genes> {
        self.gene_list
            .iter()
            .filter_map(|(id, Genes)| if id == agent_id { Some(Genes) } else { None })
            .collect()
    }


    pub fn set_genes(&mut self, agent_id: u32, genes: Vec<Genes>) {
        // Remove previous genes associated with the agent_id
        self.gene_list.retain(|(id, _)| *id != agent_id);

        // Add new genes
        for gene in genes {
            self.add_gene(agent_id, gene);
        }
    }
}

pub struct MCTSTree {
    root: Option<Arc<Mutex<MCTSNode>>>,
    current_node: Option<Arc<Mutex<MCTSNode>>>,
    genes: Option<GeneList>,
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
    
    pub fn set_genes(&mut self, gene_set: GeneList){
        self.genes = Some(gene_set);
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
        let mut gene_list = GeneList::new();
        gene_list.add_gene(agent.get_id(), agent.get_genes().clone());
        let mut action_rating = ActionRating::new();
        action_rating.generate_actions(agent.get_genes().clone());
        
        let node = MCTSNode::new(None, action_rating);
        println!("Node {} is: {}", agent.get_id(), node.to_string());
        self.insert_root(node);
        println!("INSERTING AGENT {}", agent.get_id());
    } 

     pub fn select_child(&mut self) -> &mut Arc<Mutex<MCTSNode>> { //-> &Arc<Mutex<MCTSNode>> {
        let returning_node = &mut self.root; // Start with the root node
        
        
        match returning_node {
            Some(node) => node,
            None => !unreachable!(),
        }
    }

    pub fn expand(&mut self) -> Arc<Mutex<MCTSNode>> {
        // Implement node expansion logic here
        unimplemented!()
    }
}

pub struct SimulationTree {
    forest: Option<Arc<Mutex<Vec<( u32, MCTSTree)>>>>,
    exploration_constant: f64,
}

impl SimulationTree {
    pub fn is_empty(&self) -> bool {
        self.forest.is_none()
    }
    
    pub fn insert_tree(&mut self, tree: MCTSTree, index: u32) {
        let new_tree = (index, tree);
    
        match &mut self.forest {
            Some(forest_arc) => {
                forest_arc.lock().unwrap().push(new_tree);
            },
            None => {
                let tree_arc = Arc::new(Mutex::new(vec![new_tree]));
                self.forest = Some(tree_arc);
            }
        }
    }

    pub fn get_forest(&mut self) -> &mut Arc<Mutex<Vec<( u32, MCTSTree)>>> {
        let trees = &mut self.forest;
        match trees {
            Some(return_trees) => return_trees,
            None => !unreachable!(),
        }
    }
    
    pub fn new_empty() -> Self {
        SimulationTree {
            forest: None,
            exploration_constant: 1.0,
        }
    }
}
