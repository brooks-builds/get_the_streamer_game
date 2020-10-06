use ggez::GameResult;

pub struct Credits {}

impl Credits {
    pub fn new() -> GameResult<Self> {
        Ok(Credits {})
    }

    pub fn update(&mut self) {}
}
