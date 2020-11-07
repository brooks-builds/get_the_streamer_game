mod dropzonearea;
pub mod gamewindow;
pub mod interface;
mod sidebar;
mod splash;
mod uitimer;

use ggez::{Context, GameResult};
pub use interface::Interface;

pub trait UIComponent {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn draw(&self, context: &mut Context, x: f32, y: f32) -> GameResult;
}
