use ggez::graphics::{Color, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

const DROP_ZONE_COUNT: u8 = 10;
const DROP_ZONE_HEIGHT: f32 = 50.0;

pub struct Interface {
    title: Text,
    instruction_background: Mesh,
    margin: f32,
    instruction_width: f32,
    commands: Vec<Text>,
    command_height: usize,
    command_start_at: f32,
    pub location: Rect,
    drop_zones: Vec<Rect>,
    drop_zone_background: Mesh,
    drop_zone_labels: Vec<Text>,
    single_drop_zone_width: f32,
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

        let mut drop_zones = vec![];
        let drop_zone_width = screen_width - instruction_width;
        let single_drop_zone_width = drop_zone_width / DROP_ZONE_COUNT as f32;
        let mut drop_zone_labels = vec![];

        for count in 0..DROP_ZONE_COUNT {
            let drop_zone = Rect::new(
                count as f32 * single_drop_zone_width,
                0.0,
                single_drop_zone_width,
                DROP_ZONE_HEIGHT,
            );
            drop_zones.push(drop_zone.clone());

            let mut label = Text::new(format!("{}", count));

            label.set_bounds(
                Point2::new(single_drop_zone_width, DROP_ZONE_HEIGHT),
                graphics::Align::Center,
            );
            label.set_font(Font::default(), Scale::uniform(50.0));
            drop_zone_labels.push(label);
        }

        let drop_zone_background = MeshBuilder::new()
            .rectangle(DrawMode::stroke(1.0), drop_zones[0], graphics::WHITE)
            .build(context)?;

        Ok(Interface {
            title,
            instruction_background,
            margin,
            instruction_width,
            commands,
            command_height,
            command_start_at,
            location,
            drop_zones,
            drop_zone_background,
            drop_zone_labels,
            single_drop_zone_width,
        })
    }
    pub fn draw(&self, context: &mut Context, screen_size: (f32, f32)) -> GameResult<()> {
        self.draw_background(context)?;
        self.draw_title(context, screen_size)?;
        self.draw_commands(context, screen_size)?;
        self.draw_drop_zones(context)?;

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

    fn draw_drop_zones(&self, context: &mut Context) -> GameResult<()> {
        self.drop_zones
            .iter()
            .try_for_each(|drop_zone: &Rect| -> GameResult<()> {
                graphics::draw(
                    context,
                    &self.drop_zone_background,
                    DrawParam::new().dest(Point2::new(drop_zone.x, drop_zone.y)),
                )?;
                Ok(())
            })?;

        self.drop_zone_labels.iter().enumerate().try_for_each(
            |(index, label)| -> GameResult<()> {
                let label_height = label.height(context) as f32;
                graphics::draw(
                    context,
                    label,
                    DrawParam::new().dest(Point2::new(
                        index as f32 * self.single_drop_zone_width,
                        DROP_ZONE_HEIGHT / 2.0 - label_height / 2.0,
                    )),
                )
            },
        )?;

        Ok(())
    }

    /// Take in an index like 3
    /// which should return the middle x,y coordinates of the corresponding drop zone
    pub fn get_column_coordinates_by_index(&self, index: u8) -> Point2<f32> {
        Point2::new(
            index as f32 * self.single_drop_zone_width + self.single_drop_zone_width / 2.0,
            DROP_ZONE_HEIGHT / 2.0,
        )
    }
}
