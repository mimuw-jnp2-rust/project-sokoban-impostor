use bevy::prelude::*;

use crate::consts::MAIN_MENU_FONT;
use crate::resources::{Board, VictoryTimer};
use crate::state::DisplayState;

use super::game_objects::GameObject;

#[derive(Component)]
pub struct VictoryItem;

pub fn handle_win(
    board: Res<Board>,
    mut display_state: ResMut<State<DisplayState>>,
    mut timer: ResMut<VictoryTimer>,
    time: Res<Time>,
) {
    let mut is_win = true;
    for position in board.get_all_goals().iter() {
        if board.get_object_type(*position) != GameObject::Box {
            is_win = false;
        }
    }
    if is_win {
        timer.0.tick(time.delta());
    } else {
        timer.0.reset();
    }
    if timer.0.finished() {
        display_state
            .set(DisplayState::Victory)
            .expect("Could not set state to victory");
        timer.0.reset();
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
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<DisplayState>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        app_state
            .set(DisplayState::LevelSelect)
            .expect("Could not go out of victory screen");
        keyboard_input.reset(KeyCode::Return);
    }
}
