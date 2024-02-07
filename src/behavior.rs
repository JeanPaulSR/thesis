//use crate::{simulation::GameState, mcst::NpcAction};

//Possible actions
//Attack Agent, Monster
//Steal Agent, Village, Treasure
//Rest Village
//Talk Agent
//None

//pub fn calculate_action_score(genes: &Genes, action: NpcAction) -> f32 {
//    match action {
//        NpcAction::Attack => genes.aggression,
//        NpcAction::Steal => genes.greed,
//        NpcAction::Rest => genes.self_preservation,
//        NpcAction::Talk => genes.social,
//        NpcAction::None => 0.0, 
//    }
//}



//pub struct Genes {
//    pub greed: f32,
//    pub aggression: f32,
//    pub social: f32,
//    pub self_preservation: f32,
//    pub vision: f32,
//}

//pub enum NpcAction {
//    Attack,
//    Steal,
//    Rest,
//    Talk,
//    None,
//}
// struct MCTSNode {
//     state_info: GameState, // Subset of relevant game state data
//     action: Option<NpcAction>,    // Action that led to this state
//     visits: usize,            // Number of visits to this node
//     total_reward: u32,        // Total reward from this node
//     children: Vec<MCTSNode>,  // Child nodes for possible actions
// }

// impl MCTSNode {
//     fn new(state_info: GameState, action: Option<NpcAction>) -> Self {
//         MCTSNode {
//             state_info,
//             action,
//             visits: 0,
//             total_reward: 0,
//             children: vec![],
//         }
//     }
// }

// fn select(node: &MCTSNode) -> &MCTSNode {
//     // Implement your selection logic here (e.g., UCT)
//     // Select the child node with the highest UCT score
//     // based on the current node's statistics and exploration criteria.
//     unimplemented!()
// }

// fn expand(node: &mut MCTSNode) {
//     // Implement the expansion logic
//     // Create child nodes for possible actions from the current state.
//     // Add the child nodes to the `node.children` vector.
//     unimplemented!()
// }

// fn simulate(node: &MCTSNode) -> u32 {
//     // Implement the game simulation logic.
//     // Simulate a game from the current state and return a reward/score.
//     unimplemented!()
// }

// fn backpropagate(node: &mut MCTSNode, reward: u32) {
//     // Implement backpropagation logic.
//     // Update the visit count and total reward of the node and its ancestors.
//     unimplemented!()
// }

// fn mcts_search(initial_state: GameState, iterations: usize) -> Option<NpcAction> {
//     let root = MCTSNode::new(initial_state, None);

//     for _ in 0..iterations {
//         let node = select(&root);
//         expand(node);
//         let reward = simulate(node);
//         backpropagate(node, reward);
//     }

//     // Choose the best action based on statistics of child nodes.
//     let best_child = root
//         .children
//         .iter()
//         .max_by(|a, b| {
//             a.visits
//                 .cmp(&b.visits)
//                 .then(a.total_reward.cmp(&b.total_reward))
//         });

//     match best_child {
//         Some(child) => child.action.clone(),
//         None => None, // No actions found
//     }
// }