use bevy::prelude::*;

use crate::{resources::Board, state::DisplayState};

use super::GameItem;

pub fn exit_to_main_menu(
    mut board: ResMut<Board>,
    query: Query<Entity, With<GameItem>>,
    mut commands: Commands,
) {
    board.clear();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_esc(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<DisplayState>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_state
            .push(DisplayState::MainMenu)
            .expect("Could not exit to main menu");
        keyboard_input.reset(KeyCode::Escape);
    }
}
