use super::{
    game_objects::{Background, Box, GameObjects, Goal, Player, Position, Wall},
    GameItem,
};
use crate::resources::{Board, MapSize, StartingPosition};
use crate::{consts::*, resources::Goals};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn offset_coordinate(coord: u32, max: u32) -> i32 {
    coord as i32 - (max as i32 / 2)
}

//render an object with a given image and position
fn spawn_entity<T>(
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

//render the entire map based on Board
pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
    goals: Res<Goals>,
) {
    let tile_image = asset_server.load(TILE_TEXTURE);
    let wall_image = asset_server.load(WALL_TEXTURE);
    let box_image = asset_server.load(BOX_TEXTURE);
    let goal_image = asset_server.load(GOAL_TEXTURE);
    let bottom_border = offset_coordinate(0, map_size.height);
    let top_border = offset_coordinate(map_size.height - 1, map_size.height);
    let left_border = offset_coordinate(0, map_size.width);
    let right_border = offset_coordinate(map_size.width - 1, map_size.width);
    // render all objects found in board
    for i in bottom_border..(top_border + 1) {
        for j in left_border..(right_border + 1) {
            let entity = board
                .entities
                .get(&Position { x: j, y: i })
                .unwrap_or(&GameObjects::Empty);
            match entity {
                GameObjects::Box(None) => {
                    let result = spawn_entity(
                        Box,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        box_image.clone(),
                        Position { x: j, y: i },
                        BOX_Z_INDEX,
                    );
                    *board
                        .entities
                        .get_mut(&Position { x: j, y: i })
                        .expect("Position not found in board") = GameObjects::Box(Some(result));
                }
                GameObjects::Wall => {
                    spawn_entity(
                        Wall,
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
            spawn_entity(
                Background,
                &mut commands,
                &mut meshes,
                &mut materials,
                tile_image.clone(),
                Position { x: j, y: i },
                TILE_Z_INDEX,
            );
        }
    }
    //spawn horizontal border for the level and insert it to board
    for y in (bottom_border - 1)..(top_border + 2) {
        spawn_entity(
            Wall,
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x: left_border - 1,
                y,
            },
            WALL_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x: right_border + 1,
                y,
            },
            WALL_Z_INDEX,
        );
        board.entities.insert(
            Position {
                x: left_border - 1,
                y,
            },
            GameObjects::Wall,
        );
        board.entities.insert(
            Position {
                x: right_border + 1,
                y,
            },
            GameObjects::Wall,
        );
    }
    //spawn vertical borders for the level and insert it to board
    for x in (left_border - 1)..(right_border + 2) {
        spawn_entity(
            Wall,
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x,
                y: top_border + 1,
            },
            WALL_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            &mut meshes,
            &mut materials,
            wall_image.clone(),
            Position {
                x,
                y: bottom_border - 1,
            },
            WALL_Z_INDEX,
        );
        board.entities.insert(
            Position {
                x,
                y: top_border + 1,
            },
            GameObjects::Wall,
        );
        board.entities.insert(
            Position {
                x,
                y: bottom_border - 1,
            },
            GameObjects::Wall,
        );
    }
    for position in goals.goals.iter() {
        spawn_entity(
            Goal,
            &mut commands,
            &mut meshes,
            &mut materials,
            goal_image.clone(),
            *position,
            GOAL_Z_INDEX,
        );
    }
}

//spawn player entity and setup the camera
pub fn setup_move(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    starting_position: Res<StartingPosition>,
) {
    let player_image: Handle<Image> = asset_server.load(PLAYER_TEXTURE);
    spawn_entity(
        Player {
            position: starting_position.position,
        },
        &mut commands,
        &mut meshes,
        &mut materials,
        player_image,
        starting_position.position,
        PLAYER_Z_INDEX,
    );
}
