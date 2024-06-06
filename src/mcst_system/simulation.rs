//Runs a simulation with the agents actions, assumed best actions on all other agents, and random actions otherwise
//For loop over all agent actions
//Don't care about synchronicity for now
use bevy::prelude::*;

use crate::mcst::NpcAction;
use crate::{MCSTTotal, World,
            AgentMessages, MonsterMessages, TreasureMessages, mcst};
use crate::entities::agent;
use crate::entities::agent::Agent;
use crate::WorldSim;

pub fn run_simulation(
    _world_sim: ResMut<WorldSim>,
    _agent_copy: ResMut<Vec::<Agent>>,
    _tree: ResMut<mcst::SimulationTree>,
    _agent_messages: ResMut<AgentMessages>,
    _monster_messages: ResMut<MonsterMessages>,
    _treasure_messages: ResMut<TreasureMessages>,
    _mcst_total: ResMut<MCSTTotal>,
    _agent_query: Query<&mut Agent>, 
){

}
/*

Create a list of flags that mark if it has reached the end
Check if it is currently 
If not

*/
// pub fn simulation(
//     _world: &mut ResMut<World>,
//     _agent_messages: &mut ResMut<AgentMessages>,
//     _monster_messages: &mut ResMut<MonsterMessages>,
//     _treasure_messages: &mut ResMut<TreasureMessages>,
//     npc_actions: &mut Vec<(u32, Vec<NpcAction>)>,
//     query: &mut Query<&mut Agent>,
//     _commands: &mut Commands,
// ) {
//     println!("NEW LOOP");
//     //let npc_actions_clone = npc_actions.clone();
//     //Set to false if not finished yet
//     let mut finished_flags: Vec<bool> = npc_actions.iter().map(|_| false).collect();

//     while !is_finished(&mut finished_flags) {
//         //println!("NEW ITERATION");
//         for index in 0..npc_actions.len() {
//             let (action_id, actions) = &mut npc_actions[index];
//             for mut agent in query.iter_mut() {
//                 if action_id == &mut agent.get_id(){
//                     if agent.get_status() == agent::Status::Idle{
//                         if let Some(action) = actions.pop() {
//                             println!("Reached here");
//                             agent.set_action(action);
//                             if actions.is_empty(){
//                                 if let Some(flag) = finished_flags.get_mut(index) {
//                                     *flag = true;
//                                 } else {
//                                     panic!("Index out of bounds.");
//                                 }
//                             }
//                         } else {
//                             //agent.set_random_action();
//                         }
//                         //agent.calculate_target();
//                     }
//                 }

//             }
            
//         }
//         //perform action
//         //systems::agent_message_system(agent_messages, monster_messages, world, query, commands);
//         //messages
//     }
// }

fn is_finished(
    flags: &mut Vec<bool>
) -> bool {
    for &flag in flags.iter() {
        if !flag {
            return false;
        }
    }
    true
}
