use ggez::graphics::{Color, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Interface {
    title: Text,
    instruction_background: Mesh,
    margin: f32,
    instruction_width: f32,
    commands: Vec<Text>,
    command_height: usize,
    command_start_at: f32,
    pub location: Rect,
}

impl Interface {
    pub fn new(
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
    ) -> GameResult<Interface> {
        let mut title = Text::new("Get the Streamer!");

        title.set_font(Font::default(), Scale::uniform(50.0));
        let (title_width, title_height) = title.dimensions(context);
        let margin = 10.0;
        let instruction_width = title_width as f32 + margin * 2.0;
        let location = Rect::new(
            screen_width - instruction_width,
            0.0,
            instruction_width,
            screen_height,
        );

        let instruction_background = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                location,
                Color::from_rgb(0x08, 0x20, 0x4e),
            )
            .build(context)?;

        let commands = vec![];
        let command_height = 150;
        let command_start_at = title_height as f32 + margin * 4.0;

        Ok(Interface {
            title,
            instruction_background,
            margin,
            instruction_width,
            commands,
            command_height,
            command_start_at,
            location,
        })
    }
    pub fn draw(&self, context: &mut Context, screen_size: (f32, f32)) -> GameResult<()> {
        self.draw_background(context)?;
        self.draw_title(context, screen_size)?;
        self.draw_commands(context, screen_size)?;

        Ok(())
    }

    fn draw_title(
        &self,
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
    ) -> GameResult<()> {
        let (title_width, title_height) = self.title.dimensions(context);

        graphics::draw(
            context,
            &self.title,
            DrawParam::new().dest(Point2::new(
                screen_width - title_width as f32 - self.margin,
                self.margin,
            )),
        )
    }

    fn draw_background(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(context, &self.instruction_background, DrawParam::new())
    }

    fn draw_commands(
        &self,
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
    ) -> GameResult<()> {
        self.commands
            .iter()
            .enumerate()
            .try_for_each(|(index, command_text)| {
                let command_text_size = command_text.dimensions(context);
                graphics::draw(
                    context,
                    command_text,
                    DrawParam::new().dest(Point2::new(
                        screen_width
                            - (command_text_size.0 / 2) as f32
                            - self.margin
                            - self.instruction_width / 2.0,
                        (index * self.command_height) as f32 + self.command_start_at,
                    )),
                )
            })
    }
}
