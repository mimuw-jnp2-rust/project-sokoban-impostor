use bevy::prelude::*;

use crate::consts::MAIN_MENU_FONT;
use crate::resources::{Board, Goals, Images, VictoryTimer};
use crate::state::DisplayState;

use super::game_objects::{Box, GameObjects, Position};

#[derive(Component)]
pub struct VictoryItem;

pub fn handle_box_highlight(
    goals: Res<Goals>,
    images: Res<Images>,
    mut query: Query<(&mut Handle<Image>, &Position), With<Box>>,
) {
    for (mut handle, position) in query.iter_mut() {
        if goals.goals.contains(position) {
            *handle = images.box_on_goal_image.clone();
        } else {
            *handle = images.box_image.clone();
        }
    }
}

pub fn handle_win(
    goals: Res<Goals>,
    board: Res<Board>,
    mut app_state: ResMut<State<DisplayState>>,
    mut timer: ResMut<VictoryTimer>,
    time: Res<Time>,
) {
    let mut is_win = true;
    for position in goals.goals.iter() {
        if board.get_object_type(*position) != GameObjects::Box {
            is_win = false;
        }
    }
    if is_win {
        timer.0.tick(time.delta());
    } else {
        timer.0.reset();
    }
    if timer.0.finished() {
        app_state
            .push(DisplayState::Victory)
            .expect("Could not set state to victory");
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
    mut app_state: ResMut<State<DisplayState>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        app_state
            .push(DisplayState::MainMenu)
            .expect("Could not go out of victory screen");
    }
}

pub fn delete_win(query: Query<Entity, With<VictoryItem>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
