use bevy::{prelude::*, app::AppExit};

use crate::state::DisplayState;

pub fn handle_esc(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<DisplayState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.current() == &DisplayState::MainMenu {
            app_exit.send(AppExit);
            return;                 //just in case to avoid weird behaviour before event is parsed
        }
        app_state
            .pop()
            .expect("Could not exit");
        keyboard_input.reset(KeyCode::Escape);
    }
}
