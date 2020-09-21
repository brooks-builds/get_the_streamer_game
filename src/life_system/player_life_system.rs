use ggez::graphics::Rect;

use super::LifeSystem;

#[derive(Debug)]
pub struct PlayerLifeSystem {
    lives: u8,
    iframes_left: u8,
}

impl PlayerLifeSystem {
    pub fn new() -> PlayerLifeSystem {
        PlayerLifeSystem {
            lives: 3,
            iframes_left: 0,
        }
    }
}

impl LifeSystem for PlayerLifeSystem {
    fn is_alive(&self) -> bool {
        self.lives > 0
    }

    fn hit(&mut self) {
        if self.iframes_left == 0 {
            self.lives -= 1;
            self.iframes_left = 120;
        }
    }

    fn update(&mut self, _screen_size: (f32, f32), _location: &Rect) {
        if self.iframes_left > 0 {
            self.iframes_left -= 1;
        }
    }
}
