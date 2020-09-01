mod command;
mod draw_system;
mod game_object;
mod interface;
mod physics_system;
mod sprites;

use command::Command;
use draw_system::DrawSystem;
use game_object::GameObject;
use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, timer, Context, GameResult};
use interface::Interface;
use physics_system::PhysicsSystem;
use sprites::Sprite;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use twitch_chat_wrapper::chat_message::ChatMessage;

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
        let flame_draw_system = DrawSystem::new(Some(fire_sprite), Some("#fire-<column>"), 1.5);
        let flame_size = flame_draw_system.get_size().unwrap_or((50.0, 50.0));
        let flame_game_object = GameObject::new(
            interface.location.x + interface.location.w / 2.0 - flame_size.0 / 2.0,
            150.0,
            Some(flame_draw_system),
            flame_size.0,
            flame_size.1,
            None,
            None,
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

    fn handle_command(
        &mut self,
        command: Option<Command>,
        context: &mut Context,
    ) -> GameResult<()> {
        if let Some(command) = command {
            match command {
                Command::Fire(column) => {
                    let drop_zone_location = self.interface.get_column_coordinates_by_index(column);
                    let fire_sprite = Sprite::new(context, "/LargeFlame.png", 4, 1)?;
                    let flame_draw_system = DrawSystem::new(Some(fire_sprite), None, 2.0);
                    let flame_size = flame_draw_system.get_size().unwrap_or((50.0, 50.0));
                    let physics_system = PhysicsSystem::new(true);
                    let flame_game_object = GameObject::new(
                        drop_zone_location.x - flame_size.0 / 2.0,
                        drop_zone_location.y - flame_size.1 / 2.0,
                        Some(flame_draw_system),
                        flame_size.0,
                        flame_size.1,
                        Some(physics_system),
                        Some(Duration::from_secs(6)),
                    );

                    self.game_objects.push(flame_game_object);
                }
            }
        }

        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, self.framerate_target) {
            if let Ok(chat_message) = self.receive_from_chat.try_recv() {
                match Command::new(&chat_message.message) {
                    Err(error) => self.send_to_chat.send(error.to_owned()).unwrap(),
                    Ok(command) => self.handle_command(command, context)?,
                }
            }

            let (_screen_width, screen_height) = graphics::drawable_size(context);

            self.game_objects.iter_mut().for_each(|game_object| {
                game_object.update(timer::time_since_start(context), screen_height)
            });

            self.game_objects
                .retain(|game_object| game_object.is_alive());
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
