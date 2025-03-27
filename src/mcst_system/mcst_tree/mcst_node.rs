use crate::entities::agent::Opinions;
use crate::mcst_system::mcst::{ActionRating, ActionsTaken, NpcAction};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MCTSNode {
    action: Option<NpcAction>,
    action_score: ActionsTaken,
    leader: Option<u32>,
    //leader - this number is only filled when backtracking
    depth: u16,
    height: u16,
    original_height: u16,
    visits: usize,
    total_reward: u32,
    reward_visits: u32,
    average_reward: u32,
    is_root: bool,
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
            leader: None,
            depth: 0,
            height: 0,
            original_height: 0,
            visits: 0,
            total_reward: 0,
            reward_visits: 0,
            average_reward: 0,
            is_root: true,
            children: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_depth(&self) -> u16 {
        self.depth
    }

    pub fn is_valid_exploration_node(&self) -> bool {
        self.is_leaf() || self.get_depth() == 255
    }

    pub fn select(&mut self, opinions: Opinions) -> VecDeque<NpcAction> {
        let mut node_actions;
        self.visits = self.visits + 1;
        if self.action == None {
            node_actions = VecDeque::new();
        } else {
            node_actions = VecDeque::from(vec![self.action.unwrap()]);
        }
        if self.get_depth() == 255 {
            return node_actions;
        }
        let selected_action: NpcAction = self.action_score.select_action(opinions.clone()).unwrap();
        self.action_score.perform_action(selected_action);
        if let Some(child) = self.find_child(selected_action) {
            node_actions.append(&mut child.lock().unwrap().select(opinions.clone()));
        } else {
            self.expand(selected_action);
            node_actions.push_back(selected_action);
        }
        node_actions
    }

    pub fn choose_action(&self) -> NpcAction { 
        let mut chosen_action = NpcAction::None;
        let mut best_score = 0;
    
        // Iterate through the children, locking each one to access its data
        for child in &self.children {
            let child = child.lock().unwrap(); // Lock the child to access its data
            
            // Get the child's average reward
            let child_score = child.get_average_reward();
    
            // Check if the child has an action
            if let Some(child_action) = child.get_action() {
                // Get the action rating score for the child's action
                let action_rating = self.action_score.get_action_rating();
                let action_rating_actions = action_rating.get_actions();
                
                // Safely get the key-value pair for the child's action, or handle if it doesn't exist
                if let Some(action_score) = action_rating_actions.get(&child_action) {
                    let action_score_u32 = *action_score as u32;
                    let calculated_score = action_score_u32 * child_score;
                    
                    // If this score is better than the current best score, update it
                    if best_score < calculated_score {
                        best_score = calculated_score;
                        chosen_action = child_action;
                    }
                }
            }
        }
    
        // Return the chosen action with the best score
        //println!("{}", chosen_action.to_string());
        chosen_action
    }

    fn expand(&mut self, action: NpcAction) {
        let new_child = Arc::new(Mutex::new(MCTSNode {
            action: Some(action),
            action_score: ActionsTaken::new_with_rating(self.action_score.get_action_rating()),
            leader: None,
            depth: self.depth + 1,
            height: 0,
            original_height: 0,
            visits: 0,
            total_reward: 0,
            reward_visits: 0,
            average_reward: 0,
            is_root: false,
            children: Vec::new(),
        }));
        self.children.push(new_child);
    }

    pub fn find_child(&self, action: NpcAction) -> Option<&Arc<Mutex<MCTSNode>>> {
        for child in &self.children {
            let child_node = &child.lock().unwrap();

            if let Some(child_action) = &child_node.action {
                if *child_action == action {
                    if self.leader == None{
                        return Some(&child);
                    }
                }
            }
        }

        None
    }

    pub fn find_child_with_leader(&self, action: NpcAction, leader_id: u32) -> Option<&Arc<Mutex<MCTSNode>>> {
        for child in &self.children {
            let child_node = &child.lock().unwrap();

            if let Some(child_action) = &child_node.action {
                if *child_action == action {
                    if self.leader == Some(leader_id){
                        return Some(&child);
                    }
                }
            }
        }

        None
    }

    pub fn backpropagate(&mut self, mut actions: VecDeque<NpcAction>, score: u32) -> u32 {
        if actions.is_empty() {
            self.current_reward(score);
            self.average_reward
        } else {
            let current_action = actions.pop_front().unwrap();
            let reward;

            {
                let mut current_child = self
                    .find_child(current_action)
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap();
                reward = current_child.backpropagate(actions, score);
            }

            self.current_reward(reward);
            self.average_reward
        }
    }

    fn current_reward(&mut self, reward: u32) {
        self.total_reward = self.total_reward + reward;
        self.reward_visits = self.reward_visits + 1;
        self.average_reward = self.total_reward / self.reward_visits;
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_action(&self) -> Option<NpcAction> {
        self.action
    }

    pub fn get_average_reward(&self) -> u32 {
        self.average_reward
    }

    pub fn calculate_height(&mut self) {
        self.height = 0;
        for child in &self.children {
            let mut child_node = child.lock().unwrap();
            child_node.calculate_height();
            self.height = self.height.max(child_node.get_height() + 1);
        }
    }

    pub fn get_children(&mut self) -> &mut Vec<Arc<Mutex<MCTSNode>>> {
        &mut self.children
    }

    pub fn print_children(&self) {
        println!("Current Node: {}", self.to_string());
        println!("Children {}: ", self.children.len());

        for child in &self.children {
            let child_node = child.lock().unwrap();
            println!("{}", child_node.to_string());
            child_node.print_children();
        }
    }

    
    pub fn children_to_string(&self, mut level_depth: i32) -> String {
        let mut level = format!("");
        for _ in 0..level_depth{
            level.push_str(&format!("-"));
        }
        let mut result = format!("{} Current Node: {}\n", level, self.to_string());
        result.push_str(&format!("{} Children count: {}\n", level, self.children.len()));
        level_depth = level_depth + 1;
        for child in &self.children {
            let child_node = child.lock().unwrap();
            result.push_str(&child_node.to_string());
            result.push('\n');
            result.push_str(&child_node.children_to_string(level_depth));
        }

        result
    }
}
