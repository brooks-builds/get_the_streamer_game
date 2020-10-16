use std::time::{Duration, Instant};

use ggez::{
    graphics::{self, DrawParam, Font, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Splash {
    text: Text,
    location: Point2<f32>,
    finished_at: Instant,
}

impl Splash {
    pub fn new(arena_size: (f32, f32), context: &mut Context, duration: Duration) -> Self {
        let mut text = Text::new("Starting Soon");
        text.set_font(Font::default(), Scale::uniform(100.0));
        let text_size = text.dimensions(context);
        let location = Point2::new(
            arena_size.0 / 2.0 - text_size.0 as f32 / 2.0,
            arena_size.1 / 2.0 - text_size.1 as f32 / 2.0,
        );
        let finished_at = Instant::now() + duration;

        Self {
            text,
            location,
            finished_at,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(context, &self.text, DrawParam::new().dest(self.location))
    }

    pub fn is_done(&self) -> bool {
        Instant::now() >= self.finished_at
    }
}
