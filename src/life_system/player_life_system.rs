use ggez::graphics::Rect;

use super::LifeSystem;

const IFRAMES_REDUCE_EVERY: u32 = 720;
const IFRAMES_REDUCE_BY: u8 = 10;
const IFRAMES_MIN: u8 = 10;
const IFRAMES_START: u8 = 120;

#[derive(Debug)]
pub struct PlayerLifeSystem {
    lives: u8,
    iframes_left: u8,
    iframes: u8,
    frame_count: u32,
}

impl PlayerLifeSystem {
    pub fn new() -> PlayerLifeSystem {
        PlayerLifeSystem {
            lives: 3,
            iframes_left: 0,
            iframes: IFRAMES_START,
            frame_count: IFRAMES_REDUCE_EVERY,
        }
    }
}

impl LifeSystem for PlayerLifeSystem {
    fn is_alive(&self) -> bool {
        self.lives > 0
    }

    fn hit(&mut self) -> bool {
        if self.iframes_left == 0 {
            self.lives -= 1;
            self.iframes_left = self.iframes;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _screen_size: (f32, f32), _location: &Rect) {
        if self.iframes > IFRAMES_MIN {
            self.frame_count -= 1;
            if self.frame_count == 0 {
                self.frame_count = IFRAMES_REDUCE_EVERY;
                self.iframes -= IFRAMES_REDUCE_BY;
            }
        }

        if self.iframes_left > 0 {
            self.iframes_left -= 1;
        }
    }

    fn gain_life(&mut self) {
        self.lives += 1;
    }

    fn get_lives_left(&self) -> u8 {
        self.lives
    }
}
