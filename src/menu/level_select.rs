use bevy::prelude::*;

use crate::{
    consts::{LEVEL_AMOUNT, MAIN_MENU_FONT, MAP_NAMES},
    resources::CurrentLevel,
    state::DisplayState,
};

use super::spawn_button;
#[derive(Component)]
pub struct LevelSelectItem;

#[derive(Component)]
pub enum LevelSelectItemType {
    Level(usize),
    Back,
}

pub fn setup_level_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility { is_visible: true },
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelSelectItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Level Select",
                    TextStyle {
                        font: menu_font.clone(),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
            for level_number in 0..LEVEL_AMOUNT {
                spawn_button(
                    parent,
                    LevelSelectItemType::Level(level_number + 1),
                    menu_font.clone(),
                    format!("Level {}", level_number + 1).as_str(),
                );
            }

            spawn_button(parent, LevelSelectItemType::Back, menu_font.clone(), "back");
        });
}

pub fn handle_level_click(
    mut app_state: ResMut<State<DisplayState>>,
    mut query: Query<
        (
            &mut Interaction,
            &mut BackgroundColor,
            &mut LevelSelectItemType,
        ),
        With<LevelSelectItemType>,
    >,
    mut current_level: ResMut<CurrentLevel>,
) {
    query.for_each_mut(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Clicked => match item.as_ref() {
                LevelSelectItemType::Level(number) => {
                    *current_level = CurrentLevel {
                        level_number: *number,
                        level_map_str: MAP_NAMES[*number - 1],
                    };
                    app_state
                        .push(DisplayState::Game)
                        .expect("Failed to load game");
                }
                LevelSelectItemType::Back => {
                    app_state.pop().expect("Going back to main menu failed");
                }
            },
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
        },
    )
}

// pub fn delete_level_select(mut commands: Commands, query: Query<Entity, With<LevelSelectItem>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }
