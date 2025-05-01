use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::gameworld::position::Position;
use crate::gameworld::highlight::Highlight;
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::player::Player;
use crate::HighlightMovement;

pub fn move_player(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<&mut NPCBase, With<Player>>, // Query for the NPCBase of the player
        Query<(Entity, &Position), With<Highlight>>, // Query for highlighted tiles
    )>,
    mouse_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut highlight_movement: ResMut<HighlightMovement>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = window_query.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    if let Some(cursor_world_position) =
                        camera.viewport_to_world(camera_transform, cursor_position)
                    {
                        let cursor_world_position = cursor_world_position.origin.truncate();

                        // Check if the clicked position is within a highlighted square
                        let highlighted_positions: Vec<(Entity, Position)> = param_set.p1()
                            .iter()
                            .map(|(entity, position)| (entity, *position))
                            .collect();

                        for (entity, highlight_position) in highlighted_positions.iter() {
                            let highlight_world_position = Vec2::new(
                                highlight_position.x as f32 * 32.0,
                                highlight_position.y as f32 * 32.0,
                            );

                            let half_size = Vec2::new(16.0, 16.0); // Half the size of a 32x32 square
                            let min_bounds = highlight_world_position - half_size;
                            let max_bounds = highlight_world_position + half_size;

                            if (min_bounds.x..=max_bounds.x).contains(&cursor_world_position.x)
                                && (min_bounds.y..=max_bounds.y).contains(&cursor_world_position.y)
                            {
                                // Simulate confirmation (replace with actual UI logic)
                                let player_confirms = true; // Replace with actual confirmation logic

                                if player_confirms {
                                    // Move the player
                                    if let Ok(mut player_npc_base) = param_set.p0().get_single_mut() {
                                        player_npc_base.position = *highlight_position;
                                        player_npc_base.transform.translation = Vec3::new(
                                            highlight_position.x as f32 * 32.0,
                                            highlight_position.y as f32 * 32.0,
                                            1.0,
                                        );

                                        // Update the player's transform in the ECS
                                        commands
                                            .entity(player_npc_base.entity)
                                            .insert(player_npc_base.transform.clone());

                                        // Remove all highlighted squares
                                        for (entity, _) in param_set.p1().iter() {
                                            commands.entity(entity).despawn();
                                        }
                                    }
                                }

                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
