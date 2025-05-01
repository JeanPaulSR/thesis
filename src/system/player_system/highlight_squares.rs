use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::gameworld::position::Position;
use crate::gameworld::world::GameWorld;
use crate::gameworld::highlight::Highlight;
use crate::npcs::player::Player;
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::HighlightMovement;

pub fn highlight_moveable_player_squares(
    mut commands: Commands,
    mut highlight_movement: ResMut<HighlightMovement>,
    player_query: Query<&NPCBase, With<Player>>,
    monsters_query: Query<&NPCBase>,
    game_world: Res<GameWorld>,
    mut highlight_query: Query<Entity, With<Highlight>>,
    time: Res<Time>,
) {
    if !highlight_movement.0 {
        return; // Exit early if highlighting is not enabled
    }

    // Clear any existing highlights
    for entity in highlight_query.iter() {
        commands.entity(entity).despawn();
    }

    if let Ok(player_npc_base) = player_query.get_single() {
        let player_position = player_npc_base.position;

        // Define all 8 possible adjacent positions (cardinal + diagonal directions)
        let adjacent_positions = vec![
            Position::new(player_position.x + 1, player_position.y),     // Right
            Position::new(player_position.x - 1, player_position.y),     // Left
            Position::new(player_position.x, player_position.y + 1),     // Up
            Position::new(player_position.x, player_position.y - 1),     // Down
            Position::new(player_position.x + 1, player_position.y + 1), // Top-right
            Position::new(player_position.x - 1, player_position.y + 1), // Top-left
            Position::new(player_position.x + 1, player_position.y - 1), // Bottom-right
            Position::new(player_position.x - 1, player_position.y - 1), // Bottom-left
        ];

        for position in adjacent_positions {
            if let Some(tile_arc) = game_world.get_tile(position.x, position.y) {
                let tile = tile_arc.lock().unwrap();

                // Ensure the tile is traversable
                let is_traversable = tile.get_tile_type().get_travel_weight() > 0.0;

                // Ensure no monster is present on the tile
                let is_occupied = monsters_query.iter().any(|npc_base| npc_base.position == position);

                if is_traversable && !is_occupied {
                    // Calculate opacity oscillation between 5% and 10%
                    let opacity = 0.05 + 0.05 * (time.elapsed_seconds().sin() + 1.0) / 2.0;

                    // Spawn a highlight entity
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    position.x as f32 * 32.0,
                                    position.y as f32 * 32.0,
                                    2.0, // Set z-value higher than tiles to render above
                                ),
                                scale: Vec3::new(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, opacity), // White with varying opacity
                                custom_size: Some(Vec2::new(32.0, 32.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Highlight,
                        position, // Attach the position to the highlight
                    ));
                }
            }
        }
    }

    // Reset the HighlightMovement flag to prevent repeated execution
    highlight_movement.0 = false;
}

