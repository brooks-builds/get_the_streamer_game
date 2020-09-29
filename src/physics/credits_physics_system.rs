use crate::{game_object::GameObject, life_system::LifeSystem};

use super::PhysicsSystem;
use eyre::Result;
use ggez::{graphics::Rect, Context};

#[derive(Debug)]
pub struct CreditsPhysicsSystem {
    velocity: f32,
}

impl CreditsPhysicsSystem {
    pub fn new() -> Self {
        Self { velocity: -1.0 }
    }
}

impl PhysicsSystem for CreditsPhysicsSystem {
    fn update(
        &mut self,
        location: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
        rotation: &mut f32,
        life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()> {
        println!("moving credits");
        dbg!("before move", &location);
        location.y -= 1.0;
        dbg!("after move:", &location);
        Ok(())
    }
}
