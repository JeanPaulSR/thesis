use bevy::prelude::*;

#[derive(Clone)]
pub struct Treasure {
    pub id: u32,
    pub transform: Transform,
    
}

impl Treasure {
    pub fn get_position(&self) -> (f32, f32) {
        (self.transform.translation.x / 32.0, self.transform.translation.y / 32.0)
    }
}