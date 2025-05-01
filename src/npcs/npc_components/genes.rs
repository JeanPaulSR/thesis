use std::{collections::HashMap, sync::{Arc, Mutex}};
use super::gene_type::GeneType;


#[derive(Clone, Debug)]
pub struct Genes {
    pub gene_scores: Arc<Mutex<HashMap<GeneType, f32>>>,
}
impl Genes {
    pub fn new(gene_scores: HashMap<GeneType, f32>) -> Self {
        Genes {
            gene_scores: Arc::new(Mutex::new(gene_scores)),
        }
    }

    pub fn return_type_score(&self, gene_type: GeneType) -> f32 {
        let gene_scores = self.gene_scores.lock().unwrap();
        match gene_scores.get(&gene_type) {
            Some(result) => *result,
            None => todo!(),
        }
    }

    pub fn generate() -> Self {
        use rand::distributions::{Distribution, Uniform};

        // Initialize a random number generator
        let mut rng = rand::thread_rng();

        // Define distribution ranges for agent attributes
        let greed_distribution = Uniform::new(0.5, 1.0);
        let aggression_distribution = Uniform::new(0.3, 0.8);
        let common_distribution = Uniform::new(0.0, 1.0);
        let vision_distribution = Uniform::new(3.0, 8.0);

        // Generate random values for each attribute
        let mut gene_scores = HashMap::new();
        gene_scores.insert(GeneType::Greed, greed_distribution.sample(&mut rng));
        gene_scores.insert(
            GeneType::Aggression,
            aggression_distribution.sample(&mut rng),
        );
        gene_scores.insert(GeneType::Social, common_distribution.sample(&mut rng));
        gene_scores.insert(
            GeneType::SelfPreservation,
            common_distribution.sample(&mut rng),
        );
        gene_scores.insert(GeneType::Vision, vision_distribution.sample(&mut rng));

        Genes {
            gene_scores: Arc::new(Mutex::new(gene_scores)),
        }
    }
}