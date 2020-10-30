use crate::life_system::LifeSystem;

use super::{GameObject, PhysicsSystem};
use eyre::Result;
use ggez::graphics::Rect;
use ggez::Context;
use std::time::Duration;

#[derive(Debug)]
pub struct TimerPhysicsSystem {
    velocity_y: f32,
}

impl TimerPhysicsSystem {
    pub fn new(timer_size: f32, game_time: Duration, fps: f32) -> TimerPhysicsSystem {
        let pixels_per_second = timer_size / (game_time.as_secs_f32() - 0.2); //if i understand this correctly, this is 0.2 because that is the height of the drop zone labels at the top
        let pixels_per_frame = pixels_per_second / fps;
        TimerPhysicsSystem {
            velocity_y: pixels_per_frame,
        }
    }
}

impl PhysicsSystem for TimerPhysicsSystem {
    fn update(
        &mut self,
        location: &mut Rect,
        _screen_size: (f32, f32),
        _gravity_force: f32,
        _context: &mut Context,
        _collidable_game_objects: &[GameObject],
        _rotation: &mut f32,
        _life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()> {
        location.y += self.velocity_y;

        Ok(())
    }

    fn get_velocity_x(&self) -> f32 {
        0.0
    }
}
