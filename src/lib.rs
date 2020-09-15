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
use life_system::{FireLifeSystem, LifeSystem, PlayerLifeSystem, SwordLifeSystem};
use physics::{FirePhysics, PhysicsSystem, PlayerPhysics, SwordPhysics, TimerPhysicsSystem};
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

        // create flame instruction
        let fire_sprite = Sprite::new(context, "/LargeFlame.png", 4, 1)?;
        let flame_draw_system =
            GameObjectDrawSystem::new(Some(fire_sprite), Some("#fire <column>"), 1.5);
        let flame_size = flame_draw_system.get_size().unwrap_or((50.0, 50.0));
        let flame_game_object = GameObject::new(
            interface.location.x + interface.location.w / 2.0 - flame_size.0 / 2.0,
            200.0,
            Some(Box::new(flame_draw_system)),
            flame_size.0,
            flame_size.1,
            None,
            false,
            None,
            GameObjectType::Interface,
            None,
        );

        // create sword instruction
        let sword_sprite = Sprite::new(context, "/item1BIT_sword.png", 1, 1)?;
        let sword_draw_system =
            GameObjectDrawSystem::new(Some(sword_sprite), Some("#sword <column>"), 2.5);
        let sword_size = sword_draw_system.get_size().unwrap_or((50.0, 50.0));
        let sword_game_object = GameObject::new(
            interface.location.x + interface.location.w / 2.0 - sword_size.0 / 2.0,
            350.0,
            Some(Box::new(sword_draw_system)),
            sword_size.0,
            sword_size.1,
            None,
            false,
            None,
            GameObjectType::Interface,
            None,
        );

        interface.add_game_object(sword_game_object);

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
            false,
            None,
            GameObjectType::Player,
            Some(Box::new(PlayerLifeSystem::new())),
        );

        let game_objects = vec![player];
        interface.add_game_object(flame_game_object);

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
            match command {
                Command::Fire {
                    id: column,
                    chatter,
                } => {
                    let scale = 2.0;
                    let drop_zone_location = self.interface.get_column_coordinates_by_index(column);
                    let fire_sprite = Sprite::new(context, "/LargeFlame.png", 4, 1)?;
                    let flame_draw_system =
                        GameObjectDrawSystem::new(Some(fire_sprite), None, scale);
                    let flame_size = flame_draw_system.get_size().unwrap_or((50.0, 50.0));
                    let physics_system = FirePhysics::new();
                    let flame_game_object = GameObject::new(
                        drop_zone_location.x - flame_size.0 / 2.0,
                        drop_zone_location.y - flame_size.1 / 2.0,
                        Some(Box::new(flame_draw_system)),
                        flame_size.0,
                        flame_size.1,
                        Some(Box::new(physics_system)),
                        true,
                        Some(chatter.clone()),
                        GameObjectType::Enemy,
                        Some(Box::new(FireLifeSystem::new())),
                    );

                    self.game_objects.push(flame_game_object);
                    if !self.teammates.contains(&chatter) {
                        self.teammates.push(chatter);
                    }
                }
                Command::Sword {
                    id: column,
                    chatter,
                } => {
                    let scale = 3.0;
                    let drop_zone_location = self.interface.get_column_coordinates_by_index(column);
                    let sword_sprite = Sprite::new(context, "/item1BIT_sword.png", 1, 1)?;
                    let sword_draw_system =
                        GameObjectDrawSystem::new(Some(sword_sprite), None, scale);
                    let sword_size = sword_draw_system.get_size().unwrap_or((50.0, 50.0));
                    let sword_physics = SwordPhysics::new();
                    let sword_game_object = GameObject::new(
                        drop_zone_location.x - sword_size.0 / 2.0,
                        drop_zone_location.y - sword_size.1 / 2.0,
                        Some(Box::new(sword_draw_system)),
                        sword_size.0,
                        sword_size.1,
                        Some(Box::new(sword_physics)),
                        true,
                        Some(chatter.clone()),
                        GameObjectType::Enemy,
                        Some(Box::new(SwordLifeSystem::new())),
                    );

                    self.game_objects.push(sword_game_object);
                    if !self.teammates.contains(&chatter) {
                        self.teammates.push(chatter);
                    }
                }
            }
        }

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
