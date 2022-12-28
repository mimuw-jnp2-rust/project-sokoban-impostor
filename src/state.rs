#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum DisplayState {
    Game,
    MainMenu,
    LevelSelect,
    Victory,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Moving,
    Static,
    NotInGame,
}
