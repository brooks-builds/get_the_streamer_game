mod chatter;
mod command;
mod draw_system;
mod game_object;
mod game_object_type;
mod interface;
mod life_system;
mod physics;
mod sprites;

use chatter::Chatter;
use command::Command;
use draw_system::{DrawSystem, GameObjectDrawSystem, TimerDrawSystem};
use game_object::GameObject;
use game_object_type::GameObjectType;
use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, timer, Context, GameResult};
use interface::Interface;
use life_system::{LifeSystem, PlayerLifeSystem};
use physics::{PhysicsSystem, PlayerPhysics, TimerPhysicsSystem};
use sprites::Sprite;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use twitch_chat_wrapper::chat_message::ChatMessage;

const GAME_TIME: Duration = Duration::from_secs(120);
const LIVES: u8 = 3;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    interface: Interface,
    framerate_target: u32,
    game_objects: Vec<GameObject>,
    player_hit_object_event: Receiver<Chatter>,
    winning_player: Option<Chatter>,
    teammates: Vec<Chatter>,
    damage_cooldown: u8,
    lives_left: u8,
}

impl GameState {
    pub fn new(
        send_to_chat: Sender<String>,
        receive_from_chat: Receiver<ChatMessage>,
        context: &mut Context,
    ) -> GameResult<GameState> {
        send_to_chat.send("Chat vs. Streamer game started! Use the commands on screen to drop objects that the streamer will attempt to avoid".to_owned()).unwrap();
        let screen_size = graphics::drawable_size(context);
        let mut interface = Interface::new(context, screen_size, LIVES)?;
        let framerate_target = 60;

        // create timer block
        let timer_draw_system = TimerDrawSystem::new(screen_size, context)?;
        let timer_size = timer_draw_system.get_size().unwrap_or((5.0, screen_size.1));
        let timer_physics_system =
            TimerPhysicsSystem::new(timer_size.1, GAME_TIME, framerate_target as f32);
        let timer_game_object = GameObject::new(
            screen_size.0 - interface.width,
            0.0,
            Some(Box::new(timer_draw_system)),
            timer_size.0,
            timer_size.1,
            Some(Box::new(timer_physics_system)),
            false,
            None,
            GameObjectType::Interface,
            None,
        );
        interface.add_game_object(timer_game_object);

        // create player
        let player_scale = 4.0;
        let player_sprite = Sprite::new(context, "/PlayerCharacter.png", 24, 1)?;
        let player_draw_system = GameObjectDrawSystem::new(Some(player_sprite), None, player_scale);
        let player_size = player_draw_system.get_size().unwrap_or((50.0, 50.0));
        let (send_player_hit_object_event, receive_player_hit_object_event) = channel();
        let player_physics_system = PlayerPhysics::new(send_player_hit_object_event);
        let player = GameObject::new(
            250.0,
            250.0,
            Some(Box::new(player_draw_system)),
            player_size.0,
            player_size.1,
            Some(Box::new(player_physics_system)),
            true,
            None,
            GameObjectType::Player,
            Some(Box::new(PlayerLifeSystem::new())),
        );

        let game_objects = vec![player];

        Ok(GameState {
            send_to_chat,
            receive_from_chat,
            interface,
            framerate_target,
            game_objects,
            winning_player: None,
            player_hit_object_event: receive_player_hit_object_event,
            teammates: vec![],
            damage_cooldown: 0,
            lives_left: LIVES,
        })
    }

    fn handle_command(
        &mut self,
        command: Option<Command>,
        context: &mut Context,
    ) -> GameResult<()> {
        if let Some(command) = command {
            let chatter = command.chatter.clone();
            self.game_objects.push(command.handle(
                self.interface.get_column_coordinates_by_index(command.id),
                context,
            )?);
            if !self.teammates.contains(&chatter) {
                self.teammates.push(chatter);
            }
        }
        Ok(())
    }

    fn get_player(&self) -> Option<&GameObject> {
        self.game_objects
            .iter()
            .find(|game_object| game_object.my_type == GameObjectType::Player)
    }
}
impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, self.framerate_target) {
            if let Some(_) = self.winning_player {
                return Ok(());
            }
            let screen_size = graphics::drawable_size(context);

            let game_time_left = (GAME_TIME - timer::time_since_start(context)).as_secs();
            if game_time_left == 0 {
                self.winning_player =
                    Some(Chatter::new("The Streamer".to_owned(), (200, 150, 230)));
                self.teammates = self
                    .teammates
                    .clone()
                    .into_iter()
                    .map(|mut chatter| {
                        chatter.name = format!("not {}", chatter.name);
                        chatter
                    })
                    .collect();
                return Ok(());
            }

            if let Ok(chat_message) = self.receive_from_chat.try_recv() {
                let chatter_name = if let Some(display_name) = chat_message.display_name {
                    display_name.clone()
                } else {
                    chat_message.name.clone()
                };
                match Command::new(
                    &chat_message.message,
                    Chatter::new(chatter_name, chat_message.color_rgb),
                ) {
                    Err(error) => self.send_to_chat.send(error.to_owned()).unwrap(),
                    Ok(command) => self.handle_command(command, context)?,
                }
            }

            let arena_size = (screen_size.0 - self.interface.width, screen_size.1);

            let collidable_game_objects: Vec<GameObject> = self
                .game_objects
                .clone()
                .into_iter()
                .filter(|game_object| game_object.collidable)
                .collect();

            self.game_objects.iter_mut().for_each(|game_object| {
                if let Err(error) = game_object.update(
                    timer::time_since_start(context),
                    arena_size,
                    context,
                    &collidable_game_objects,
                ) {
                    eprintln!("error running update: {}", error)
                }
            });

            self.game_objects
                .retain(|game_object| game_object.is_alive());

            #[cfg(debug_assertions)]
            println!("game object count: {}", self.game_objects.len());

            if let Ok(chatter_name) = self.player_hit_object_event.try_recv() {
                let message_to_chat = format!("The streamer was hit by {}", &chatter_name.name);
                self.send_to_chat.send(message_to_chat).unwrap();
            }

            if self
                .game_objects
                .iter()
                .find(|game_object| game_object.my_type == GameObjectType::Player)
                .is_none()
            {
                self.winning_player = Some(Chatter::new("Someone".to_owned(), (50, 25, 200)));
            }

            // get the player lives left
            let lives_left = if let Some(player) = self.get_player() {
                player.get_lives_left().unwrap_or(3)
            } else {
                3
            };
            if let Err(error) = self.interface.update(context, lives_left) {
                eprintln!("Error updating game objects in interface: {}", error);
            }

            if self.damage_cooldown > 0 {
                self.damage_cooldown -= 1;
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        let screen_size = graphics::drawable_size(context);

        for game_object in self.game_objects.iter() {
            let iframes_active =
                GameObjectType::Player == game_object.my_type && self.damage_cooldown > 0;
            game_object.draw(context, iframes_active)?;
        }

        self.interface.draw(
            context,
            screen_size,
            self.winning_player.as_ref(),
            &self.teammates,
        )?;

        let fps = ggez::timer::fps(context);
        println!("fps: {}", fps);

        graphics::present(context)
    }
}
