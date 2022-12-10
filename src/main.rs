use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    utils::{Duration, HashMap},
};

mod consts;
mod game_objects;
mod resources;
use crate::consts::*;
use crate::resources::*;
use crate::game_objects::*;
use crate::game_objects::Direction;

fn main() {
    let mut entities = HashMap::new();
    for x in -2..4 {
        entities.insert(Position { x, y: 3 }, GameObjects::Wall);
        entities.insert(Position { x, y: -3 }, GameObjects::Wall);
    };

    App::new()
        .insert_resource(InputTimer(Timer::from_seconds(
            MOVE_DELAY,
            TimerMode::Repeating,
        )))
        .insert_resource(Board { entities })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_background)
        .add_startup_system(setup_walls)
        .add_startup_system(setup_move)
        .add_system(keyboard_input_system)
        .run();
}

fn update_pos(
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
    direction: Direction,
    mut timer: ResMut<InputTimer>,
    board: Res<Board>,
) {
    let (mut player, mut transform) = sprite_position.single_mut();
    let mut new_position = player.position;
    match direction {
        Direction::Up => {
            new_position.y += 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::Down => {
            new_position.y -= 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::Left => {
            new_position.x -= 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::Right => {
            new_position.x += 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::None => (),
    }
    if *(board
        .entities
        .get(&new_position)
        .unwrap_or(&GameObjects::Empty))
        == GameObjects::Empty
    {
        player.position = new_position;
        [transform.translation.x, transform.translation.y] =
            [player.position.x, player.position.y].map(|el| TILE_SIZE * el as f32);
    }
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>) -> Direction {
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        Direction::Right
    } else {
        Direction::None
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    sprite_position: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    mut timer: ResMut<InputTimer>,
    board: Res<Board>,
) {
    // move is only possible once every MOVE_DELAY seconds so only when timer is finished
    if timer.0.finished() {
        let dir = set_direction(keyboard_input);
        update_pos(sprite_position, dir, timer, board);
    } else {
        timer.0.tick(time.delta());
    }
}

fn setup_move(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let player_image: Handle<Image> = asset_server.load(PLAYER_TEXTURE);
    commands.spawn((
        Player {
            position: Position { x: 0, y: 0 },
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                })))
                .into(),
            material: materials.add(player_image.clone().into()),
            transform: Transform::from_xyz(
                0.,
                0.,
                1.,
            ),
            ..default()
        },
    ));
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tile_image: Handle<Image> = asset_server.load(TILE_TEXTURE);
    for i in 1..((BOX_HEIGHT / TILE_SIZE) as u32) {
        for j in 1..((BOX_WIDTH / TILE_SIZE) as u32) {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2 {
                        x: TILE_SIZE,
                        y: TILE_SIZE,
                    })))
                    .into(),
                material: materials.add(tile_image.clone().into()),
                transform: Transform::from_xyz(
                    j as f32 * TILE_SIZE - (BOX_WIDTH) / 2.,
                    i as f32 * TILE_SIZE - (BOX_HEIGHT) / 2.,
                    0.,
                ),
                ..default()
            });
        }
    }
}

fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    let wall_image: Handle<Image> = asset_server.load(WALL_TEXTURE);
    for (position, _element) in &board.entities {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                })))
                .into(),
            material: materials.add(wall_image.clone().into()),
            transform: Transform::from_xyz(
                position.x as f32 * TILE_SIZE,
                position.y as f32 * TILE_SIZE,
                1.,
            ),
            ..default()
        });
    }
}
