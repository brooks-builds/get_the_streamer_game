use super::draw_system::DrawSystem;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

pub struct GameObject {
    location: Rect,
    draw_system: Option<DrawSystem>,
    children: Vec<GameObject>,
}

impl GameObject {
    pub fn new(
        x: f32,
        y: f32,
        draw_system: Option<DrawSystem>,
        width: f32,
        height: f32,
    ) -> GameObject {
        GameObject {
            location: Rect::new(x, y, width, height),
            draw_system,
            children: vec![],
        }
    }

    pub fn update(&mut self, time_since_start: std::time::Duration) {
        if let Some(draw_system) = &mut self.draw_system {
            draw_system.update(time_since_start);
        }

        self.children
            .iter_mut()
            .for_each(|game_object| game_object.update(time_since_start));
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        if let Some(draw_system) = &self.draw_system {
            draw_system.draw(context, Point2::new(self.location.x, self.location.y))?;
        }

        for game_object in &self.children {
            game_object.draw(context)?;
        }

        return Ok(());
    }

    pub fn add_child(&mut self, game_object: GameObject) {
        self.children.push(game_object);
    }
}
