use crate::gameworld::position::Position;
use crate::gameworld::tile_types::TileType;
use crate::gameworld::world::{self, GameWorld};
use crate::npcs::agent::{flight_or_fight, Agent};
use crate::npcs::monster::Monster;
use crate::npcs::npc_components::npc_action::{NpcAction, WorkType};
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::npc_components::npc_status::Status;
use crate::npcs::npc_components::npc_type::NPCType;
use crate::npcs::npc_components::target::Target;
use crate::npcs::treasure::Treasure;
use bevy::prelude::*;

use crate::system::pathfinding::pathfinding_calculation::{
    a_star_pathfinding, a_star_with_current_path, recalculate_path_around_multiple_positions,
};

pub fn handle_selected_action_system(
    mut param_set: ParamSet<(
        Query<(&mut Agent, &NPCBase)>, // Query for agents and their NPCBase
        Query<(&Position, &Agent, &NPCBase)>, // Query for all agents (reference only)
        Query<(&Position, &Monster, &NPCBase)>, // Query for all monsters
        Query<(&Position, &Treasure, &NPCBase)>, // Query for all treasures
    )>,
    world: Res<GameWorld>,
) {
    // Clone the second query (agents) for reference
    let agent_references: Vec<(Position, Agent, NPCBase)> = param_set
        .p1()
        .iter()
        .map(|(position, agent, npc_base)| (*position, agent.clone(), npc_base.clone()))
        .collect();

    // Clone the third query (monsters) for reference
    let monster_references: Vec<(Position, Monster, NPCBase)> = param_set
        .p2()
        .iter()
        .map(|(position, monster, npc_base)| (*position, monster.clone(), npc_base.clone()))
        .collect();

    // Clone the fourth query (treasures) for reference
    let treasure_references: Vec<(Position, Treasure, NPCBase)> = param_set
        .p3()
        .iter()
        .map(|(position, treasure, npc_base)| (*position, treasure.clone(), npc_base.clone()))
        .collect();

    // Iterate over all agents and their associated NPCBase
    for (mut agent, npc_base) in param_set.p0().iter_mut() {
        // Match on the agent's current status
        match agent.get_status() {
            Status::Idle => {
                handle_idle_actions(
                    &mut agent,
                    npc_base,
                    &world,
                    &agent_references,    // Pass the cloned agent references
                    &monster_references,  // Pass the cloned monster references
                    &treasure_references, // Pass the cloned treasure references
                );
            }
            Status::Moving => {
                handle_moving_actions(
                    &mut agent,
                    npc_base,
                    &world,
                    &agent_references,    // Pass the cloned agent references
                    &monster_references,  // Pass the cloned monster references
                    &treasure_references, // Pass the cloned treasure references
                );
            }
            Status::Attacking => {
                // Check for the target agent or monster
                let target_type = agent.get_target();

                match target_type {
                    Target::Agent => {
                        let target_id = agent.get_agent_target_id();
                        if let Some((target_position, target_agent, target_npc_base)) =
                            agent_references
                                .iter()
                                .find(|(_, a, _)| a.get_id() == target_id as i32)
                        {
                            if !is_next_to_target(npc_base.get_position(), *target_position, 1) {
                                let path = a_star_pathfinding(
                                    &world,
                                    npc_base.get_position(),
                                    *target_position,
                                );
                                agent.set_path(path);
                                agent.set_status(Status::Moving);
                            } else {
                                if target_npc_base.get_energy() <= 0
                                    || target_agent.get_status() == Status::Dead
                                {
                                    set_finish(&mut agent, npc_base, &world);
                                }
                            }
                        } else {
                            set_finish(&mut agent, npc_base, &world);
                        }
                    }
                    Target::Monster => {
                        let target_id = agent.get_monster_target_id();
                        if let Some((target_position, target_monster, target_npc_base)) =
                            monster_references
                                .iter()
                                .find(|(_, m, _)| m.get_id() == target_id as i32)
                        {
                            if !is_next_to_target(npc_base.get_position(), *target_position, 1) {
                                let path = a_star_pathfinding(
                                    &world,
                                    npc_base.get_position(),
                                    *target_position,
                                );
                                agent.set_path(path);
                                agent.set_status(Status::Moving);
                            } else {
                                if target_npc_base.get_energy() <= 0
                                    || target_monster.get_status() == Status::Dead
                                {
                                    set_finish(&mut agent, npc_base, &world);
                                }
                            }
                        } else {
                            set_finish(&mut agent, npc_base, &world);
                        }
                    }
                    _ => {
                        agent.set_status(Status::RequiresInstruction);
                    }
                }
            }
            Status::Finished => {
                if let Some(village_position) = agent.get_tile_target() {
                    handle_path_recalculation_and_monster_avoidance(
                        &mut agent,
                        &world,
                        &monster_references,
                        village_position,
                        npc_base.get_position(),
                    );
                } else {
                    agent.set_status(Status::RequiresInstruction);
                }
            }
            Status::Working => {
                match agent.get_action() {
                    NpcAction::Steal => {
                        let target_id = agent.get_agent_target_id();
                        if let Some((target_position, target_agent, target_npc_base)) =
                            &agent_references
                                .iter()
                                .find(|(_, a, _)| a.get_id() == target_id as i32)
                        {
                            if !is_next_to_target(npc_base.get_position(), *target_position, 1) {
                                let path = a_star_pathfinding(
                                    &world,
                                    npc_base.get_position(),
                                    *target_position,
                                );
                                agent.set_path(path);
                                agent.set_status(Status::Moving);
                            } else {
                                if target_npc_base.get_energy() <= 0
                                    || target_agent.get_status() == Status::Dead
                                {
                                    set_finish(&mut agent, npc_base, &world);
                                }
                            }
                        } else {
                            set_finish(&mut agent, npc_base, &world);
                        }
                    }
                    NpcAction::TreasureHunt | NpcAction::Work(_) => {
                        // Handle other working actions
                    }
                    _ => {
                        agent.set_status(Status::RequiresInstruction);
                    }
                }
            }
            Status::Dead => {}
            Status::Following => {
                let leader_id = agent.get_leader_id();
                if let Some((_, leader_agent, leader_npc_base)) = &agent_references
                    .iter()
                    .find(|(_, a, _)| a.get_id() == leader_id)
                {
                    if leader_agent.get_action() != agent.get_action()
                        || npc_base.get_energy() <= 0
                        || leader_agent.get_status() == Status::Dead
                    {
                        agent.set_status(Status::Idle);
                        handle_idle_actions(
                            &mut agent,
                            npc_base,
                            &world,
                            &agent_references,
                            &monster_references,
                            &treasure_references,
                        );
                    }
                } else {
                    agent.set_status(Status::Idle);
                    handle_idle_actions(
                        &mut agent,
                        npc_base,
                        &world,
                        &agent_references,
                        &monster_references,
                        &treasure_references,
                    );
                }
            }
            Status::Retaliating | Status::Fleeing => {
                let retaliation_target_id = agent.get_retaliation_target_id();
                let retaliation_target_type = agent.get_retaliation_target();

                let (retaliation_target, finished_retaliating) = match retaliation_target_type {
                    Target::Agent => {
                        if let Some((position, agent, npc_base)) = &agent_references
                            .iter()
                            .find(|(_, a, _)| a.get_id() == retaliation_target_id as i32)
                        {
                            let is_dead = agent.get_status() == Status::Dead;
                            (Some((position, npc_base)), is_dead)
                        } else {
                            (None, true)
                        }
                    }
                    Target::Monster => {
                        if let Some((position, monster, npc_base)) = &monster_references
                            .iter()
                            .find(|(_, m, _)| m.get_id() == retaliation_target_id as i32)
                        {
                            let is_dead = monster.get_status() == Status::Dead;
                            (Some((position, npc_base)), is_dead)
                        } else {
                            (None, true)
                        }
                    }
                    _ => (None, true),
                };

                let mut finished_retaliating = finished_retaliating;

                if let Some((target_position, target_npc_base)) = retaliation_target {
                    if is_next_to_target(npc_base.get_position(), *target_position, 1) {
                        if target_npc_base.get_energy() <= 0 {
                            finished_retaliating = true;
                        }
                    } else {
                        finished_retaliating = true;
                    }
                } else {
                    finished_retaliating = true;
                }

                if finished_retaliating {
                    agent.set_status(Status::Idle);
                    handle_idle_actions(
                        &mut agent,
                        npc_base,
                        &world,
                        &agent_references,
                        &monster_references,
                        &treasure_references,
                    );
                }
            }
            Status::Recovering => {
                if npc_base.get_energy() >= npc_base.get_max_energy() {
                    agent.set_status(Status::Idle);
                    handle_idle_actions(
                        &mut agent,
                        npc_base,
                        &world,
                        &agent_references,
                        &monster_references,
                        &treasure_references,
                    );
                }
            }
            Status::RequiresInstruction => {
                eprintln!(
                    "Agent {} is waiting for instructions. This should not happen!",
                    agent.get_id()
                );
            }
            Status::Talking => {}
        }
    }
}

fn set_finish(agent: &mut Agent, npc_base: &NPCBase, world: &GameWorld) {
    if let Some(target_village) =
        world.find_closest_tiletype(npc_base.get_position(), TileType::Village)
    {
        let path = a_star_pathfinding(world, npc_base.get_position(), target_village);
        agent.set_path(path);
        agent.set_status(Status::Finished);
        agent.set_tile_target(Some(target_village));
    } else {
        agent.set_status(Status::RequiresInstruction);
    }
}

fn handle_idle_actions(
    agent: &mut Agent,
    npc_base: &NPCBase,
    world: &GameWorld,
    agent_positions: &[(Position, Agent, NPCBase)],
    monster_positions: &[(Position, Monster, NPCBase)],
    treasure_positions: &[(Position, Treasure, NPCBase)],
) {
    // Use the NPCBase's position instead of calculating it manually
    let agent_position = npc_base.get_position();

    match agent.get_action() {
        NpcAction::AttackAgent => {
            if let Some(target_id) = agent.find_best_agent() {
                if let Some((target_position, _, _)) = agent_positions
                    .iter()
                    .find(|(_, a, _)| a.get_id() == target_id as i32)
                {
                    if is_next_to_target(agent_position, *target_position, 1) {
                        agent.set_status(Status::Attacking);
                        agent.set_agent_target_id(target_id);
                    } else {
                        let path = a_star_pathfinding(world, agent_position, *target_position);
                        agent.set_path(path);
                        agent.set_status(Status::Moving);
                        agent.set_agent_target_id(target_id);
                    }
                }
            }
        }
        NpcAction::AttackMonster => {
            if let Some((monster_position, monster, _)) =
                monster_positions.iter().min_by_key(|(pos, _, _)| {
                    let dx = (pos.x - agent_position.x).abs();
                    let dy = (pos.y - agent_position.y).abs();
                    dx + dy
                })
            {
                if is_next_to_target(agent_position, *monster_position, 1) {
                    agent.set_status(Status::Attacking);
                    agent.set_monster_target_id(monster.get_id());
                } else {
                    let path = a_star_pathfinding(world, agent_position, *monster_position);
                    agent.set_path(path);
                    agent.set_status(Status::Moving);
                    agent.set_monster_target_id(monster.get_id());
                }
            }
        }
        NpcAction::Steal => {
            if let Some(target_id) = agent.find_worst_agent() {
                if let Some((target_position, _, _)) = agent_positions
                    .iter()
                    .find(|(_, a, _)| a.get_id() == target_id as i32)
                {
                    if is_next_to_target(agent_position, *target_position, 1) {
                        agent.set_status(Status::Working);
                        agent.set_agent_target_id(target_id);
                    } else {
                        let path = a_star_pathfinding(world, agent_position, *target_position);
                        agent.set_path(path);
                        agent.set_status(Status::Moving);
                        agent.set_agent_target_id(target_id);
                    }
                }
            }
        }
        NpcAction::TreasureHunt => {
            if let Some((treasure_position, treasure, _)) =
                treasure_positions.iter().min_by_key(|(pos, _, _)| {
                    let dx = (pos.x - agent_position.x).abs();
                    let dy = (pos.y - agent_position.y).abs();
                    dx + dy
                })
            {
                if is_next_to_target(agent_position, *treasure_position, 1) {
                    agent.set_status(Status::Working);
                    agent.set_treasure_target_id(treasure.get_id());
                } else {
                    let path = a_star_pathfinding(world, agent_position, *treasure_position);
                    agent.set_path(path);
                    agent.set_status(Status::Moving);
                    agent.set_treasure_target_id(treasure.get_id());
                }
            }
        }
        NpcAction::Rest | NpcAction::Talk => {
            if let Some(village_position) =
                world.find_closest_tiletype(agent_position, TileType::Village)
            {
                if is_next_to_target(agent_position, village_position, 0) {
                    let action = agent.get_action();
                    agent.set_status(if action == NpcAction::Rest {
                        Status::Recovering
                    } else {
                        Status::Talking
                    });
                } else {
                    let path = a_star_pathfinding(world, agent_position, village_position);
                    agent.set_path(path);
                    agent.set_status(Status::Moving);
                }
            }
        }
        NpcAction::Work(work_type) => match work_type {
            WorkType::Farming(target_position) | WorkType::Mining(target_position) => {
                if is_next_to_target(agent_position, target_position, 0) {
                    agent.set_status(Status::Working);
                } else {
                    let path = a_star_pathfinding(world, agent_position, target_position);
                    agent.set_path(path);
                    agent.set_status(Status::Moving);
                }
            }
            WorkType::Merchant => {
                if let Some(village_position) =
                    world.find_closest_tiletype(agent_position, TileType::Village)
                {
                    if is_next_to_target(agent_position, village_position, 0) {
                        agent.set_status(Status::Working);
                    } else {
                        let path = a_star_pathfinding(world, agent_position, village_position);
                        agent.set_path(path);
                        agent.set_status(Status::Moving);
                    }
                }
            }
        },
        NpcAction::None => {
            eprintln!(
                "Error: Agent {} has no action set while idle!",
                agent.get_id()
            );
        }
    }
}

fn handle_moving_actions(
    agent: &mut Agent,
    npc_base: &NPCBase,
    world: &GameWorld,
    agent_query: &[(Position, Agent, NPCBase)],
    monster_query: &[(Position, Monster, NPCBase)],
    treasure_query: &[(Position, Treasure, NPCBase)],
) {
    // Use the NPCBase's position to determine the agent's current position
    let agent_position = npc_base.get_position();

    match agent.get_action() {
        NpcAction::AttackAgent => {
            let target_id = agent.get_agent_target_id();
            if let Some((target_position, _, _)) = agent_query
                .iter()
                .find(|(_, a, _)| a.get_id() == target_id as i32)
            {
                if is_next_to_target(agent_position, *target_position, 1) {
                    agent.set_status(Status::Attacking);
                    agent.set_path(Vec::new()); // Clear the path
                } else {
                    handle_path_recalculation_and_monster_avoidance(
                        agent,
                        world,
                        monster_query,
                        *target_position,
                        agent_position,
                    );
                }
            }
        }
        NpcAction::AttackMonster => {
            let target_id = agent.get_monster_target_id();
            if let Some((monster_position, _, _)) = monster_query
                .iter()
                .find(|(_, m, _)| m.get_id() == target_id as i32)
            {
                if is_next_to_target(agent_position, *monster_position, 1) {
                    agent.set_status(Status::Attacking);
                    agent.set_path(Vec::new()); // Clear the path
                } else {
                    handle_path_recalculation_and_monster_avoidance(
                        agent,
                        world,
                        monster_query,
                        *monster_position,
                        agent_position,
                    );
                }
            }
        }
        NpcAction::Steal => {
            let target_id = agent.get_agent_target_id();
            if let Some((target_position, _, _)) = agent_query
                .iter()
                .find(|(_, a, _)| a.get_id() == target_id as i32)
            {
                if is_next_to_target(agent_position, *target_position, 1) {
                    agent.set_status(Status::Attacking); // Treat stealing as attacking
                    agent.set_path(Vec::new());
                } else {
                    handle_path_recalculation_and_monster_avoidance(
                        agent,
                        world,
                        monster_query,
                        *target_position,
                        agent_position,
                    );
                }
            }
        }
        NpcAction::TreasureHunt => {
            let target_id = agent.get_treasure_target_id();
            if let Some((treasure_position, _, _)) = treasure_query
                .iter()
                .find(|(_, t, _)| t.get_id() == target_id as i32)
            {
                if is_next_to_target(agent_position, *treasure_position, 1) {
                    agent.set_status(Status::Working); // Treasure hunting is treated as working
                    agent.set_path(Vec::new());
                } else {
                    handle_path_recalculation_and_monster_avoidance(
                        agent,
                        world,
                        monster_query,
                        *treasure_position,
                        agent_position,
                    );
                }
            }
        }
        NpcAction::Rest | NpcAction::Talk => {
            if let Some(village_position) =
                world.find_closest_tiletype(agent_position, TileType::Village)
            {
                if is_next_to_target(agent_position, village_position, 0) {
                    let action = agent.get_action();
                    agent.set_status(if action == NpcAction::Rest {
                        Status::Recovering
                    } else {
                        Status::Talking
                    });
                } else {
                    handle_path_recalculation_and_monster_avoidance(
                        agent,
                        world,
                        monster_query,
                        village_position,
                        agent_position,
                    );
                }
            }
        }
        NpcAction::Work(work_type) => match work_type {
            WorkType::Farming(target_position) | WorkType::Mining(target_position) => {
                if is_next_to_target(agent_position, target_position, 0) {
                    agent.set_status(Status::Working);
                } else {
                    handle_path_recalculation_and_monster_avoidance(
                        agent,
                        world,
                        monster_query,
                        target_position,
                        agent_position,
                    );
                }
            }
            WorkType::Merchant => {
                if let Some(village_position) =
                    world.find_closest_tiletype(agent_position, TileType::Village)
                {
                    if is_next_to_target(agent_position, village_position, 0) {
                        agent.set_status(Status::Working);
                    } else {
                        handle_path_recalculation_and_monster_avoidance(
                            agent,
                            world,
                            monster_query,
                            village_position,
                            agent_position,
                        );
                    }
                }
            }
        },
        NpcAction::None => {
            eprintln!(
                "Error: Agent {} has no action set while moving!",
                agent.get_id()
            );
            agent.set_status(Status::Idle);
        }
    }
}

// Check if the target is in the avoidance area
fn handle_path_recalculation_and_monster_avoidance(
    agent: &mut Agent,
    world: &GameWorld,
    monster_query: &[(Position, Monster, NPCBase)],
    target_position: Position,
    agent_position: Position,
) {
    // Recalculate path if the last position is not the target
    if let Some(path) = agent.get_path() {
        let mut recalculated_path = path;
        if recalculated_path.last() != Some(&target_position) {
            recalculated_path = a_star_with_current_path(world, recalculated_path, target_position);
            agent.set_path(recalculated_path.clone()); // Set the recalculated path
        }

        // Collect all monsters within 4 tiles
        let avoid_positions: Vec<(Position, i32)> = monster_query
            .iter()
            .filter(|(monster_position, _, _)| {
                let dx = (monster_position.x - agent_position.x).abs();
                let dy = (monster_position.y - agent_position.y).abs();
                dx <= 4 && dy <= 4
            })
            .map(|(monster_position, _, _)| (*monster_position, 4)) // Use a radius of 4 for all monsters
            .collect();

        if !avoid_positions.is_empty() {
            // Check if any monster is in the direction the agent is going
            let next_positions: Vec<_> = recalculated_path.iter().take(3).cloned().collect();

            if avoid_positions.iter().any(|(monster_position, _)| {
                next_positions.iter().any(|pos| {
                    let dx = (monster_position.x - pos.x).abs();
                    let dy = (monster_position.y - pos.y).abs();
                    dx <= 3 && dy <= 3
                })
            }) {
                // Check the agent's flight_or_fight response
                if flight_or_fight(agent, NPCType::Monster, None) {
                    // Flight: Recalculate path around all monsters
                    let new_path = recalculate_path_around_multiple_positions(
                        world,
                        recalculated_path,
                        avoid_positions,
                        target_position,
                    );
                    agent.set_path(new_path); // Update the path
                }
            }
        }
    }
}

pub fn is_next_to_target(
    current_position: Position,
    target_position: Position,
    distance_in_tiles: i32,
) -> bool {
    let dx = (current_position.x - target_position.x).abs();
    let dy = (current_position.y - target_position.y).abs();
    dx <= distance_in_tiles && dy <= distance_in_tiles
}
