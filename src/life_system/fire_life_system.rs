use std::time::{Duration, Instant};

use ggez::graphics::Rect;

use super::LifeSystem;

#[derive(Debug)]
pub struct FireLifeSystem {
    birth_time: Instant,
    time_to_live: Duration,
}

impl FireLifeSystem {
    pub fn new() -> FireLifeSystem {
        FireLifeSystem {
            birth_time: Instant::now(),
            time_to_live: Duration::from_secs(6),
        }
    }
}

impl LifeSystem for FireLifeSystem {
    fn is_alive(&self) -> bool {
        self.birth_time.elapsed() < self.time_to_live
    }

    fn hit(&mut self) {}

    fn update(&mut self, _screen_size: (f32, f32), _location: &Rect) {}

    fn gain_life(&mut self) {}

    fn get_lives_left(&self) -> u8 {
        0
    }
}
