use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use crate::consts::*;
use crate::game_objects::{Position, Player};
use crate::resources::Board;
fn spawn_tile_with_texture(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    image: Handle<Image>,
    position: Position,
    height: f32,
) {
    commands.spawn(MaterialMesh2dBundle {
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
            height,
        ),
        ..default()
    });
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    board: ResMut<Board>,
) {
    let tile_image: Handle<Image> = asset_server.load(TILE_TEXTURE);
    let wall_image: Handle<Image> = asset_server.load(WALL_TEXTURE);
    let box_image: Handle<Image> = asset_server.load(BOX_TEXTURE);
    for i in (-(BOX_HEIGHT as i32) / 2)..(BOX_HEIGHT as i32 / 2) {
        for j in (-(BOX_WIDTH as i32) / 2)..(BOX_WIDTH as i32 / 2) {
            if board.entities.contains_key(&Position{x:j, y:i, movable: false}) {
                spawn_tile_with_texture(&mut commands, &mut meshes, &mut materials, wall_image.clone(), Position {x:j, y:i, movable: false}, 1.);
            }
            else if board.entities.contains_key(&Position{x:j, y:i, movable: true}) {
                spawn_tile_with_texture(&mut commands, &mut meshes, &mut materials, box_image.clone(), Position {x:j, y:i, movable: true}, 1.);
            }
            else {
                spawn_tile_with_texture(&mut commands, &mut meshes, &mut materials, tile_image.clone(), Position {x:j, y:i, movable: false}, 0.);
            }
        }
    }
}

pub fn setup_move(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let player_image: Handle<Image> = asset_server.load(PLAYER_TEXTURE);
    commands.spawn((
        Player {
            position: Position { x: 0, y: 0, movable: false },
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                })))
                .into(),
            material: materials.add(player_image.clone().into()),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
    ));
}
