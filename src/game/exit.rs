use bevy::prelude::*;

use crate::state::DisplayState;

use super::GameItem;

pub fn despawn_board(query: Query<Entity, With<GameItem>>, mut commands: Commands) {
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
            .set(DisplayState::MainMenu)
            .expect("Could not exit to main menu");
        keyboard_input.reset(KeyCode::Escape);
    }
}
