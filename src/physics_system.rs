use ggez::graphics::Rect;
use ggez::nalgebra::Point2;

const GRAVITY_Y: f32 = 0.3;

pub struct PhysicsSystem {
    velocity: Point2<f32>,
    affected_by_gravity: bool,
}

impl PhysicsSystem {
    pub fn new(affected_by_gravity: bool) -> PhysicsSystem {
        PhysicsSystem {
            velocity: Point2::new(0.0, 0.0),
            affected_by_gravity,
        }
    }

    pub fn update(&mut self, location: &mut Rect, screen_height: f32) {
        if self.affected_by_gravity {
            self.velocity.y += GRAVITY_Y;
        }

        location.x += self.velocity.x;
        location.y += self.velocity.y;

        if location.y + location.h >= screen_height {
            self.velocity.y = 0.0;
            self.affected_by_gravity = false;
            location.y = screen_height - location.h;
        }
    }
}
