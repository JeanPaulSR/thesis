//For loop that takes all the agents

//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity
use bevy::prelude::*;
use bevy::app::Events;
use bevy::app::AppExit;
use crate::mcst::MCTSNode;
use crate::World;
use crate::AgentMessages;
use crate::MonsterMessages;
use crate::TreasureMessages;
use crate::mcst;
use crate::entities;


use crate::entities::agent::Agent;



pub fn run_simulation(
    mut world: ResMut<World>,
    mut tree: ResMut<mcst::MCTSTree>,
    mut agent_messages: ResMut<AgentMessages>,
    mut monster_messages: ResMut<MonsterMessages>,
    mut treasure_messages: ResMut<TreasureMessages>,
    mut iteration_counter: Local<i32>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut query: Query<&mut Agent>, 
){
    if tree.is_empty() {
        tree.insert_node(MCTSNode::new(None));
        let mut gene_list = mcst::GeneList::new();
        for mut agent in query.iter_mut() {
            gene_list.add_gene(agent.get_id(), agent.get_genes().clone());
        }
        tree.set_genes(gene_list);
        tree.initialize_node();
        println!("MCST Setup Complete");
    }
    
    //SELECT CHILD
    // Query for all mutable Agent components
    for mut agent in query.iter_mut() {
        if agent.get_id() == 1 {
            let genes = agent.get_genes();
            let (x, y) = agent.get_position();
            println!("Position for agent 1: ({},{})", x, y);
            // Found the desired agent by ID
            agent.set_agent_target_id(2);
            agent.set_target(entities::agent::Target::Agent);
            agent.set_action(mcst::NpcAction::Attack);
            //agent.perform_action(world, commands, agent_messages);
        }
    }



    // End Condition
    *iteration_counter += 1;
    // Check if it's time to end the simulation
    if *iteration_counter >= 10 {
        // Trigger AppExit event to end the program
        app_exit_events.send(AppExit);
    }
}
