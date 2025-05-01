use std::sync::{Arc, Mutex};
use crate::mcst_system::mcst_tree::mcst_node::{Node, NodeType};
use crate::mcst_system::mcst::NpcAction;

// Helper function to create a sample tree for testing
fn create_sample_tree() -> Arc<Mutex<Node>> {
    let root = Node {
        node_type: NodeType::ActionNode {
            action: NpcAction::None,
            visits: 0,
            total_reward: 0,
            children: Vec::new(),
        },
        depth: 0,
        parent: None,
    };

    let root_arc = Arc::new(Mutex::new(root));

    // Add child nodes
    {
        let mut root_lock = root_arc.lock().unwrap();
        root_lock.expand(NpcAction::Move);
        root_lock.expand(NpcAction::Attack);
    }

    // Add grandchildren to the first child
    {
        let root_lock = root_arc.lock().unwrap();
        let first_child = root_lock
            .select_best_action()
            .expect("First child should exist");
        let mut first_child_lock = first_child.lock().unwrap();
        first_child_lock.expand(NpcAction::Defend);
        first_child_lock.expand(NpcAction::Gather);
    }

    root_arc
}

#[test]
fn test_tree_creation() {
    let root = create_sample_tree();

    // Check root node
    let root_lock = root.lock().unwrap();
    assert!(matches!(root_lock.node_type, NodeType::ActionNode { .. }));
    assert_eq!(root_lock.depth, 0);

    // Check children of root
    if let NodeType::ActionNode { children, .. } = &root_lock.node_type {
        assert_eq!(children.len(), 2);
    } else {
        panic!("Root node should be an ActionNode");
    }
}

#[test]
fn test_monte_carlo_calculation() {
    let root = create_sample_tree();

    // Simulate some visits and rewards
    {
        let root_lock = root.lock().unwrap();
        if let NodeType::ActionNode { children, .. } = &root_lock.node_type {
            let first_child = &children[0];
            let mut first_child_lock = first_child.lock().unwrap();
            if let NodeType::ActionNode {
                visits,
                total_reward,
                ..
            } = &mut first_child_lock.node_type
            {
                *visits = 10;
                *total_reward = 50;
            }
        }
    }

    // Check Monte Carlo calculation
    let root_lock = root.lock().unwrap();
    if let NodeType::ActionNode { children, .. } = &root_lock.node_type {
        let first_child = &children[0];
        let first_child_lock = first_child.lock().unwrap();
        assert_eq!(first_child_lock.calculate_monte_carlo(), 5); // 50 / 10 = 5
    }
}

#[test]
fn test_tree_traversal_and_print() {
    let root = create_sample_tree();

    fn traverse_and_print(node: Arc<Mutex<Node>>, depth: usize) {
        let node_lock = node.lock().unwrap();
        match &node_lock.node_type {
            NodeType::ActionNode { action, .. } => {
                println!("{}ActionNode: {:?}", "  ".repeat(depth), action);
            }
            NodeType::InformationNode { action_taken, .. } => {
                println!("{}InformationNode: {:?}", "  ".repeat(depth), action_taken);
            }
        }

        if let NodeType::ActionNode { children, .. } | NodeType::InformationNode { children, .. } =
            &node_lock.node_type
        {
            for child in children {
                traverse_and_print(child.clone(), depth + 1);
            }
        }
    }

    traverse_and_print(root, 0);
}