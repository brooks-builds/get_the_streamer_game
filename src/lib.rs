use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, Context, GameResult};
use std::sync::mpsc::{Receiver, Sender};
use twitch_chat_wrapper::chat_message::ChatMessage;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
}

impl GameState {
    pub fn new(
        send_to_chat: Sender<String>,
        receive_from_chat: Receiver<ChatMessage>,
    ) -> GameState {
        GameState {
            send_to_chat,
            receive_from_chat,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        if let Ok(chat_message) = self.receive_from_chat.try_recv() {
            dbg!(chat_message);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        graphics::present(context)
    }
}
