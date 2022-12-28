// use bevy::prelude::*;

// use crate::events::MoveEvent;

// pub fn handle_keypress(keyboard: Res<Input<KeyCode>>, time: Res<Time>, writer: EventWriter<MoveEvent>) {
//     if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
//         writer.send()
//     }
//     else if keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {

//     }
//     else if keyboard.any_pressed([KeyCode::S, KeyCode::Down]) {

//     }
//     else if keyboard.any_pressed([KeyCode::D, KeyCode::Right]) {

//     }
use crate::consts::TILE_SIZE;
use crate::state::GameState;
use crate::Position;
use bevy::prelude::*;

use crate::game::game_objects::{Box, Direction, GameObjects, Player};
use crate::resources::{Board, InputTimer, MovementData, MovementEntities};

fn update_box(
    board: &mut ResMut<Board>,
    dir: Direction,
    pos: Position,
    current_entity: Entity,
    boxes_moved: &mut Vec<(Entity, Position)>,
) -> bool {
    let next_position = pos.neighbour(dir);
    let next_position_object = board
        .entities
        .get(&next_position)
        .unwrap_or(&GameObjects::Empty);
    match next_position_object {
        GameObjects::Empty => {
            boxes_moved.push((current_entity, pos));
            let my_box = board.entities.remove(&pos).expect("Box not found in board");
            board.entities.insert(next_position, my_box);
            true
        }
        GameObjects::Box(next_entity) => {
            let next_entity = next_entity.expect("Box not rendered yet");
            let can_move = update_box(board, dir, next_position, next_entity, boxes_moved);
            if can_move {
                boxes_moved.push((current_entity, pos));
                let my_box = board.entities.remove(&pos).expect("Box not found in board");
                board.entities.insert(next_position, my_box);
            }
            can_move
        }
        GameObjects::Wall => false,
        GameObjects::Player(_) => false,
    }
}

fn update_pos(
    direction: Direction,
    board: &mut ResMut<Board>,
    movement_data: &mut ResMut<MovementData>,
    app_state: &mut ResMut<State<GameState>>,
) {
    let new_position = board.player_position.neighbour(direction);
    if direction == Direction::None {
        ()
    }
    let object_blocking = board
        .entities
        .get(&new_position)
        .unwrap_or(&GameObjects::Empty);

    match object_blocking {
        GameObjects::Box(entity) => {
            let entity = entity.expect("Box not rendered");
            let mut boxes_moved = Vec::new();
            let can_move = update_box(board, direction, new_position, entity, &mut boxes_moved);
            if can_move {
                let player_object = board
                    .entities
                    .get(&board.player_position)
                    .expect("Player not found in board");
                match *player_object {
                    GameObjects::Player(Some(player_entity)) => {
                        movement_data.data = Some(MovementEntities{direction, player_data: (player_entity, board.player_position), boxes_data: boxes_moved});
                        app_state.push(GameState::Moving).expect("Could not enter moving state");
                    }
                    _ => panic!("Something other than rendered player on player position"),
                }
                let player_position = board.player_position;
                let player = board
                    .entities
                    .remove(&player_position)
                    .expect("Player not found in board");
                board
                    .entities
                    .insert(player_position.neighbour(direction), player);
                board.player_position = player_position.neighbour(direction);
            }
        }
        GameObjects::Wall => (),
        GameObjects::Player(_) => (),
        GameObjects::Empty => {
            let player_object = board
                .entities
                .get(&board.player_position)
                .expect("Player not found in board");
            match *player_object {
                GameObjects::Player(Some(player_entity)) => {
                    movement_data.data = Some(MovementEntities{direction, player_data: (player_entity, board.player_position), boxes_data: Vec::new()});
                    app_state.push(GameState::Moving).expect("Could not enter moving state");
                }
                _ => panic!("Something other than rendered player on player position"),
            }
            let player_position = board.player_position;
            let player = board
                .entities
                .remove(&player_position)
                .expect("Player not found in board");
            board
                .entities
                .insert(player_position.neighbour(direction), player);
            board.player_position = player_position.neighbour(direction);
        }
    }
}

fn set_direction(
    keyboard_input: Res<Input<KeyCode>>,
) -> Direction {
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        Direction::Right
    } else {
        Direction::None
    }
}

pub fn handle_keypress(
    keyboard_input: Res<Input<KeyCode>>,
    mut board: ResMut<Board>,
    mut movement_data: ResMut<MovementData>,
    mut app_state: ResMut<State<GameState>>,
) {
    let dir = set_direction(keyboard_input);
    update_pos(dir, &mut board, &mut movement_data, &mut app_state);
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Moving).with_system(move_animation))
        .add_system_set(SystemSet::on_resume(GameState::Moving).with_system(move_animation))
        .add_system_set(SystemSet::on_update(GameState::Moving).with_system(move_animation))
        .add_system_set(SystemSet::on_exit(GameState::Moving).with_system(end_animation))
        .add_system_set(SystemSet::on_pause(GameState::Moving).with_system(end_animation));

        app.add_system_set(SystemSet::on_update(GameState::Static).with_system(handle_keypress));
    }
}

fn modify_transform(mut transform: Mut<Transform>, direction: Direction, timer: &ResMut<InputTimer>, starting_position: Position) {
    match direction {
        Direction::Down => {
            transform.translation.y = (starting_position.y as f32 - timer.0.percent()) * TILE_SIZE;
        }
        Direction::Up => {
            transform.translation.y = (starting_position.y as f32 + timer.0.percent()) * TILE_SIZE;
        }
        Direction::Left => {
            transform.translation.x = (starting_position.x as f32 - timer.0.percent()) * TILE_SIZE;
        }
        Direction::Right => {
            transform.translation.x = (starting_position.x as f32 + timer.0.percent()) * TILE_SIZE;
        }
        _ => ()
    }
}

pub fn move_animation(
    time: Res<Time>,
    movement_data: Res<MovementData>,
    mut query: Query<&mut Transform, Or<(With<Player>, With<Box>)>>,
    mut timer: ResMut<InputTimer>,
    mut app_state: ResMut<State<GameState>>,
) {
    timer.0.tick(time.delta());
    let movement_data = movement_data.data.as_ref().expect("Movement data not initialized when entering movement");
    let (player_entity, player_position) = movement_data.player_data;
    let player_transform = query.get_mut(player_entity).expect("Player entity not found");
    let direction = movement_data.direction;
    modify_transform(player_transform, direction, &timer, player_position);
    for (box_entity, box_position) in movement_data.boxes_data.iter() {
        let transform = query.get_mut(*box_entity).expect("Moved box entity not found");
        modify_transform(transform, direction, &timer, *box_position);
    }
    if timer.0.finished() {
        app_state.push(GameState::Static).expect("Could not correctly finish movement animation");
    }
}

pub fn end_animation(
    mut movement_data: ResMut<MovementData>,
    mut timer: ResMut<InputTimer>,
) {
    movement_data.data = None;
    timer.0.reset();
}
