use ggez::graphics::{DrawParam, Font, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Interface {
    title: Text,
}

impl Interface {
    pub fn new() -> Interface {
        let mut title = Text::new("Get the Streamer!");

        title.set_font(Font::default(), Scale::uniform(50.0));

        Interface { title }
    }
    pub fn draw(&self, context: &mut Context, screen_size: (f32, f32)) -> GameResult<()> {
        self.draw_title(context, screen_size)?;

        Ok(())
    }

    pub fn draw_title(
        &self,
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
    ) -> GameResult<()> {
        let (title_width, title_height) = self.title.dimensions(context);

        graphics::draw(
            context,
            &self.title,
            DrawParam::new().dest(Point2::new(screen_width - title_width as f32 - 10.0, 10.0)),
        )
    }
}
