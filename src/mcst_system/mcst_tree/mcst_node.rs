use crate::mcst_system::mcst::{ActionRating, ActionsTaken, NpcAction};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MCTSNode {
    action: Option<NpcAction>,
    action_score: ActionsTaken,
    depth: u8,
    height: u16,
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
            depth: 0,
            height: 0,
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

    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    pub fn is_valid_exploration_node(&self) -> bool {
        self.is_leaf() || self.get_depth() == 255
    }

    pub fn select(&mut self) -> VecDeque<NpcAction> {
        let mut node_actions;
        self.visits = self.visits + 1;
        if self.is_root() {
            node_actions = VecDeque::new();
        } else {
            node_actions = VecDeque::from(vec![self.action.unwrap()]);
        }
        if self.get_depth() == 255 {
            return node_actions;
        }
        let selected_action: NpcAction = self.action_score.select_action().unwrap();
        self.action_score.perform_action(selected_action);
        if let Some(child) = self.find_child(selected_action) {
            node_actions.append(&mut child.lock().unwrap().select());
        } else {
            self.expand(selected_action);
            node_actions.push_back(selected_action);
        }
        node_actions
    }

    fn expand(&mut self, action: NpcAction) {
        let new_child = Arc::new(Mutex::new(MCTSNode {
            action: Some(action),
            action_score: ActionsTaken::new_with_rating(self.action_score.get_action_rating()),
            depth: self.depth + 1,
            height: 0,
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
                    return Some(&child);
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

    pub fn calculate_height(&mut self) {
        self.height = 0;
        for child in &self.children {
            let mut child_node = child.lock().unwrap();
            child_node.calculate_height();
            self.height = self.height.max(child_node.get_height() + 1);
        }
    }

    pub fn get_children(&self) -> &[Arc<Mutex<MCTSNode>>] {
        &self.children
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
}
