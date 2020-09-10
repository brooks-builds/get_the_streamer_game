use super::PhysicsSystem;
use crate::GameObject;
use eyre::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;
use ggez::Context;
use rand::prelude::*;

#[derive(Debug)]
pub struct SwordPhysics {
    velocity_x: f32,
    velocity_y: f32,
}

impl SwordPhysics {
    pub fn new() -> SwordPhysics {
        SwordPhysics {
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    fn is_first_fall(&self) -> bool {
        self.velocity_x == 0.0
    }

    fn calculate_rotation(&self) -> f32 {
        self.velocity_y.atan2(self.velocity_x) + std::f32::consts::PI * 0.25
    }
}

impl PhysicsSystem for SwordPhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
        rotation: &mut f32,
    ) -> Result<()> {
        self.velocity_y += gravity_force;
        location.y += self.velocity_y;
        location.x += self.velocity_x;
        *rotation = self.calculate_rotation();
        if location.y + location.h > screen_size.1 {
            location.y = screen_size.1 - location.h;
            self.velocity_y *= -0.9;

            if self.is_first_fall() {
                self.velocity_x = rand::random::<f32>() * 15.0;
            }
        }

        if location.x < 0.0 {
            location.x = 0.0;
            self.velocity_x *= -1.0;
        } else if location.x + location.w > screen_size.0 {
            location.x = screen_size.0 - location.w;
            self.velocity_x *= -1.0;
        }
        Ok(())
    }
}
