use super::utilities;
use crate::{chatter::Chatter, running_state::RunningState};
use ggez::{
    graphics::DrawParam,
    graphics::Font,
    graphics::Scale,
    graphics::{self, Text},
    nalgebra::Point2,
    Context, GameResult,
};
use rand::prelude::*;
use rand::seq::IteratorRandom;
use std::{fs::File, path::Path};

const VELOCITY_Y: f32 = -5.0;

pub struct Credits {
    all_credits: Vec<(Text, Point2<f32>)>,
}

impl Credits {
    pub fn new(
        running_state: RunningState,
        context: &mut Context,
        screen_size: (f32, f32),
        hitting_chatters: &Vec<Chatter>,
    ) -> GameResult<Self> {
        let mut rng = thread_rng();
        let streamer_won_message =
            if let Some(messages) = utilities::load_messages("streamer_wins_messages.txt") {
                messages
                    .choose(&mut rng)
                    .unwrap_or("Streamer won!".to_owned())
            } else {
                "Streamer won!".to_owned()
            };

        let mut all_credits = vec![];
        let did_chat_win = matches!(running_state, RunningState::ChatWon);
        all_credits.push(Self::create_game_over_text(context, screen_size));
        if !did_chat_win {
            Self::create_credit(
                context,
                screen_size,
                &streamer_won_message,
                Some(100.0),
                0,
                &mut all_credits,
            );
        }

        Ok(Credits { all_credits })
    }

    fn create_credit(
        context: &mut Context,
        (screen_width, screen_height): (f32, f32),
        title: &str,
        font_scale: Option<f32>,
        credit_number: u32,
        all_credits: &mut Vec<(Text, Point2<f32>)>,
    ) {
        let mut text = Text::new(title);
        let scale = font_scale.unwrap_or(50.0);
        text.set_font(Font::default(), Scale::uniform(scale));
        let text_size = text.dimensions(context);
        let location = Point2::new(
            screen_width / 2.0 - text_size.0 as f32 / 2.0,
            screen_height + 50_f32 * credit_number as f32,
        );
        all_credits.push((text, location));
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

    pub fn update(&mut self) {
        for credit in &mut self.all_credits {
            credit.1.y += VELOCITY_Y;
        }

        self.all_credits.retain(|credit| credit.1.y > -100.0);
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        self.all_credits.iter().try_for_each(|credit| {
            graphics::draw(context, &credit.0, DrawParam::new().dest(credit.1))
        })
    }
}
