mod fire_life_system;
mod player_life_system;
mod snake_life_system;
mod sword_life_system;

pub use fire_life_system::FireLifeSystem;
use ggez::graphics::Rect;
pub use player_life_system::PlayerLifeSystem;
pub use snake_life_system::SnakeLifeSystem;
pub use sword_life_system::SwordLifeSystem;

pub trait LifeSystem
where
    Self: std::fmt::Debug,
{
    fn is_alive(&self) -> bool;
    fn hit(&mut self);
    fn update(&mut self, screen_size: (f32, f32), location: &Rect);
}
