use crate::{
    game::{
        game_objects::{GameObjects, Player, Position},
        GameItem,
    },
    resources::Board,
};

use crate::consts::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

//render an object with a given image and position
pub fn spawn_entity<T>(
    component: T,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    image: Handle<Image>,
    position: Position,
    z_index: f32,
) -> Entity
where
    T: Component,
{
    commands
        .spawn((
            component,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    })))
                    .into(),
                material: materials.add(image.into()),
                transform: Transform::from_xyz(
                    position.x as f32 * TILE_SIZE,
                    position.y as f32 * TILE_SIZE,
                    z_index,
                ),
                ..default()
            },
        ))
        .insert(GameItem)
        .id()
}

//spawn player entity
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
) {
    let player_image: Handle<Image> = asset_server.load(PLAYER_TEXTURE);
    let player_entity = spawn_entity(
        Player,
        &mut commands,
        &mut meshes,
        &mut materials,
        player_image,
        board.player_position,
        PLAYER_Z_INDEX,
    );
    let position = board.player_position;
    board
        .entities
        .insert(position, GameObjects::Player(Some(player_entity)));
}
