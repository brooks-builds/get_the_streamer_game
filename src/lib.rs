mod chatter;
mod command;
mod credits;
mod draw_system;
mod game_object;
mod game_object_type;
mod interface;
mod life_system;
mod physics;
mod running_state;
mod splash;
mod sprites;
mod utilities;

use chatter::Chatter;
use command::Command;
use credits::Credits;
use draw_system::{DrawSystem, PlayerDrawSystem, TimerDrawSystem};
use game_object::GameObject;
use game_object_type::GameObjectType;
use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{graphics, timer, Context, GameResult};
use interface::Interface;
use life_system::{LifeSystem, PlayerLifeSystem};
use physics::{PhysicsSystem, PlayerPhysics, TimerPhysicsSystem};
use running_state::RunningState;
use splash::Splash;
use sprites::Sprite;
use std::time::Duration;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Instant,
};
use twitch_chat_wrapper::chat_message::ChatMessage;

pub const DROP_ZONE_COUNT: u8 = 10;
const GAME_TIME: Duration = Duration::from_secs(120);
const SPLASH_DURATION: Duration = Duration::from_secs(15);
const LIVES: u8 = 3;
const FRAMERATE_TARGET: u32 = 60;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    screen_size: (f32, f32),
    interface: Interface,
    game_objects: Vec<GameObject>,
    player_hit_object_event: Receiver<Chatter>,
    hitting_chatters: Vec<Chatter>,
    teammates: Vec<Chatter>,
    running_state: RunningState,
    credits: Option<Credits>,
    splash: Splash,
    game_start_time: Instant,
}

impl GameState {
    pub fn new(
        send_to_chat: Sender<String>,
        receive_from_chat: Receiver<ChatMessage>,
        screen_size: (f32, f32),
        context: &mut Context,
    ) -> GameResult<GameState> {
        send_to_chat.send("Chat vs. Streamer game started! Use the commands on screen to drop objects that the streamer will attempt to avoid".to_owned()).unwrap();
        let mut interface = Interface::new(context, screen_size, LIVES)?;

        // create timer block
        let timer_game_object = Self::create_timer(
            screen_size,
            context,
            interface.width,
            SPLASH_DURATION,
            (0.0, 1.0, 0.0),
        )?;
        interface.add_game_object(timer_game_object);

        // create player
        let player_scale = 4.0;
        let player_forward_sprite = Sprite::new(context, "/player_forward.png", 8, 1)?;
        let player_left_sprite = Sprite::new(context, "/player_left.png", 8, 1)?;
        let player_draw_system =
            PlayerDrawSystem::new(player_left_sprite, player_forward_sprite, player_scale);
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
        let splash = Splash::new(
            (screen_size.0 - interface.width, screen_size.1),
            context,
            SPLASH_DURATION,
        );
        let game_start_time = Instant::now();

        Ok(GameState {
            send_to_chat,
            receive_from_chat,
            screen_size,
            interface,
            game_objects,
            hitting_chatters: vec![],
            player_hit_object_event: receive_player_hit_object_event,
            teammates: vec![],
            running_state: RunningState::StartingSoon,
            credits: None,
            splash,
            game_start_time,
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
                if !self
                    .teammates
                    .iter()
                    .any(|teammate_chatter| teammate_chatter.name == chatter.name)
                {
                    self.teammates.push(chatter);
                }
            }
        }
        Ok(())
    }

    fn get_player(&self) -> Option<&GameObject> {
        self.game_objects
            .iter()
            .find(|game_object| game_object.my_type == GameObjectType::Player)
    }

    /// Remove chatters from teammates that also hit the streamer
    fn remove_hitting_teammates(&mut self) {
        let hitters = self.hitting_chatters.clone();
        self.teammates.retain(|teammate_chatter| {
            !hitters
                .iter()
                .any(|hitting_chatter| hitting_chatter.name == teammate_chatter.name)
        });
    }

    fn create_timer(
        screen_size: (f32, f32),
        context: &mut Context,
        interface_width: f32,
        duration: Duration,
        color: (f32, f32, f32),
    ) -> GameResult<GameObject> {
        let timer_draw_system = TimerDrawSystem::new(screen_size, context, color)?;
        let timer_size = timer_draw_system.get_size().unwrap_or((5.0, screen_size.1));
        let timer_physics_system =
            TimerPhysicsSystem::new(timer_size.1, duration, FRAMERATE_TARGET as f32);
        let timer_game_object = GameObject::new(
            screen_size.0 - interface_width,
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
        Ok(timer_game_object)
    }
}
impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, FRAMERATE_TARGET) {
            match self.running_state {
                RunningState::StartingSoon => {
                    if let Err(error) = self.interface.update(context, LIVES) {
                        eprintln!("Error updating game objects in interface: {}", error);
                    }
                    if self.splash.is_done() {
                        self.running_state = RunningState::Playing;
                        let timer = Self::create_timer(
                            self.screen_size,
                            context,
                            self.interface.width,
                            GAME_TIME,
                            (1.0, 0.0, 0.0),
                        )?;
                        self.interface.add_game_object(timer);
                        self.game_start_time = Instant::now();
                    }
                }
                RunningState::Playing => {
                    // get the player lives left
                    let lives_left = if let Some(player) = self.get_player() {
                        player.get_lives_left().unwrap_or(3)
                    } else {
                        0
                    };

                    if lives_left == 0 {
                        self.running_state = RunningState::ChatWon;
                    }

                    if let Err(error) = self.interface.update(context, lives_left) {
                        eprintln!("Error updating game objects in interface: {}", error);
                    }

                    let game_time_left =
                        GAME_TIME.as_secs() - self.game_start_time.elapsed().as_secs();
                    if game_time_left == 0 {
                        self.running_state = RunningState::PlayerWon;
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
                        self.screen_size.0 - self.interface.width,
                        self.screen_size.1,
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

                    #[cfg(debug_assertions)]
                    println!("game object count: {}", self.game_objects.len());

                    if let Ok(chatter) = self.player_hit_object_event.try_recv() {
                        let message_to_chat = format!("The streamer was hit by {}", &chatter.name);
                        self.send_to_chat.send(message_to_chat).unwrap();
                        if !self
                            .hitting_chatters
                            .iter()
                            .any(|hitting_chatter| chatter.name == hitting_chatter.name)
                        {
                            self.hitting_chatters.push(chatter);
                        }
                    }

                    if self
                        .game_objects
                        .iter()
                        .find(|game_object| game_object.my_type == GameObjectType::Player)
                        .is_none()
                    {
                        self.running_state = RunningState::ChatWon;
                    }
                }
                RunningState::ChatWon | RunningState::PlayerWon => {
                    if let Some(credits) = &mut self.credits {
                        if !credits.update() {
                            ggez::event::quit(context);
                        }
                    } else {
                        self.remove_hitting_teammates();
                        self.credits = Some(Credits::new(
                            self.running_state,
                            context,
                            self.screen_size,
                            &self.hitting_chatters,
                            &self.teammates,
                        )?);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.interface
            .draw(context, self.screen_size, &self.running_state)?;

        match self.running_state {
            RunningState::StartingSoon => self.splash.draw(context)?,
            RunningState::Playing => {
                for game_object in self.game_objects.iter() {
                    game_object.draw(context)?;
                }
            }
            RunningState::PlayerWon | RunningState::ChatWon => {
                if let Some(credits) = &self.credits {
                    credits.draw(context)?;
                }
            }
        }

        graphics::present(context)
    }
}
