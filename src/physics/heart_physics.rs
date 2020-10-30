use crate::{game_object::GameObject, game_object_type::GameObjectType, life_system::LifeSystem};

use super::PhysicsSystem;
use eyre::Result;
use ggez::{graphics::Rect, Context};

#[derive(Debug)]
pub struct HeartPhysics {
    velocity_y: f32,
}

impl HeartPhysics {
    pub fn new() -> Self {
        Self { velocity_y: 0.0 }
    }
}

impl PhysicsSystem for HeartPhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        _screen_size: (f32, f32),
        gravity_force: f32,
        _context: &mut Context,
        collidable_game_objects: &[GameObject],
        _rotation: &mut f32,
        life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()> {
        self.velocity_y += gravity_force / 4.0;
        location.y += self.velocity_y;

        if let Some(player) = collidable_game_objects
            .iter()
            .find(|game_object| game_object.my_type == GameObjectType::Player)
        {
            if location.overlaps(&player.location) {
                if let Some(life_system) = life_system {
                    life_system.hit();
                }
            }
        }

        Ok(())
    }

    fn get_velocity_x(&self) -> f32 {
        0.0
    }
}
