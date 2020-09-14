pub mod fire_physics;
mod player_physics;
mod sword_physics;
mod timer_physics;

use super::{Chatter, GameObject};
use eyre::Result;
pub use fire_physics::FirePhysics;
use ggez::graphics::Rect;
use ggez::Context;
pub use player_physics::PlayerPhysics;
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
    ) -> Result<bool>;
}
