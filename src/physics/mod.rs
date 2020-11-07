pub mod fire_physics;
mod heart_physics;
mod player_physics;
mod snake_physics;
mod sword_physics;

use crate::life_system::LifeSystem;

use super::{Chatter, GameObject};
use eyre::Result;
pub use fire_physics::FirePhysics;
use ggez::graphics::Rect;
use ggez::Context;
pub use heart_physics::HeartPhysics;
pub use player_physics::PlayerPhysics;
pub use snake_physics::SnakePhysics;
pub use sword_physics::SwordPhysics;

pub trait PhysicsSystem
where
    Self: std::fmt::Debug,
{
    #[allow(clippy::clippy::too_many_arguments)]
    fn update(
        &mut self,
        location: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
        collidable_game_objects: &[GameObject],
        rotation: &mut f32,
        life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()>;

    fn get_velocity_x(&self) -> f32;
}
