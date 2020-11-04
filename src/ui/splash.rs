use ggez::{
    graphics::{self, DrawParam, Font, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};

use super::UIComponent;

pub struct Splash {
    width: f32,
    height: f32,
    text: Text,
}

impl Splash {
    pub fn new(context: &mut Context, max_width: f32, max_height: f32) -> Self {
        let display_string = "Starting Soon";
        let mut text = Text::new(display_string);
        let scale: f32 = max_height
            .min(max_width / display_string.len() as f32)
            .floor();
        text.set_font(Font::default(), Scale::uniform(scale));
        let dimensions = text.dimensions(context);

        Self {
            width: dimensions.0 as f32,
            height: dimensions.1 as f32,
            text,
        }
    }
}

impl UIComponent for Splash {
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }
    fn draw(&self, context: &mut Context, x: f32, y: f32) -> GameResult {
        graphics::draw(
            context,
            &self.text,
            DrawParam::new().dest(Point2::new(x - self.width * 0.5, y)),
        )
    }
}
