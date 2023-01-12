use background::render_board;
use bevy::prelude::*;

use crate::consts::*;
use crate::game::{game_objects::Position, GameItem};
use crate::labels::Labels;
use crate::resources::Images;
use crate::state::{GameState, Move, DisplayState};

use self::background::setup_border;
use self::text::{display_level_text, despawn_level_text};

pub mod background;
mod text;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Images>();
        app.add_startup_system(window_set_fullscreen);

        app.add_system_set(SystemSet::on_enter(DisplayState::Game)
            .with_system(display_level_text)
        );

        app.add_system_set(SystemSet::on_exit(DisplayState::Game).with_system(despawn_level_text));
        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Static)))
                .label(Labels::Display)
                .with_system(despawn_board.before(render_board).before(setup_border))
                .with_system(render_board)
                .with_system(setup_border),
        );
    }
}

//render an object with a given image and position
pub fn spawn_entity<T>(
    component: T,
    commands: &mut Commands,
    image: Handle<Image>,
    position: Position,
    z_index: f32,
) -> Entity
where
    T: Component,
{
    commands
        .spawn((SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(
                position.x as f32 * TILE_SIZE,
                position.y as f32 * TILE_SIZE,
                z_index,
            )
            .with_scale(Vec3::new(
                TILE_SIZE / IMAGE_SIZE,
                TILE_SIZE / IMAGE_SIZE,
                1.,
            )),
            ..default()
        },))
        .insert(component)
        .insert(GameItem)
        .id()
}

fn despawn_board(query: Query<Entity, With<GameItem>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn window_set_fullscreen(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_mode(WindowMode::BorderlessFullscreen);
}