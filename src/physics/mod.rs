mod credits_physics_system;
pub mod fire_physics;
mod heart_physics;
mod player_physics;
mod snake_physics;
mod sword_physics;
mod timer_physics;

use crate::life_system::LifeSystem;

use super::{Chatter, GameObject};
pub use credits_physics_system::CreditsPhysicsSystem;
use eyre::Result;
pub use fire_physics::FirePhysics;
use ggez::graphics::Rect;
use ggez::Context;
pub use heart_physics::HeartPhysics;
pub use player_physics::PlayerPhysics;
pub use snake_physics::SnakePhysics;
pub use sword_physics::SwordPhysics;
pub use timer_physics::TimerPhysicsSystem;

pub trait PhysicsSystem
where
    Self: std::fmt::Debug,
{
    fn update(
        &mut self,
        location: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
        rotation: &mut f32,
        life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()>;
}
