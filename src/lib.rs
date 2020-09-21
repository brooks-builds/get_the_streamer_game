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
const MAX_IFRAMES: u8 = 120;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    interface: Interface,
    framerate_target: u32,
    game_objects: Vec<GameObject>,
    player_hit_object_event: Receiver<Chatter>,
    winning_player: Option<Chatter>,
    teammates: Vec<Chatter>,
    lives_left: u8,
    damage_cooldown: u8,
}

impl GameState {
    pub fn new(
        send_to_chat: Sender<String>,
        receive_from_chat: Receiver<ChatMessage>,
        context: &mut Context,
    ) -> GameResult<GameState> {
        send_to_chat.send("Chat vs. Streamer game started! Use the commands on screen to drop objects that the streamer will attempt to avoid".to_owned()).unwrap();
        let screen_size = graphics::drawable_size(context);
        let mut interface = Interface::new(context, screen_size)?;
        let framerate_target = 60;

        Self::create_instruction(
            &mut interface,
            context,
            "/LargeFlame.png",
            (4, 1),
            "#fire <column>",
            1.5,
            200.0,
        )?;

        Self::create_instruction(
            &mut interface,
            context,
            "/item1BIT_sword.png",
            (1, 1),
            "#sword <column>",
            2.5,
            300.0,
        )?;

        Self::create_instruction(
            &mut interface,
            context,
            "/snake.png",
            (4, 1),
            "#snake <column>",
            2.5,
            400.0,
        )?;

        // create timer block
        let timer_draw_system = TimerDrawSystem::new(screen_size, context)?;
        let timer_size = timer_draw_system.get_size().unwrap_or((5.0, screen_size.1));
        let timer_physics_system =
            TimerPhysicsSystem::new(timer_size.1, GAME_TIME, framerate_target as f32);
        let timer_game_object = GameObject::new(
            screen_size.0 - interface.instruction_width,
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
            lives_left: 3,
            damage_cooldown: 0,
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

    fn create_instruction(
        interface: &mut Interface,
        context: &mut Context,
        sprite_path: &'static str,
        sprite_count: (u16, u16),
        label: &'static str,
        scale_by: f32,
        y: f32,
    ) -> GameResult<()> {
        let sprite = Sprite::new(context, sprite_path, sprite_count.0, sprite_count.1)?;
        let draw_system = GameObjectDrawSystem::new(Some(sprite), Some(label), scale_by);
        let size = draw_system.get_size().unwrap_or((50.0, 50.0));
        let game_object = GameObject::new(
            interface.location.x + interface.location.w / 2.0 - size.0 / 2.0,
            y,
            Some(Box::new(draw_system)),
            size.0,
            size.1,
            None,
            false,
            None,
            GameObjectType::Interface,
            None,
        );

        interface.add_game_object(game_object);

        Ok(())
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

            let arena_size = (
                screen_size.0 - self.interface.instruction_width,
                screen_size.1,
            );

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

            if let Ok(chatter_name) = self.player_hit_object_event.try_recv() {
                // we got hit, so lose a life
                if self.damage_cooldown == 0 {
                    self.lives_left -= 1;
                    self.damage_cooldown = MAX_IFRAMES;
                    let message_to_chat = format!(
                        "The streamer was hit by {}. The player has {} lives left",
                        &chatter_name.name, &self.lives_left
                    );
                    self.send_to_chat.send(message_to_chat).unwrap();

                    if self.lives_left == 0 {
                        self.winning_player = Some(chatter_name);
                    }
                }
            }

            if let Err(error) = self.interface.update(context) {
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

        graphics::present(context)
    }
}
