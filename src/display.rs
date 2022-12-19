use crate::consts::*;
use crate::game_objects::{Box, GameObjects, Player, Position};
use crate::resources::{Board, MapSize, StartingPosition};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
fn spawn_tile_with_texture(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    image: Handle<Image>,
    position: Position,
    z_index: f32,
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
            z_index,
        ),
        ..default()
    });
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
) {
    let tile_image: Handle<Image> = asset_server.load(TILE_TEXTURE);
    let wall_image: Handle<Image> = asset_server.load(WALL_TEXTURE);
    let box_image: Handle<Image> = asset_server.load(BOX_TEXTURE);
    for i in (-(map_size.height as i32) / 2)..(map_size.height as i32 / 2) {
        for j in (-(map_size.width as i32) / 2)..(map_size.width as i32 / 2) {
            let entity = board
                .entities
                .get(&Position { x: j, y: i })
                .unwrap_or(&GameObjects::Empty);
            match entity {
                GameObjects::Box(None) => {
                    let result = spawn_box(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        box_image.clone(),
                        Position { x: j, y: i },
                    );
                    *board.entities.get_mut(&Position { x: j, y: i }).unwrap() =
                        GameObjects::Box(Some(result));
                }
                GameObjects::Wall => {
                    spawn_tile_with_texture(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        wall_image.clone(),
                        Position { x: j, y: i },
                        WALL_Z_INDEX,
                    );
                }
                _ => (),
            }
            //spawn background for every tile
            spawn_tile_with_texture(
                &mut commands,
                &mut meshes,
                &mut materials,
                tile_image.clone(),
                Position { x: j, y: i },
                TILE_Z_INDEX,
            );
        }
    }
    for y in (-(map_size.height as i32) / 2 - 1)..(map_size.height as i32 / 2 + 1) {
        let left_border_index = -(map_size.width as i32) / 2 - 1;
        let right_border_index = map_size.width as i32 / 2;
        spawn_tile_with_texture(
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x: left_border_index,
                y,
            },
            WALL_Z_INDEX,
        );
        spawn_tile_with_texture(
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x: right_border_index,
                y,
            },
            WALL_Z_INDEX,
        );
        board.entities.insert(
            Position {
                x: left_border_index,
                y,
            },
            GameObjects::Wall,
        );
        board.entities.insert(
            Position {
                x: right_border_index,
                y,
            },
            GameObjects::Wall,
        );
    }
    for x in (-(map_size.width as i32) / 2 - 1)..(map_size.width as i32 / 2 + 1) {
        let bottom_border_index = -(map_size.height as i32) / 2 - 1;
        let top_border_index = map_size.height as i32 / 2;
        spawn_tile_with_texture(
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x,
                y: top_border_index,
            },
            WALL_Z_INDEX,
        );
        spawn_tile_with_texture(
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x,
                y: bottom_border_index,
            },
            WALL_Z_INDEX,
        );
        board.entities.insert(
            Position {
                x,
                y: top_border_index,
            },
            GameObjects::Wall,
        );
        board.entities.insert(
            Position {
                x,
                y: bottom_border_index,
            },
            GameObjects::Wall,
        );
    }
}
pub fn spawn_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    image: Handle<Image>,
    position: Position,
) -> Entity {
    commands
        .spawn((
            Box,
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
                    BOX_Z_INDEX,
                ),
                ..default()
            },
        ))
        .id()
}

pub fn setup_move(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    starting_position: Res<StartingPosition>,
) {
    commands.spawn(Camera2dBundle::default());
    let player_image: Handle<Image> = asset_server.load(PLAYER_TEXTURE);
    let [starting_x, starting_y] = [starting_position.position.x, starting_position.position.y]
        .map(|el| TILE_SIZE * el as f32);
    commands.spawn((
        Player {
            position: starting_position.position,
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                })))
                .into(),
            material: materials.add(player_image.clone().into()),
            transform: Transform::from_xyz(starting_x, starting_y, PLAYER_Z_INDEX),
            ..default()
        },
    ));
}
