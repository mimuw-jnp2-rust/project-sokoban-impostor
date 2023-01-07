use background::setup_background;
use bevy::prelude::*;

use crate::consts::*;
use crate::game::{game_objects::Position, GameItem};
use crate::labels::Labels;
use crate::resources::Images;
use crate::state::DisplayState;

use self::background::setup_border;

pub mod background;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Images>();
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game)
                .label(Labels::Display)
                .with_system(setup_background)
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
        .insert(position)
        .id()
}
