use super::{GameObject, PhysicsSystem};
use eyre::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::Context;

#[derive(Debug)]
pub struct FirePhysics {
    velocity: Point2<f32>,
    affected_by_gravity: bool,
}

impl FirePhysics {
    pub fn new() -> FirePhysics {
        FirePhysics {
            velocity: Point2::new(0.0, 0.0),
            affected_by_gravity: true,
        }
    }
}

impl PhysicsSystem for FirePhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        screen_height: (f32, f32),
        gravity_force: f32,
        _context: &mut Context,
        _collidable_game_objects: &Vec<GameObject>,
        _rotation: &mut f32,
    ) -> Result<()> {
        if self.affected_by_gravity {
            self.velocity.y += gravity_force;
        }

        location.x += self.velocity.x;
        location.y += self.velocity.y;

        if location.y + location.h >= screen_height.1 {
            self.velocity.y = 0.0;
            self.affected_by_gravity = false;
            location.y = screen_height.1 - location.h;
        }

        Ok(())
    }
}
