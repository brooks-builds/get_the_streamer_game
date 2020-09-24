use ggez::graphics::Rect;

use super::LifeSystem;

#[derive(Debug)]
pub struct HeartLifeSystem {
    alive: bool,
    die_in: u8,
}

impl HeartLifeSystem {
    pub fn new() -> Self {
        Self {
            alive: true,
            die_in: 2,
        }
    }
}

impl LifeSystem for HeartLifeSystem {
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn hit(&mut self) {
        self.die_in -= 1;
    }

    fn update(&mut self, screen_size: (f32, f32), location: &Rect) {
        if location.y > screen_size.1 {
            self.alive = false;
        }

        if self.die_in == 0 {
            self.alive = false;
        }
    }

    fn gain_life(&mut self) {}

    fn get_lives_left(&self) -> u8 {
        0
    }
}
