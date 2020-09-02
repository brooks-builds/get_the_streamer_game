use super::PhysicsSystem;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::Context;

const MOVE_SPEED: f32 = 1.0;

pub struct ItemPhysics {
    velocity: Point2<f32>,
    affected_by_gravity: bool,
}

impl ItemPhysics {
    pub fn new() -> ItemPhysics {
        ItemPhysics {
            velocity: Point2::new(0.0, 0.0),
            affected_by_gravity: true,
        }
    }
}

impl PhysicsSystem for ItemPhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        screen_height: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
    ) {
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
    }
}
