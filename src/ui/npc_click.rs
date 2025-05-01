use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::npcs::agent::Agent;
use crate::npcs::monster::Monster;
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::npc_components::npc_type::NPCType;
use crate::npcs::treasure::Treasure;
use crate::SelectedNPC;
use crate::ui::setup_ui::SelectedNPCText;

pub fn npc_click_system(
    mut selected_npc: ResMut<SelectedNPC>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    npc_query: Query<(Entity, &NPCBase)>,
    button_query: Query<(&Style, &GlobalTransform), With<crate::ui::setup_ui::EndTurnButton>>, // Query the button's style and transform
    panel_query: Query<&GlobalTransform, With<crate::ui::setup_ui::UIPanel>>, // Query the panel's transform
) {
    // Check if the left mouse button was just pressed
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                // Check if the click is within the "End Turn" button's area
                if let Ok((button_style, button_transform)) = button_query.get_single() {
                    let button_position = button_transform.translation().truncate();
                    let button_size = Vec2::new(
                        match button_style.width {
                            Val::Px(value) => value,
                            _ => 0.0,
                        },
                        match button_style.height {
                            Val::Px(value) => value,
                            _ => 0.0,
                        },
                    );

                    let button_min = button_position - button_size / 2.0;
                    let button_max = button_position + button_size / 2.0;

                    if cursor_position.x >= button_min.x
                        && cursor_position.x <= button_max.x
                        && cursor_position.y >= button_min.y
                        && cursor_position.y <= button_max.y
                    {
                        // Ignore clicks in the button's area
                        return;
                    }
                }

                // Check if the click is within the panel's area
                if let Ok(panel_transform) = panel_query.get_single() {
                    let panel_position = panel_transform.translation().truncate();
                    let panel_size = Vec2::new(160.0, 600.0); // Adjust based on your panel size

                    let panel_min = panel_position - panel_size / 2.0;
                    let panel_max = panel_position + panel_size / 2.0;

                    if cursor_position.x >= panel_min.x
                        && cursor_position.x <= panel_max.x
                        && cursor_position.y >= panel_min.y
                        && cursor_position.y <= panel_max.y
                    {
                        // Ignore clicks in the panel's area
                        return;
                    }
                }

                // Existing NPC click logic...
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    if let Some(cursor_world_position) =
                        camera.viewport_to_world(camera_transform, cursor_position)
                    {
                        let cursor_world_position = cursor_world_position.origin.truncate();

                        let mut closest_npc: Option<(Entity, &NPCBase, Vec2, f32, Vec2)> = None;

                        for (entity, npc_base) in npc_query.iter() {
                            let transform = &npc_base.transform;
                            let sprite_size = npc_base
                                .sprite_bundle
                                .sprite
                                .custom_size
                                .unwrap_or(Vec2::new(32.0, 32.0));

                            let npc_position = transform.translation.truncate();
                            let distance = npc_position.distance(cursor_world_position);

                            if closest_npc.is_none() || distance < closest_npc.unwrap().3 {
                                closest_npc =
                                    Some((entity, npc_base, npc_position, distance, sprite_size));
                            }
                        }

                        if let Some((entity, npc_base, npc_position, distance, npc_size)) =
                            closest_npc
                        {
                            let half_size = npc_size / 2.0;

                            if (npc_position.x - half_size.x..=npc_position.x + half_size.x)
                                .contains(&cursor_world_position.x)
                                && (npc_position.y - half_size.y..=npc_position.y + half_size.y)
                                    .contains(&cursor_world_position.y)
                            {
                                selected_npc.0 = Some(entity);
                                return;
                            }
                        }

                        selected_npc.0 = None;
                    }
                }
            }
        }
    }
}

pub fn update_selected_npc_text(
    selected_npc: Res<SelectedNPC>,
    npc_query: Query<&NPCBase>,
    agent_query: Query<&Agent>,
    monster_query: Query<&Monster>,
    treasure_query: Query<&Treasure>,
    mut text_query: Query<&mut Text, With<SelectedNPCText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Some(selected_entity) = selected_npc.0 {
            if let Ok(npc_base) = npc_query.get(selected_entity) {
                // Display personalized information based on the NPC type
                let npc_info = match npc_base.npc_type {
                    NPCType::Agent => {
                        if let Ok(agent) = agent_query.get(selected_entity) {
                            format!(
                                "NPC Type: Agent\nPosition: {:?}\nEnergy: {}/{}\nID: {}\nStatus: {:?}\nAction: {:?}",
                                npc_base.position,
                                npc_base.energy,
                                npc_base.max_energy,
                                agent.get_id(),
                                agent.get_status(),
                                agent.get_action(),
                            )
                        } else {
                            "Error: Unable to retrieve Agent information.".to_string()
                        }
                    }
                    NPCType::Monster => {
                        if let Ok(monster) = monster_query.get(selected_entity) {
                            format!(
                                "NPC Type: Monster\nPosition: {:?}\nEnergy: {}/{}\nReward: {}",
                                npc_base.position,
                                npc_base.energy,
                                npc_base.max_energy,
                                monster.get_reward()
                            )
                        } else {
                            "Error: Unable to retrieve Monster information.".to_string()
                        }
                    }
                    NPCType::Treasure => {
                        if let Ok(treasure) = treasure_query.get(selected_entity) {
                            format!(
                                "NPC Type: Treasure\nPosition: {:?}\nReward: {}",
                                npc_base.position,
                                treasure.get_reward()
                            )
                        } else {
                            "Error: Unable to retrieve Treasure information.".to_string()
                        }
                    }
                    NPCType::Player => {
                        format!(
                            "NPC Type: Player\nPosition: {:?}\nEnergy: {}/{}",
                            npc_base.position,
                            npc_base.energy,
                            npc_base.max_energy,
                        )
                    }
                };

                // Update the text with the NPC's personalized information
                text.sections[0].value = npc_info;
            } else {
                // If the selected entity is invalid, reset the text
                text.sections[0].value = "No NPC Selected".to_string();
            }
        } else {
            // If no NPC is selected, reset the text
            text.sections[0].value = "No NPC Selected".to_string();
        }
    }
}