use bevy::prelude::*;

mod level_select;
mod main_menu;
use crate::{
    resources::Board,
    state::{DisplayState, GameState},
};

use level_select::{delete_level_select, handle_level_click, setup_level_select};
use main_menu::{delete_main_menu, handle_menu_click, setup_main_menu};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::MainMenu)
                .with_system(setup_main_menu)
                .with_system(reset_game_state)
                .with_system(clear_board),
        )
        .add_system_set(SystemSet::on_update(DisplayState::MainMenu).with_system(handle_menu_click))
        .add_system_set(SystemSet::on_exit(DisplayState::MainMenu).with_system(delete_main_menu));

        app.add_system_set(
            SystemSet::on_enter(DisplayState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::LevelSelect).with_system(handle_level_click),
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::LevelSelect).with_system(delete_level_select),
        );
    }
}

pub fn spawn_button<T>(parent: &mut ChildBuilder, entity: T, menu_font: Handle<Font>, value: &str)
where
    T: Component,
{
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Px(30.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(entity)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    value,
                    TextStyle {
                        font_size: 15.0,
                        color: Color::BLACK,
                        font: menu_font,
                    },
                )
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
        });
}

pub fn reset_game_state(mut game_state: ResMut<State<GameState>>) {
    if game_state.current() != &GameState::NotInGame {
        game_state
            .overwrite_set(GameState::NotInGame)
            .expect("Could not reset game state");
    }
}

pub fn clear_board(mut board: ResMut<Board>) {
    board.clear();
}
