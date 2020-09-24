use super::{Chatter, GameObject};
use eyre::Result;
use ggez::graphics::{
    Align, Color, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text,
};
use ggez::nalgebra::Point2;
use ggez::{graphics, timer, Context, GameResult};
use graphics::Image;

const DROP_ZONE_COUNT: u8 = 10;
const DROP_ZONE_HEIGHT: f32 = 50.0;
const GAME_OVER_FONT_SIZE: f32 = 150.0;

pub struct Interface {
    pub width: f32,
    drop_zones: Vec<Rect>,
    drop_zone_background: Mesh,
    drop_zone_labels: Vec<Text>,
    single_drop_zone_width: f32,
    game_objects: Vec<GameObject>,
    instruction_image: Image,
    heart_image: Image,
    player_lives_left: u8,
}

impl Interface {
    pub fn new(
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
        player_lives_left: u8,
    ) -> GameResult<Interface> {
        let instruction_image = Image::new(context, "/sidebar.png")?;
        let width = instruction_image.width().into();
        let margin = 100.0;
        let mut drop_zones = vec![];
        let drop_zone_width = screen_width - width;
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

        let mut game_over_text = Text::new("Game Over!");
        game_over_text.set_font(Font::default(), Scale::uniform(GAME_OVER_FONT_SIZE));
        game_over_text.set_bounds(Point2::new(screen_width, screen_height), Align::Center);

        let dark_mask = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, screen_width, screen_height),
                Color::new(0.0, 0.0, 0.0, 0.9),
            )
            .build(context)?;

        let mut attack_subtitle = Text::new("Attack the Streamer with following commands:");
        attack_subtitle.set_font(Font::default(), Scale::uniform(30.0));
        attack_subtitle.set_bounds(Point2::new(width, 60.0), Align::Center);

        let mut help_subtitle = Text::new("Help the Streamer with following commands:");
        help_subtitle.set_font(Font::default(), Scale::uniform(30.0));
        help_subtitle.set_bounds(Point2::new(width, 60.0), Align::Center);

        let mut player_lives_left_subtitle = Text::new("Streamer Lives Left:");
        player_lives_left_subtitle.set_font(Font::default(), Scale::uniform(30.0));
        player_lives_left_subtitle.set_bounds(Point2::new(width, 60.0), Align::Center);

        let heart_image = Image::new(context, "/heart.png")?;

        Ok(Interface {
            width,
            drop_zones,
            drop_zone_background,
            drop_zone_labels,
            single_drop_zone_width,
            game_objects: vec![],
            instruction_image,
            heart_image,
            player_lives_left,
        })
    }
    pub fn draw(
        &mut self,
        context: &mut Context,
        screen_size: (f32, f32),
        winning_player: Option<&Chatter>,
        teammates: &Vec<Chatter>,
    ) -> GameResult<()> {
        self.draw_drop_zones(context)?;

        graphics::draw(
            context,
            &self.instruction_image,
            DrawParam::new().dest(Point2::new(screen_size.0 - self.width, 0.0)),
        )?;

        let mut heart_x = screen_size.0
            - (self.width / 2.0)
            - ((self.heart_image.width() as f32) * self.player_lives_left as f32) / 2.0;
        for index in 0..self.player_lives_left {
            graphics::draw(
                context,
                &self.heart_image,
                DrawParam::new().dest(Point2::new(heart_x, 1015.0)),
            )?;

            heart_x += self.heart_image.width() as f32 + 5.0;
        }

        Ok(())
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

    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }

    pub fn update(&mut self, context: &mut Context, player_lives_left: u8) -> Result<()> {
        let time_since_start = timer::time_since_start(context);
        let screen_size = graphics::drawable_size(context);
        let collidable_game_objects = vec![];
        self.player_lives_left = player_lives_left;

        self.game_objects.iter_mut().try_for_each(|game_object| {
            game_object.update(
                time_since_start,
                screen_size,
                context,
                &collidable_game_objects,
            )
        })
    }

    fn draw_player_lives_left(
        &self,
        context: &mut Context,
        screen_size: (f32, f32),
    ) -> GameResult<()> {
        Ok(())
    }
}
