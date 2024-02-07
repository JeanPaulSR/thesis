use crate::simulation::GameState;
use crate::entities::agent::Target;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::rc::Rc;

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

    // Getters
    pub fn get_action(&self) -> &NpcAction {
        &self.action
    }

    pub fn get_target(&self) -> &Target {
        &self.target
    }

    pub fn get_target_id(&self) -> u32 {
        self.target_id
    }

    pub fn get_tile_target(&self) -> &Option<(u32, u32)> {
        &self.tile_target
    }

    // Setters
    pub fn set_action(&mut self, action: NpcAction) {
        self.action = action;
    }

    pub fn set_target(&mut self, target: Target) {
        self.target = target;
    }

    pub fn set_target_id(&mut self, target_id: u32) {
        self.target_id = target_id;
    }

    pub fn set_tile_target(&mut self, tile_target: Option<(u32, u32)>) {
        self.tile_target = tile_target;
    }
}

pub type MCTSNodeRef = Rc<RefCell<MCTSNode>>;

#[derive(Clone)]
pub struct MCTSNode {
    action: ActionTaken,
    agent_id: u32,
    state_info: GameState,
    visits: usize,
    total_reward: u32,
    parent: Option<MCTSNodeRef>,
    children: Vec<MCTSNodeRef>,
}

impl MCTSNode {
     fn new(action: ActionTaken, agent_id: u32, state_info: GameState) -> MCTSNodeRef {
        Rc::new(RefCell::new(MCTSNode {
            action,
            agent_id,
            state_info,
            visits: 0,
            total_reward: 0,
            parent: None,
            children: Vec::new(),
        }))
    }
    
    // Getters
    pub fn get_agent_id(&self) -> u32 {
        self.agent_id
    }

    pub fn get_action(&self) -> &ActionTaken {
        &self.action
    }

    pub fn get_state_info(&self) -> &GameState {
        &self.state_info
    }

    pub fn get_visits(&self) -> usize {
        self.visits
    }

    pub fn get_total_reward(&self) -> u32 {
        self.total_reward
    }

    pub fn get_children(&self) -> Vec<MCTSNode> {
        self.children.iter().map(|child| child.borrow().clone()).collect()
    }

    // Setters
    pub fn set_agent_id(&mut self, agent_id: u32) {
        self.agent_id = agent_id;
    }

    pub fn set_action(&mut self, action: ActionTaken) {
        self.action = action;
    }

    pub fn set_state_info(&mut self, state_info: GameState) {
        self.state_info = state_info;
    }

    pub fn set_visits(&mut self, visits: usize) {
        self.visits = visits;
    }

    pub fn set_total_reward(&mut self, total_reward: u32) {
        self.total_reward = total_reward;
    }

    pub fn set_children(&mut self, children: Vec<MCTSNode>) {
        self.children = children.iter().map(|child| Rc::new(RefCell::new(child.clone()))).collect();
    }

    // Add a child to the current node
    pub fn add_child(&mut self, child_action: ActionTaken, child_state_info: GameState, agent_id: u32) {
        // Create a new child MCTSNode
        let parent_rc = Rc::new(RefCell::new(self.clone()));

        let child_node = Rc::new(RefCell::new(MCTSNode {
            action: child_action,
            agent_id: agent_id,
            state_info: child_state_info,
            visits: 0,
            total_reward: 0,
            parent: Some(Rc::clone(&parent_rc)),
            children: Vec::new(),
        }));

        // Add the child to the current node's children
        self.children.push(Rc::clone(&child_node));
    }

    fn create_children(&mut self) {
    // Clone agents before iterating to avoid borrowing conflicts
    let agents = self.get_state_info().get_agents().clone();

    // Attack action first
    for (agent_id, agent_data) in agents {
        self.add_child(
            ActionTaken::new_constructor(NpcAction::Attack, Target::Agent, agent_id, None),
            GameState::new(),
            agent_id,
        );
    }
}

    fn simulate(&self) -> u32 {
        // Implement your simulation logic
        // For simplicity, we'll return a random reward between 0 and 100
        rand::random::<u32>() % 101
    }

    fn backpropagate(&mut self, reward: u32) {
        self.visits += 1;
        self.total_reward += reward;
        if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().backpropagate(reward);
        }
    }

    fn mcts_iteration(&mut self, exploration_param: f64) {
        // Selection phase
        let selected_node = self.select_child(exploration_param);

        // Expansion phase
        if let Some(node) = selected_node {
            node.borrow_mut().create_children();
        
            // Store the result of choose_mut in a variable before attempting to mutate it
            let mut rng = rand::thread_rng();
            if let Some(random_child) = node.borrow_mut().children.choose_mut(&mut rng) {
                let reward = random_child.borrow().simulate();
                random_child.borrow_mut().backpropagate(reward);
            }
        }
    }

    fn select_child(&self, exploration_param: f64) -> Option<MCTSNodeRef> {
        self.children
            .iter()
            .max_by(|a, b| {
                a.borrow().uct_value(exploration_param)
                    .partial_cmp(&b.borrow().uct_value(exploration_param))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|child| Rc::clone(child))
    }

    fn uct_value(&self, exploration_param: f64) -> f64 {
        if self.visits == 0 {
            f64::INFINITY
        } else {
            let exploitation_term = f64::from(self.total_reward) / self.visits as f64;
            let exploration_term = exploration_param * (f64::ln((self.parent_visits() + 1) as f64) / self.visits as f64).sqrt();
            exploitation_term + exploration_term
        }
    }

    fn parent_visits(&self) -> usize {
        self.parent.as_ref().map_or(0, |p| p.borrow().visits)
    }
}



pub struct MCTSTree {
    root_node: Option<MCTSNode>,
}

impl MCTSTree {
    // Constructor function
    pub fn new_node(root_node: MCTSNode) -> Self {
        MCTSTree { root_node: Some(root_node) } 
    }

    pub fn new() -> Self {
        MCTSTree {
            root_node: None,
        }
    }
}

//Possible actions
//Attack Agent, Monster
//Steal Agent, Village, Treasure
//Rest Village
//Talk Agent
//None

//#[allow(dead_code)]
//#[derive(Clone)]
//#[derive(Debug)]
//pub enum NpcAction {
//    Attack,
//    Steal,
//    Rest,
//    Talk,
//    None,
//}

pub fn generate_nodes(node: MCTSNode){
    //agent_id = node.get_agent_id();
    //gamestate = node.get_state_info();


    //Create a node for each action
        //If the action or target would result in in being too far away to find, then don't include it


}

fn check_distance(coord1: (u32, u32), coord2: (u32, u32), n: u32) -> bool {
    let distance_squared = (coord1.0 as i32 - coord2.0 as i32).pow(2) + (coord1.1 as i32 - coord2.1 as i32).pow(2);
    let max_distance_squared = n as i32 * n as i32;

    distance_squared <= max_distance_squared
}

//Simulation starts with a current game state
//Each agent chooses an action based on *PARAMETERS*
//Each action is recorded in the node, such that replicating the choices in the future is easy
//as they only have to follow the path down
//Selecting a node is based off of Agent genes, using the calculate_action_score function, which
//calculates how likely an action is to be taken
//


//pub fn calculate_uct_score(child: &MCTSNode, parent: &MCTSNode, c: f64) -> f64 {
//    let total_reward = child.total_reward as f64;
//    let visits = child.visits as f64;
//    let parent_visits = parent.visits as f64;

//    total_reward / visits + c * (f64::ln(parent_visits) / visits).sqrt()
//}

//pub fn select(node: &MCTSNode, agent_genes: &Genes) -> usize {
//    let mut best_child_index = 0;
//    let mut best_score = f64::NEG_INFINITY;

//    let c = 2_f64.sqrt();
//    for (index, child) in node.children.iter().enumerate() {
//        let uct_score = calculate_uct_score(&child, &node, c);
//        let action_score = calculate_action_score(agent_genes, child.action.clone().unwrap_or(NpcAction::None));
//        let combined_score = uct_score * action_score as f64;

//        if combined_score > best_score {
//            best_score = combined_score;
//            best_child_index = index;
//        }
//    }

//    best_child_index
//}

//pub fn mcts_search(initial_state: GameState, iterations: usize, agent_genes: Genes) -> Option<NpcAction> {
//    let root = MCTSNode::new(initial_state, None);
//    let mut best_child: Option<&MCTSNode> = None; // Initialize as None

//    for _ in 0..iterations {
//        let best_child_index = select(&root, &agent_genes);
//        best_child = Some(&root.children[best_child_index]); // Update best_child

//        // Expand, simulate, and backpropagate as needed
//        // This is where you would implement the rest of the MCTS algorithm
//    }

//    // After the iterations, choose the best action based on the selected child node
//    match best_child {
//        Some(child) => child.action.clone(),
//        None => None, // No actions found
//    }
//}