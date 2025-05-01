use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::gameworld::highlight::Highlight;
use crate::gameworld::world::GameWorld;
use crate::npcs::npc_components::npc_base::NPCBase;
use crate::npcs::npc_components::npc_type::NPCType;
use crate::npcs::player::Player;
use crate::{EndTurn, SelectedNPC};

pub fn setup_ui(mut commands: Commands) {
    // Collapsed state indicator (>>)
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "<<",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(5.0),
                top: Val::Px(5.0),
                display: Display::Flex, // Initially visible
                ..Default::default()
            },
            ..Default::default()
        },
        CollapsedText, // Marker component for the collapsed state
    ));

    // Expanded panel
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.0),  // 20% width
                height: Val::Percent(100.0), // Full height
                position_type: PositionType::Absolute,
                right: Val::Px(0.0), // Align to the right
                top: Val::Px(0.0),   // Align to the top
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column, // Stack children vertically
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)), // Dark gray background
            ..Default::default()
        },
        UIPanel, // Marker component for the panel
    ))
    .with_children(|parent| {
        // Add a collapse button (<<) in the top-right corner
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    position_type: PositionType::Absolute,
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)), // Button color
                ..Default::default()
            },
            CollapseButton, // Marker component for the button
        ))
        .with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    ">>",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });

        // Add a title text below the button
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Selected NPC:",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            style: Style {
                margin: UiRect {
                    top: Val::Px(40.0), // Add margin to push it below the button
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                },
                ..Default::default()
            },
            ..Default::default()
        });

        // Add a placeholder for NPC information
        parent.spawn((
            TextBundle {
                text: Text::from_section(
                    "No NPC Selected",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::GRAY,
                        ..Default::default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)), // Add some margin
                    ..Default::default()
                },
                ..Default::default()
            },
            SelectedNPCText, // Marker component for the NPC info text
        ));

        // Placeholder for the agent action button (initially hidden)
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0), // Align to the bottom
                    right: Val::Px(10.0),  // Align to the right
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::None, // Initially hidden
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(0.2, 0.8, 0.2)), // Green button
                ..Default::default()
            },
            AgentActionButton, // Marker component for the button
        ))
        .with_children(|button| {
            button.spawn(TextBundle {
                text: Text::from_section(
                    "Display Tree",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
    });

    // Add an "End Turn" button at the top-left corner
    commands.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0), // Move to the left
                top: Val::Px(10.0),  // Keep it at the top
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgb(0.8, 0.2, 0.2)), // Red button
            ..Default::default()
        },
        EndTurnButton, // Marker component for the "End Turn" button
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "End Turn",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            ..Default::default()
        });
    });
}

// Marker components
#[derive(Component)]
pub struct UIPanel;

#[derive(Component)]
pub struct CollapseButton;

#[derive(Component)]
pub struct SelectedNPCText;

#[derive(Component)]
pub struct CollapsedText;

#[derive(Component)]
pub struct EndTurnButton;

// Add a new marker component for the agent action button
#[derive(Component)]
pub struct AgentActionButton;

#[derive(Resource)]
pub struct PanelState {
    pub is_collapsed: bool,
}

pub fn toggle_panel_system(
    mut panel_state: ResMut<PanelState>,
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &Children,
    ), (Changed<Interaction>, With<CollapseButton>)>,
    mut panel_query: Query<&mut Style, With<UIPanel>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut background_color, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Toggle the panel state
                panel_state.is_collapsed = !panel_state.is_collapsed;

                // Update the panel's visibility
                if let Ok(mut panel_style) = panel_query.get_single_mut() {
                    panel_style.display = if panel_state.is_collapsed {
                        Display::None
                    } else {
                        Display::Flex
                    };
                }

                // Update the button text
                for &child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].value = if panel_state.is_collapsed {
                            ">>".to_string()
                        } else {
                            "<<".to_string()
                        };
                    }
                }

                *background_color = BackgroundColor(Color::rgb(0.4, 0.4, 0.4)); // Change button color on click
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::rgb(0.5, 0.5, 0.5)); // Change button color on hover
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::rgb(0.3, 0.3, 0.3)); // Default button color
            }
        }
    }
}


pub fn end_turn_button_system(
    mut commands: Commands,
    mut end_turn: ResMut<EndTurn>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<EndTurnButton>)>,
    player_query: Query<&NPCBase, With<Player>>,
    monsters_query: Query<&NPCBase>,
    game_world: Res<GameWorld>,
    highlight_query: Query<Entity, With<Highlight>>,
) {
    let mut should_highlight = false; // Flag to track if highlights should be created

    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Set the EndTurn resource to true
                end_turn.0 = true;
                println!("End Turn triggered!");

                // Set the flag to true
                should_highlight = true;

                // Change button color on click
                *background_color = BackgroundColor(Color::rgb(0.6, 0.1, 0.1));
            }
            Interaction::Hovered => {
                // Change button color on hover
                *background_color = BackgroundColor(Color::rgb(0.9, 0.3, 0.3));
            }
            Interaction::None => {
                // Reset button color
                *background_color = BackgroundColor(Color::rgb(0.8, 0.2, 0.2));
            }
        }
    }
}



