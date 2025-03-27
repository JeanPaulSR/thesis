use crate::entities::agent::GeneType::{Aggression, Greed, SelfPreservation, Social};

use bevy::prelude::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::entities::agent::{Genes, Opinions};
use rand::Rng;

use super::mcst_tree::mcst_tree::MCTSTree;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NpcAction {
    AttackAgent,
    AttackMonster,
    Steal,
    TreasureHunt,
    Rest,
    Talk,
    None,
}

impl ToString for NpcAction {
    fn to_string(&self) -> String {
        match self {
            NpcAction::AttackAgent => "Attack Agent".to_string(),
            NpcAction::AttackMonster => "Attack Monster".to_string(),
            NpcAction::Steal => "Steal".to_string(),
            NpcAction::TreasureHunt => "Treasure Hunt".to_string(),
            NpcAction::Rest => "Rest".to_string(),
            NpcAction::Talk => "Talk".to_string(),
            NpcAction::None => "Root".to_string(),
        }
    }
}

#[derive(Default, Clone)]
pub struct ActionRating {
    actions: HashMap<NpcAction, f32>,
}

impl ActionRating {
    pub fn new() -> Self {
        let mut actions = HashMap::new();
        actions.insert(NpcAction::AttackAgent, 0.0);
        actions.insert(NpcAction::AttackMonster, 0.0);
        actions.insert(NpcAction::Steal, 0.0);
        actions.insert(NpcAction::TreasureHunt, 0.0);
        actions.insert(NpcAction::Rest, 0.0);
        actions.insert(NpcAction::Talk, 0.0);
        actions.insert(NpcAction::None, 0.0);

        ActionRating { actions }
    }
    
    pub fn generate_ratings(&mut self, genes: Genes) {
        self.actions.clear();
        //Attack Agent Action
        let agression = genes.return_type_score(Aggression);
        let self_preservation = genes.return_type_score(SelfPreservation);
        let attack_score = 0.5 + (((agression - 0.5) - (self_preservation - 0.5) )/2.0);

        self.actions.insert(
            NpcAction::AttackAgent,
            attack_score,
        );

        
        self.actions.insert(
            NpcAction::AttackMonster,
            attack_score,
        );
        
        let greed= genes.return_type_score(Greed);
        let wealth_score = 0.5 + (((greed - 0.5) - (self_preservation - 0.5) )/2.0);

        self.actions.insert(
            NpcAction::Steal,
            wealth_score,
        );
        self.actions.insert(
            NpcAction::TreasureHunt,
            wealth_score,
        );

        self.actions.insert(
            NpcAction::Rest,
            self_preservation,
        );

        self.actions
            .insert(NpcAction::Talk, genes.return_type_score(Social));
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

#[derive(Clone)]
pub struct ActionsTaken {
    action_rating: ActionRating,
    actions_vec: Vec<(NpcAction, u32)>,
}

impl ActionsTaken {
    pub fn new() -> Self {
        let action_rating = ActionRating::default();
        let mut actions_vec = Vec::new();

        actions_vec.push((NpcAction::AttackAgent, 0));
        actions_vec.push((NpcAction::AttackMonster, 0));
        actions_vec.push((NpcAction::Steal, 0));
        actions_vec.push((NpcAction::TreasureHunt, 0));
        actions_vec.push((NpcAction::Rest, 0));
        actions_vec.push((NpcAction::Talk, 0));
        actions_vec.push((NpcAction::None, 0));
        ActionsTaken {
            action_rating,
            actions_vec,
        }
    }

    pub fn new_with_rating(action_rating: ActionRating) -> Self {
        let mut actions_vec = Vec::new();

        actions_vec.push((NpcAction::AttackAgent, 0));
        actions_vec.push((NpcAction::AttackMonster, 0));
        actions_vec.push((NpcAction::Steal, 0));
        actions_vec.push((NpcAction::TreasureHunt, 0));
        actions_vec.push((NpcAction::Rest, 0));
        actions_vec.push((NpcAction::Talk, 0));
        actions_vec.push((NpcAction::None, 0));
        ActionsTaken {
            action_rating,
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

    pub fn get_action_rating(&self) -> ActionRating {
        self.action_rating.clone()
    }

    pub fn select_action(&self, opinions: Opinions ) -> Option<NpcAction> {
        let total_visits: f64 = self
            .actions_vec
            .iter()
            .map(|(_, visits)| *visits as f64)
            .sum();

        let mut best_action: Option<NpcAction> = None;
        let mut best_score: f64 = f64::NEG_INFINITY;

        for (action, visits) in &self.actions_vec {
            if *action == NpcAction::None {
                continue;
            }
            let selected_action_rating = self.action_rating.actions.get(action).unwrap_or(&0.0);

            let score = if *visits == 0 {
                f64::INFINITY
            } else {
                let exploitation = *selected_action_rating as f64;
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

#[derive(Resource)]
pub struct SimulationTree {
    forest: Option<Arc<Mutex<Vec<(u32, MCTSTree)>>>>,
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
            }
            None => {
                let tree_arc = Arc::new(Mutex::new(vec![new_tree]));
                self.forest = Some(tree_arc);
            }
        }
    }

    pub fn get_forest(&mut self) -> &mut Arc<Mutex<Vec<(u32, MCTSTree)>>> {
        let trees = &mut self.forest;
        match trees {
            Some(return_trees) => return_trees,
            //Should never ever enter her. System is otherwise fundamentally flawed
            None => {
                println!("Error generation MCST Tree, system unsalvegable");
                std::process::exit(0);
            }
        }
    }

    pub fn new_empty() -> Self {
        SimulationTree {
            forest: None,
            exploration_constant: 1.0,
        }
    }

    pub fn print_tree_id(&self, id: u32) {
        if let Some(forest) = &self.forest {
            let locked_forest = forest.lock().unwrap();
            for (tree_id, tree) in locked_forest.iter() {
                if *tree_id == id {
                    println!("Printing Tree with ID {}", id);
                    tree.print_tree();
                    return;
                }
            }
            println!("Tree with ID {} not found", id);
        } else {
            println!("Forest is empty");
        }
    }

    pub fn print_trees(&self) {
        if let Some(forest) = &self.forest {
            let locked_forest = forest.lock().unwrap();
            println!("Printing all trees in the forest:");
            for (tree_id, tree) in locked_forest.iter() {
                println!("Tree ID: {}", tree_id);
                tree.print_tree();
            }
        } else {
            println!("Forest is empty");
        }
    }
    

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(forest) = &self.forest {
            let locked_forest = forest.lock().unwrap();
            for (tree_id, tree) in locked_forest.iter() {
                result.push_str(&format!("Tree ID: {}\n", tree_id));
                result.push_str(&tree.to_string());
                result.push('\n'); 
            }
        } else {
            result.push_str("Forest is empty\n");
        }
        result
    }
}
