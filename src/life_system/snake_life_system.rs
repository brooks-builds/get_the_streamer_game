use ggez::graphics::Rect;

use super::LifeSystem;

#[derive(Debug)]
pub struct SnakeLifeSystem {
    on_screen: bool,
}

impl SnakeLifeSystem {
    pub fn new() -> Self {
        Self { on_screen: true }
    }
}

impl LifeSystem for SnakeLifeSystem {
    fn is_alive(&self) -> bool {
        self.on_screen
    }

    fn hit(&mut self) {}

    fn update(&mut self, screen_size: (f32, f32), location: &Rect) {
        if location.x + location.w < 0.0 || location.x > screen_size.0 {
            self.on_screen = false;
        }
    }

    fn gain_life(&mut self) {}

    fn get_lives_left(&self) -> u8 {
        0
    }
}
