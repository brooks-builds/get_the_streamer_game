#[derive(Clone, Copy, PartialEq)]
pub enum RunningState {
    Playing,
    PlayerWon,
    ChatWon,
}

impl RunningState {
    pub fn is_game_over(&self) -> bool {
        match self {
            RunningState::Playing => false,
            RunningState::PlayerWon | RunningState::ChatWon => true,
        }
    }
}
