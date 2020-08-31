mod command;
mod draw_system;
mod game_object;
mod interface;
mod sprites;

use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, timer, Context, GameResult};
use interface::Interface;
use sprites::Sprite;
use std::sync::mpsc::{Receiver, Sender};
use twitch_chat_wrapper::chat_message::ChatMessage;

use draw_system::DrawSystem;
use game_object::GameObject;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    interface: Interface,
    framerate_target: u32,
    game_objects: Vec<GameObject>,
}

impl GameState {
    pub fn new(
        send_to_chat: Sender<String>,
        receive_from_chat: Receiver<ChatMessage>,
        context: &mut Context,
    ) -> GameResult<GameState> {
        let screen_size = graphics::drawable_size(context);
        let interface = Interface::new(context, screen_size)?;
        let fire_sprite = Sprite::new(context, "/LargeFlame.png", 4, 1)?;
        let framerate_target = 60;

        // create flame instruction
        let flame_draw_system = DrawSystem::new(Some(fire_sprite), Some("#fire-<column>"));
        let flame_size = flame_draw_system.get_size().unwrap_or((50.0, 50.0));
        let flame_game_object = GameObject::new(
            interface.location.x + interface.location.w / 2.0 - flame_size.0 / 2.0,
            150.0,
            Some(flame_draw_system),
            flame_size.0,
            flame_size.1,
        );

        let game_objects = vec![flame_game_object];

        Ok(GameState {
            send_to_chat,
            receive_from_chat,
            interface,
            framerate_target,
            game_objects,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, self.framerate_target) {
            if let Ok(chat_message) = self.receive_from_chat.try_recv() {
                dbg!(chat_message);
            }

            self.game_objects
                .iter_mut()
                .for_each(|game_object| game_object.update(timer::time_since_start(context)));
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        let screen_size = graphics::drawable_size(context);
        self.interface.draw(context, screen_size)?;

        for game_object in self.game_objects.iter() {
            game_object.draw(context)?;
        }

        graphics::present(context)
    }
}
