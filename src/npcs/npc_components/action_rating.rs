use std::collections::HashMap;

use rand::Rng;

use super::{gene_type::GeneType, genes::Genes, npc_action::NpcAction};


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
        let agression = genes.return_type_score(GeneType::Aggression);
        let self_preservation = genes.return_type_score(GeneType::SelfPreservation);
        let attack_score = 0.5 + (((agression - 0.5) - (self_preservation - 0.5) )/2.0);

        self.actions.insert(
            NpcAction::AttackAgent,
            attack_score,
        );

        
        self.actions.insert(
            NpcAction::AttackMonster,
            attack_score,
        );
        
        let greed= genes.return_type_score(GeneType::Greed);
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
            .insert(NpcAction::Talk, genes.return_type_score(GeneType::Social));
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