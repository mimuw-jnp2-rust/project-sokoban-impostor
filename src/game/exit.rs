use bevy::prelude::*;

use crate::{resources::Board, state::GameState};

use super::GameItem;

pub fn exit_to_main_menu(
    mut board: ResMut<Board>,
    query: Query<Entity, With<GameItem>>,
    mut commands: Commands,
) {
    board.entities.clear();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_esc(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<GameState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_state
            .push(GameState::MainMenu)
            .expect("Could not exit to main menu");
    }
}
