use crate::entities::agent::GeneType::{Aggression, SelfPreservation, Greed, Social};



use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::entities::agent::Genes;
use rand::Rng;


use super::mcst_tree::mcst_tree::MCTSTree;

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
            NpcAction::None => "Root".to_string(),
        }
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

    //MODIFY VALUES WITH GENES
    //pub enum GeneType {
    //    Greed,
    //    Aggression,
    //    Social,
    //    SelfPreservation,
    //    Vision,
    //}

    //#[derive(Clone, Debug)]
    //pub struct Genes {
    //    pub gene_scores: HashMap<GeneType, f32>,
    //}
    pub fn generate_ratings(&mut self, genes: Genes) {
        self.actions.clear();
        self.actions.insert(NpcAction::Attack, 1.0 * genes.return_type_score(Aggression) * (1.0 - genes.return_type_score(SelfPreservation)));
        self.actions.insert(NpcAction::Steal, 1.0 * genes.return_type_score(Greed) * (1.0 - genes.return_type_score(SelfPreservation)));
        self.actions.insert(NpcAction::Rest, 1.0 * genes.return_type_score(SelfPreservation));
        self.actions.insert(NpcAction::Talk, 1.0 * genes.return_type_score(Social));
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
            
        actions_vec.push((NpcAction::Attack, 0));
        actions_vec.push((NpcAction::Steal, 0));
        actions_vec.push((NpcAction::Rest, 0));
        actions_vec.push((NpcAction::Talk, 0));
        actions_vec.push((NpcAction::None, 0));
        ActionsTaken { action_rating, actions_vec }
    }

    pub fn new_with_rating(
        action_rating: ActionRating,
    ) -> Self {
        let mut actions_vec = Vec::new();
            
        actions_vec.push((NpcAction::Attack, 0));
        actions_vec.push((NpcAction::Steal, 0));
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

    pub fn get_action_rating(&self) -> ActionRating{
        self.action_rating.clone()
    }

    pub fn select_action(&self) -> Option<NpcAction> {
        let total_visits: f64 = self.actions_vec.iter().map(|(_, visits)| *visits as f64).sum();

        let mut best_action: Option<NpcAction> = None;
        let mut best_score: f64 = f64::NEG_INFINITY;

        for (action, visits) in &self.actions_vec {
            if *action == NpcAction::None{
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

pub struct SimulationTree {
    forest: Option<Arc<Mutex<Vec<( u32,MCTSTree)>>>>,
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
            for (tree_id, _) in locked_forest.iter() {
                println!("Tree ID: {}", tree_id);
            }
        } else {
            println!("Forest is empty");
        }
    }
}
