pub mod item_physics;
mod player_physics;

use ggez::graphics::Rect;
use ggez::Context;
pub use item_physics::ItemPhysics;
pub use player_physics::PlayerPhysics;

pub trait PhysicsSystem {
    fn update(
        &mut self,
        location: &mut Rect,
        screen_size: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
    );
}
