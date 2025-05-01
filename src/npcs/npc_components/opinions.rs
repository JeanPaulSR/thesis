use std::{collections::HashMap, sync::{Arc, Mutex}};


#[derive(Clone, Debug)]
pub struct Opinions {
    pub opinion_scores: Arc<Mutex<HashMap<i32, f32>>>,
}
