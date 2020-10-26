use eyre::Result;
use ggez::{graphics::Rect, Context};

use crate::{game_object::GameObject, game_object_type::GameObjectType, life_system::LifeSystem};

use super::PhysicsSystem;

const SNAKE_SPEED: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct SnakePhysics {
    velocity_x: f32,
    velocity_y: f32,
    affected_by_gravity: bool,
}

impl SnakePhysics {
    pub fn new() -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            affected_by_gravity: true,
        }
    }

    fn set_x_velocity(&mut self, collidable_game_objects: &Vec<GameObject>, sprite: &Rect) {
        let player = collidable_game_objects
            .iter()
            .find(|game_object| game_object.my_type == GameObjectType::Player);

        if let Some(player) = player {
            if player.location.x < sprite.x {
                self.velocity_x = -SNAKE_SPEED;
            } else {
                self.velocity_x = SNAKE_SPEED;
            }
        }
    }
}

impl PhysicsSystem for SnakePhysics {
    fn update(
        &mut self,
        sprite: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        _context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
        rotation: &mut f32,
        _life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()> {
        if self.affected_by_gravity {
            self.velocity_y += gravity_force;
            *rotation = std::f32::consts::FRAC_PI_3 + 0.3;
        }

        sprite.y += self.velocity_y;
        sprite.x += self.velocity_x;

        if sprite.y + sprite.h > screen_size.1 {
            sprite.y = screen_size.1 - sprite.h;
            self.velocity_y = 0.0;
            self.affected_by_gravity = false;
            self.set_x_velocity(collidable_game_objects, sprite);
            if self.velocity_x < 0.0 {
                *rotation = 3.14159;
            } else {
                *rotation = 0.0;
            }
        }
        Ok(())
    }

    fn get_velocity_x(&self) -> f32 {
        self.velocity_x
    }
}
