mod chat_test_mock;
mod chatter;
mod command;
mod credits;
mod draw_system;
mod game_assets;
mod game_object;
mod game_object_type;
mod life_system;
mod physics;
mod running_state;
mod sprites;
mod utilities;
mod gamestate;
mod ui;

use chatter::Chatter;
use draw_system::{DrawSystem};
use game_assets::GameAssets;
use game_object::GameObject;
use game_object_type::GameObjectType;
use gamestate::GameState;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::graphics::Image;
use ggez::{event, Context, ContextBuilder};
use std::sync::Mutex;

use lazy_static::lazy_static;
use life_system::LifeSystem;
use physics::PhysicsSystem;
use sprites::Sprite;

const DROP_ZONE_COUNT: u8 = 10;

lazy_static! {
    static ref GAME_ASSETS: Mutex<GameAssets> = Mutex::new(GameAssets::new());
}

pub fn get_image_from_assets(context: &mut Context, path: String) -> Image {
    return GAME_ASSETS.lock().unwrap().get_image(context, path);
}

const WINDOW_SIZE: (f32, f32) = (1920.0, 1080.0);

pub struct RunConfig {
    pub test_bot_chatters: u32,
    pub test_command_occurences: &'static [(&'static str, u32)],
    pub attach_to_twitch_channel: bool,
}

impl Default for RunConfig {
    fn default() -> Self {
        RunConfig {
            test_bot_chatters:0,
            test_command_occurences: &[],
            attach_to_twitch_channel: true
            // test_bot_chatters: 5,
            // test_command_occurences: &[("fire", 1), ("sword", 1), ("snake", 1), ("heart", 1)],
            // attach_to_twitch_channel: false,
        }
    }
}

pub fn run_game(run_config: Option<RunConfig>) {
        
    let (context, event_loop) = &mut match ContextBuilder::new("Get the Streamer", "Brooks Builds")
        .window_setup(WindowSetup::default().title("Get the Streamer"))
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1).resizable(true))
        .build()
    {
        Ok((context, event_loop)) => (context, event_loop),
        Err(error) => panic!(error),
    };

    let game_state =
        &mut GameState::new(run_config, WINDOW_SIZE, context).unwrap();
    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
