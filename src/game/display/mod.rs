use background::setup_background;
use bevy::prelude::*;
use render::spawn_player;

use crate::resources::Images;
use crate::state::DisplayState;

pub mod background;
pub mod render;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Images>();
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game)
                .with_system(spawn_player)
                .with_system(setup_background),
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::Game)
                .with_system(spawn_player)
                .with_system(setup_background),
        );
    }
}