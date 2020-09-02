use super::PhysicsSystem;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;
use ggez::{input, Context};

const MOVE_FORCE: f32 = 1.0;

pub struct PlayerPhysics {
    velocity: Point2<f32>,
    affected_by_gravity: bool,
}

impl PlayerPhysics {
    pub fn new() -> PlayerPhysics {
        PlayerPhysics {
            velocity: Point2::new(0.0, 0.0),
            affected_by_gravity: true,
        }
    }
}

impl PhysicsSystem for PlayerPhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
    ) {
        if self.affected_by_gravity {
            self.velocity.y += gravity_force;
        }
        location.x += self.velocity.x;
        location.y += self.velocity.y;

        if location.y + location.h > screen_size.1 {
            self.affected_by_gravity = false;
            self.velocity.y = 0.0;
            location.y = screen_size.1 - location.h;
        }

        if input::keyboard::is_key_pressed(context, KeyCode::A) {
            self.velocity.x -= MOVE_FORCE;
        } else if input::keyboard::is_key_pressed(context, KeyCode::S) {
            self.velocity.x += MOVE_FORCE;
        }
    }
}
