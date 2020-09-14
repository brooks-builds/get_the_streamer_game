mod game_object_draw_system;
mod timer_draw_system;

use super::Sprite;
pub use game_object_draw_system::GameObjectDrawSystem;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
pub use timer_draw_system::TimerDrawSystem;

pub trait DrawSystem
where
    Self: std::fmt::Debug,
{
    fn update(&mut self, time_since_start: std::time::Duration);
    fn draw(
        &self,
        context: &mut Context,
        location: Point2<f32>,
        rotation: &f32,
        iframes: bool,
    ) -> GameResult<()>;
    fn get_size(&self) -> Option<(f32, f32)>;
}
