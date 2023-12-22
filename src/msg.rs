#[derive(Debug, Eq, PartialEq)]
pub enum MetaMsg {
    LevelUp,
    LevelDown,
    PauseGame,
    ResumeGame,
    SuspendGame,
    NewGame,
    NextLevel,
    Quit,
    Menu,
    MenuMouseUp,
    About,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameMsg {
    PieceLanded,
    DropPiece,
}
