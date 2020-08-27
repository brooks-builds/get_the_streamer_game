mod interface;
mod sprites;

use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, timer, Context, GameResult};
use interface::Interface;
use sprites::Sprite;
use std::sync::mpsc::{Receiver, Sender};
use twitch_chat_wrapper::chat_message::ChatMessage;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    interface: Interface,
    fire_sprite: Sprite,
    framerate_target: u32,
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

        Ok(GameState {
            send_to_chat,
            receive_from_chat,
            interface,
            fire_sprite,
            framerate_target,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, self.framerate_target) {
            if let Ok(chat_message) = self.receive_from_chat.try_recv() {
                dbg!(chat_message);
            }

            self.fire_sprite.update(timer::time_since_start(context));
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        let screen_size = graphics::drawable_size(context);
        self.interface.draw(context, screen_size)?;

        self.fire_sprite.draw(context)?;

        graphics::present(context)
    }
}
