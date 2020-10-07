use ggez::{
    graphics::DrawParam,
    graphics::Font,
    graphics::Scale,
    graphics::{self, Text},
    nalgebra::Point2,
    Context, GameResult,
};

use crate::{chatter::Chatter, running_state::RunningState};

pub struct Credits {
    velocity_y: f32,
    texts: Vec<(Text, Point2<f32>)>,
}

impl Credits {
    pub fn new(
        running_state: RunningState,
        context: &mut Context,
        hitting_chatters: &Vec<Chatter>,
    ) -> GameResult<Self> {
        // todo - rewrite create functions to be one function, and then add in all the chatters
        let mut texts = vec![];
        let screen_size = graphics::drawable_size(context);
        let did_chat_win = matches!(running_state, RunningState::ChatWon);
        texts.push(Self::create_game_over_text(context, screen_size));
        if !did_chat_win {
            texts.push(Self::create_streamer_won_text(context, screen_size));
        }

        Ok(Credits {
            velocity_y: -1.0,
            texts,
        })
    }

    fn create_game_over_text(
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
    ) -> (Text, Point2<f32>) {
        let mut game_over_text = Text::new("Game Over");
        game_over_text.set_font(Font::default(), Scale::uniform(50.0));
        let (game_over_width, game_over_height) = game_over_text.dimensions(context);
        let game_over_location = Point2::new(
            screen_width / 2.0 - game_over_width as f32 / 2.0,
            screen_height / 2.0 - game_over_height as f32 / 2.0,
        );
        (game_over_text, game_over_location)
    }

    fn create_streamer_won_text(
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
    ) -> (Text, Point2<f32>) {
        let mut text = Text::new("Too bad you all couldn't hit me!");
        text.set_font(Font::default(), Scale::uniform(75.0));
        let (game_over_width, game_over_height) = text.dimensions(context);
        let location = Point2::new(
            screen_width / 2.0 - game_over_width as f32 / 2.0,
            screen_height,
        );
        (text, location)
    }

    pub fn update(&mut self) {
        for credit in &mut self.texts {
            credit.1.y += self.velocity_y;
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        self.texts.iter().try_for_each(|credit| {
            graphics::draw(context, &credit.0, DrawParam::new().dest(credit.1))
        })
    }
}
