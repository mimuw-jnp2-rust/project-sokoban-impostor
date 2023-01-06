#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum DisplayState {
    Game,
    MainMenu,
    LevelSelect,
    Victory,
    Restarting,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Moving,
    Static,
    NotInGame,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Static
    }
}
