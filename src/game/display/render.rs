use crate::{
    game::{
        game_objects::{Player, Position},
        GameItem,
    },
    resources::{Board, Images},
};

use crate::consts::*;
use bevy::prelude::*;

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

//spawn player entity
pub fn spawn_player(mut commands: Commands, images: Res<Images>, mut board: ResMut<Board>) {
    let position = board.get_player_position();
    let player_entity = spawn_entity(
        Player,
        &mut commands,
        images.player_image.clone(),
        position,
        PLAYER_Z_INDEX,
    );
    board.insert_entity(position, player_entity);
}
