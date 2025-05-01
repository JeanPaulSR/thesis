use bevy::prelude::*;
use bevy::ui::{Style, Val}; // Ensure these are imported
use crate::npcs::player::Player;
use crate::npcs::npc_components::npc_base::NPCBase;

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct PlayerHealthBarBackground;

pub fn setup_player_health_ui(mut commands: Commands) {
    // Create a health bar UI in the bottom-left corner
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    width: Val::Px(200.0), // Background width
                    height: Val::Px(20.0), // Background height
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)), // Background color
                ..Default::default()
            },
            PlayerHealthBarBackground,
        ))
        .with_children(|parent| {
            // Add the actual health bar (foreground)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0), // Full width initially
                        height: Val::Percent(100.0), // Full height
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.0, 1.0, 0.0)), // Green color for the health bar
                    ..Default::default()
                },
                PlayerHealthBar,
            ));
        });
}

pub fn update_player_health_ui(
    player_query: Query<&NPCBase, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<PlayerHealthBar>>,
) {
    if let Ok(player_npc_base) = player_query.get_single() {
        let energy = player_npc_base.get_energy();
        let max_energy = player_npc_base.get_max_energy();
        let percentage = (energy as f32 / max_energy as f32) * 100.0;

        for mut style in health_bar_query.iter_mut() {
            style.width = Val::Percent(percentage); // Update the width of the health bar
        }
    }
}