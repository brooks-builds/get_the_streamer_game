use ggez::graphics::Rect;

use super::LifeSystem;

#[derive(Debug)]
pub struct SwordLifeSystem {
    bounces: u8,
    breaks_at: u8,
}

impl SwordLifeSystem {
    pub fn new() -> SwordLifeSystem {
        SwordLifeSystem {
            bounces: 0,
            breaks_at: 7,
        }
    }
}

impl LifeSystem for SwordLifeSystem {
    fn is_alive(&self) -> bool {
        self.bounces < self.breaks_at
    }

    fn hit(&mut self) -> bool {
        self.bounces += 1;
        true
    }

    fn update(&mut self, _screen_size: (f32, f32), _location: &Rect) {}

    fn gain_life(&mut self) {}

    fn get_lives_left(&self) -> u8 {
        0
    }
}
