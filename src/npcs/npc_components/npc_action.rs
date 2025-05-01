use crate::gameworld::position::Position;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorkType {
    Farming(Position),
    Mining(Position),
    Merchant,
}

impl ToString for WorkType {
    fn to_string(&self) -> String {
        match self {
            WorkType::Farming(position) => "Farming".to_string() + " at " + &position.to_string(),
            WorkType::Mining(position) => "Mining".to_string() + " at " + &position.to_string(),
            WorkType::Merchant => "Merchant".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NpcAction {
    AttackAgent,
    AttackMonster,
    Steal,
    TreasureHunt,
    Rest,
    Talk,
    Work(WorkType),
    None,
}

impl NpcAction {
    /// Returns an iterator over all variants of the enum.
    pub fn iter() -> impl Iterator<Item = NpcAction> {
        static ACTIONS: [NpcAction; 7] = [
            NpcAction::AttackAgent,
            NpcAction::AttackMonster,
            NpcAction::Steal,
            NpcAction::TreasureHunt,
            NpcAction::Rest,
            NpcAction::Talk,
            NpcAction::None,
        ];
        ACTIONS.iter().copied()
    }
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
            NpcAction::Work(work_type) => work_type.to_string() + &" Work ".to_string(),
        }
    }
}
