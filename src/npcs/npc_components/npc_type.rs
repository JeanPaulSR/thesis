use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Component)]
pub enum NPCType {
    Agent,
    Monster,
    Treasure,
    Player,
}

impl NPCType {
    pub fn to_string(&self) -> &'static str {
        match self {
            NPCType::Agent => "agent",
            NPCType::Monster => "monster",
            NPCType::Treasure => "treasure",
            NPCType::Player => "player",
        }
    }
}