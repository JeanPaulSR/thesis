use crate::{simulation::GameState, entities::agent::Genes};

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
pub enum NpcAction {
    Attack,
    Steal,
    Rest,
    Talk,
    None,
    // Add more actions as needed
}

pub struct MCTSNode {
    state_info: GameState, // Subset of relevant game state data
    action: Option<NpcAction>,    // Action that led to this state
    visits: usize,            // Number of visits to this node
    total_reward: u32,        // Total reward from this node
    children: Vec<MCTSNode>,  // Child nodes for possible actions
}

impl MCTSNode {
    fn new(state_info: GameState, action: Option<NpcAction>) -> Self {
        MCTSNode {
            state_info,
            action,
            visits: 0,
            total_reward: 0,
            children: vec![],
        }
    }
}

pub struct MCTSTree{
    agent_id: u32,
    root_node: MCTSNode,
}

pub fn calculate_action_score(genes: &Genes, action: NpcAction) -> f32 {
    match action {
        NpcAction::Attack => genes.aggression,
        NpcAction::Steal => genes.greed,
        NpcAction::Rest => genes.self_preservation,
        NpcAction::Talk => genes.social,
        NpcAction::None => 0.0, // Default score for actions not influenced by genes
        // Add more actions as needed
    }
}

pub fn calculate_uct_score(child: &MCTSNode, parent: &MCTSNode, c: f64) -> f64 {
    let total_reward = child.total_reward as f64;
    let visits = child.visits as f64;
    let parent_visits = parent.visits as f64;

    total_reward / visits + c * (f64::ln(parent_visits) / visits).sqrt()
}

pub fn select(node: &MCTSNode, agent_genes: &Genes) -> usize {
    let mut best_child_index = 0;
    let mut best_score = f64::NEG_INFINITY;

    let c = 2_f64.sqrt();
    for (index, child) in node.children.iter().enumerate() {
        let uct_score = calculate_uct_score(&child, &node, c);
        let action_score = calculate_action_score(agent_genes, child.action.clone().unwrap_or(NpcAction::None));
        let combined_score = uct_score * action_score as f64;

        if combined_score > best_score {
            best_score = combined_score;
            best_child_index = index;
        }
    }

    best_child_index
}

pub fn mcts_search(initial_state: GameState, iterations: usize, agent_genes: Genes) -> Option<NpcAction> {
    let root = MCTSNode::new(initial_state, None);
    let mut best_child: Option<&MCTSNode> = None; // Initialize as None

    for _ in 0..iterations {
        let best_child_index = select(&root, &agent_genes);
        best_child = Some(&root.children[best_child_index]); // Update best_child

        // Expand, simulate, and backpropagate as needed
        // This is where you would implement the rest of the MCTS algorithm
    }

    // After the iterations, choose the best action based on the selected child node
    match best_child {
        Some(child) => child.action.clone(),
        None => None, // No actions found
    }
}