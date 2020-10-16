#[derive(Clone, Copy, PartialEq)]
pub enum RunningState {
    StartingSoon,
    Playing,
    PlayerWon,
    ChatWon,
}

impl RunningState {
    pub fn is_game_over(&self) -> bool {
        match self {
            RunningState::Playing | RunningState::StartingSoon => false,
            RunningState::PlayerWon | RunningState::ChatWon => true,
        }
    }
}
