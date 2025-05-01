#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Idle,
    Finished,
    Working,
    Moving,
    Dead,
    Following,
    Retaliating,
    Fleeing,
    Recovering,
    Attacking,
    Talking,
    RequiresInstruction,
}
