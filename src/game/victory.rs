use bevy::prelude::*;

use crate::consts::MAIN_MENU_FONT;
use crate::resources::{Board, Goals};
use crate::state::GameState;

use super::game_objects::GameObjects;

#[derive(Component)]
pub struct VictoryItem;

pub fn handle_win(goals: Res<Goals>, board: Res<Board>, mut app_state: ResMut<State<GameState>>) {
    let mut is_win = true;
    for position in goals.goals.iter() {
        if board.entities.get(position).unwrap_or(&GameObjects::Empty) != &GameObjects::Box(None) {
            is_win = false;
        }
    }
    if is_win {
        app_state
            .push(GameState::Victory)
            .expect("Error while going to victory");
    }
}

pub fn setup_win(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::LIME_GREEN),
            visibility: Visibility { is_visible: true },
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(VictoryItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Level completed!\n
                    Press Enter to enter the main menu.",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        font: menu_font.clone(),
                    },
                )
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
        });
}

pub fn handle_win_click(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        app_state
            .push(GameState::MainMenu)
            .expect("Could not go out of victory screen");
    }
}

pub fn delete_win(query: Query<Entity, With<VictoryItem>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
