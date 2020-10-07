#[derive(Clone, Copy)]
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

    pub fn did_chat_win(&self) -> bool {
        match self {
            RunningState::ChatWon => true,
            _ => false,
        }
    }
}
