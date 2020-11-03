use ggez::{
    graphics::{self, DrawParam, Font, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Splash {
    text: Text,
    text_width: f32,
}

impl Splash {
    pub fn new(context: &mut Context, max_width:f32, max_height:f32) -> Self {
        let display_string = "Starting Soon";
        let mut text = Text::new(display_string);
        let scale: f32 = max_height.min(max_width/display_string.len() as f32).floor();
        text.set_font(Font::default(), Scale::uniform(scale));
        let text_width = text.dimensions(context).0 as f32;

        Self { text, text_width }
    }

    pub fn draw(&self, context: &mut Context, x: f32, y: f32) -> GameResult<()> {
        
        graphics::draw(
            context,
            &self.text,
            DrawParam::new().dest(Point2::new(
                x - self.text_width * 0.5,
                y,
            )),
        )
    }
}
