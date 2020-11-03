mod game_object_draw_system;
mod player_draw_system;

use super::Sprite;
pub use game_object_draw_system::GameObjectDrawSystem;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
pub use player_draw_system::PlayerDrawSystem;

pub trait DrawSystem
where
    Self: std::fmt::Debug,
{
    fn update(&mut self, time_since_start: std::time::Duration, velocity_x: f32);
    fn draw(&self, context: &mut Context, location: Point2<f32>, rotation: &f32) -> GameResult<()>;
    fn get_size(&self) -> Option<(f32, f32)>;
}
