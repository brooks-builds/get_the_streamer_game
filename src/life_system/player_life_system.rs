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

    fn hit(&mut self) -> bool {
        if self.iframes_left == 0 {
            self.lives -= 1;
            self.iframes_left = 12;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _screen_size: (f32, f32), _location: &Rect) {
        if self.iframes_left > 0 {
            self.iframes_left -= 1;
        }

        #[cfg(debug_assertions)]
        println!("player has {} lives left", self.lives);
    }

    fn gain_life(&mut self) {
        #[cfg(debug_assertions)]
        println!("player gained a life");

        self.lives += 1;
    }

    fn get_lives_left(&self) -> u8 {
        self.lives
    }
}
