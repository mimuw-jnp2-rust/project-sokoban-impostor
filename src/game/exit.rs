use bevy::prelude::*;

use crate::{
    resources::{Board, Goals},
    state::DisplayState,
};

use super::GameItem;

pub fn exit_to_main_menu(
    mut board: ResMut<Board>,
    mut goals: ResMut<Goals>,
    query: Query<Entity, With<GameItem>>,
    mut commands: Commands,
) {
    board.clear();
    goals.goals.clear();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_esc(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<DisplayState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_state
            .push(DisplayState::MainMenu)
            .expect("Could not exit to main menu");
    }
}
