use crate::{
    chat_test_mock,
    chatter::Chatter,
    command,
    command::CommandInstance,
    command::CommandParser,
    credits::Credits,
    draw_system::{DrawSystem, PlayerDrawSystem},
    game_object::GameObject,
    game_object_type::GameObjectType,
    game_world::GameWorld,
    life_system::PlayerLifeSystem,
    physics::PlayerPhysics,
    running_state::RunningState,
    sprites::Sprite,
    ui::gamewindow::GamePlayWindow,
    ui::Interface,
    utilities, RunConfig,
};

use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{audio, audio::SoundSource, graphics, timer, Context, GameResult};
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
    time::Instant,
};
use twitch_chat_wrapper::chat_message::ChatMessage;

const LIVES: u8 = 3;
pub const FRAMERATE_TARGET: u32 = 60;
const GAME_TIME: Duration = Duration::from_secs(120);
const SPLASH_DURATION: Duration = Duration::from_secs(15);
const SCORES_FILE_NAME: &str = "/high_scores";
const DROP_ZONE_HEIGHT: f32 = 50.0;

pub struct GameState {
    send_to_chat: Sender<String>,
    receive_from_chat: Receiver<ChatMessage>,
    screen_size: (f32, f32),
    interface: Interface,
    gameworld: GameWorld,
    player_hit_object_event: Receiver<Chatter>,
    running_state: RunningState,
    credits: Option<Credits>,
    game_start_time: Instant,
    object_sound: audio::Source,
    scores: HashMap<String, u128>,
    command_parser: CommandParser,
    gamewindow: GamePlayWindow,
}

impl GameState {
    pub fn new(
        run_config: Option<RunConfig>,
        screen_size: (f32, f32),
        context: &mut Context,
    ) -> GameResult<GameState> {
        let conf = run_config.unwrap_or_default();
        let (send_to_game, receive_from_chat) = channel::<ChatMessage>();
        let (send_to_chat, receive_from_game) = channel::<String>();

        if conf.test_bot_chatters > 0 {
            chat_test_mock::run(
                send_to_game.clone(),
                conf.test_bot_chatters,
                conf.test_command_occurences,
                SPLASH_DURATION,
                250,
                1500,
            );
        }

        if conf.attach_to_twitch_channel {
            let _twitchchat_thread = thread::spawn(move || {
                twitch_chat_wrapper::run(receive_from_game, send_to_game).unwrap();
            });
        }

        let game_started_message = format!("In {} seconds the Get the Streamer game will begin, you can play through chat with the commands on the right side of the game.", SPLASH_DURATION.as_secs());
        send_to_chat.send(game_started_message).unwrap();

        let interface = Interface::new(
            context,
            LIVES,
            crate::DROP_ZONE_COUNT,
            SPLASH_DURATION,
            DROP_ZONE_HEIGHT,
        )?;

        // create player
        let (send_player_hit_object_event, receive_player_hit_object_event) = channel();

        let player: GameObject = Self::create_player(context, send_player_hit_object_event);
        let game_start_time = Instant::now();

        let mut gameworld: GameWorld = GameWorld::new(
            screen_size.0 - interface.sidebar_width,
            screen_size.1 - DROP_ZONE_HEIGHT,
            DROP_ZONE_HEIGHT,
        );
        gameworld.add_game_object(player);
        let gamewindow = GamePlayWindow::new(
            screen_size.0 - interface.sidebar_width,
            screen_size.1 - DROP_ZONE_HEIGHT,
        );

        //@ootsby - 2020-11-04
        //I need to be able to hush things up. This is a temp solution until someone gets around to a
        //central sound asset manager and volume setting.
        let mut object_sound = audio::Source::new(context, "/threeTone1.ogg").unwrap();
        object_sound.set_volume(crate::GLOBAL_VOLUME);

        Ok(GameState {
            send_to_chat,
            receive_from_chat,
            screen_size,
            interface,
            gamewindow,
            gameworld,
            player_hit_object_event: receive_player_hit_object_event,
            running_state: RunningState::StartingSoon,
            credits: None,
            game_start_time,
            object_sound,
            scores: HashMap::new(),
            command_parser: CommandParser::new(&command::COMMAND_MAPPING),
        })
    }

    fn create_player(
        context: &mut Context,
        send_player_hit_object_event: Sender<Chatter>,
    ) -> GameObject {
        let player_scale = 4.0;
        let player_forward_sprite = Sprite::new(context, "/player_forward.png", 8, 1);
        let player_left_sprite = Sprite::new(context, "/player_left.png", 8, 1);
        let player_draw_system =
            PlayerDrawSystem::new(player_left_sprite, player_forward_sprite, player_scale);
        let player_size = player_draw_system.get_size().unwrap_or((50.0, 50.0));
        let player_physics_system = PlayerPhysics::new(context, send_player_hit_object_event);
        GameObject::new(
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
        )
    }

    fn handle_command(
        &mut self,
        command: Option<CommandInstance>,
        context: &mut Context,
    ) -> GameResult<()> {
        if let Some(command) = command {
            let chatter = command.chatter.clone();
            self.object_sound.play().unwrap();
            self.gameworld.add_game_object(command.handle(
                self.gameworld.get_column_coordinates_by_index(command.id),
                context,
            )?);
            let score = self.scores.entry(chatter.name).or_insert(0);
            *score += 1;
        }
        Ok(())
    }

    fn update_scores(&self, high_scores: &mut HashMap<String, u128>) {
        for (username, score) in &self.scores {
            let high_score = high_scores.entry(username.to_owned()).or_insert(0);
            *high_score += *score;
        }
    }

    fn send_game_started_message(&self) {
        let message = format!(
            "You have {} seconds to send your commands to Get the Streamer!",
            GAME_TIME.as_secs()
        );
        if let Err(error) = self.send_to_chat.send(message) {
            eprintln!("error sending game started message to chat: {}", error);
        }
    }

    fn send_game_ended_message(&self, winner: RunningState) {
        let (highest_scorer, score) = self
            .get_highest_scorer()
            .unwrap_or_else(|| ("nobody".to_owned(), 0));

        let message = match winner {
            RunningState::ChatWon => format!(
                "You all won, highest scorer was {} with {} points!",
                highest_scorer, score
            ),
            _ => format!(
                "The Streamer won the game despite the best efforts of {} who got {} points!",
                highest_scorer, score
            ),
        };

        if let Err(error) = self.send_to_chat.send(message) {
            eprintln!("error sending game ended message to chat: {}", error);
        }
    }

    fn end_game(&mut self, new_running_state: RunningState) {
        self.send_game_ended_message(new_running_state);
        self.running_state = new_running_state;
    }

    fn get_highest_scorer(&self) -> Option<(String, u128)> {
        if let Some(scorer) = self.scores.iter().max_by(|a, b| a.1.cmp(b.1)) {
            Some((scorer.0.to_owned(), *scorer.1))
        } else {
            None
        }
    }
}
impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if let Ok(chat_message) = self.receive_from_chat.try_recv() {
            if matches!(self.running_state, RunningState::Playing) {
                let chatter_name = if let Some(display_name) = chat_message.display_name {
                    display_name
                } else {
                    chat_message.name.clone()
                };
                match self.command_parser.parse_message(
                    &chat_message.message,
                    Chatter::new(
                        chatter_name,
                        chat_message.color_rgb,
                        chat_message.subscriber,
                    ),
                ) {
                    Err(error) => self.send_to_chat.send(error.to_owned()).unwrap(),
                    Ok(command) => self.handle_command(command, context)?,
                }
            }
        }

        while timer::check_update_time(context, FRAMERATE_TARGET) {
            match self.running_state {
                RunningState::StartingSoon => {
                    if let Err(error) = self.interface.update(context, LIVES) {
                        eprintln!("Error updating game objects in interface: {}", error);
                    }
                    if self.interface.splash_is_done() {
                        self.send_game_started_message();
                        self.running_state = RunningState::Playing;
                        self.interface.set_timer(
                            context,
                            Instant::now(),
                            GAME_TIME,
                            (1.0, 0.0, 0.0, 1.0),
                        );
                        self.game_start_time = Instant::now();
                    }
                }
                RunningState::Playing => {
                    // get the player lives left
                    let lives_left = if let Some(player) = self.gameworld.get_player() {
                        player.get_lives_left().unwrap_or(3)
                    } else {
                        0
                    };

                    if let Err(error) = self.interface.update(context, lives_left) {
                        eprintln!("Error updating game objects in interface: {}", error);
                    }

                    let game_time_left =
                        GAME_TIME.as_secs() - self.game_start_time.elapsed().as_secs();
                    if game_time_left == 0 {
                        self.end_game(RunningState::PlayerWon);
                    }

                    let _ = self.gameworld.update(context);

                    if let Ok(chatter) = self.player_hit_object_event.try_recv() {
                        let message_to_chat = format!("Hit! {} gets 10 points", &chatter.name);
                        let _ = self.send_to_chat.send(message_to_chat);
                        let score = self.scores.entry(chatter.name).or_insert(0);
                        *score += 10;
                    }

                    if self.gameworld.get_player().is_none() {
                        self.end_game(RunningState::ChatWon);
                    }
                }
                RunningState::ChatWon | RunningState::PlayerWon => {
                    if let Some(credits) = &mut self.credits {
                        if !credits.update() {
                            ggez::event::quit(context);
                        }
                    } else {
                        let mut high_scores = utilities::load_scores(SCORES_FILE_NAME, context);
                        self.update_scores(&mut high_scores);
                        if let Err(error) =
                            utilities::save_scores(context, SCORES_FILE_NAME, &high_scores)
                        {
                            eprintln!("Error saving high scores to disk: {}", error);
                        }
                        self.credits = Some(Credits::new(
                            self.running_state,
                            context,
                            self.screen_size,
                            &high_scores,
                            &self.scores,
                        )?);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        self.interface.draw(context, &self.running_state)?;

        match self.running_state {
            RunningState::StartingSoon => (),
            RunningState::Playing => {
                let _ =
                    self.gamewindow
                        .draw_gameworld(context, 0.0, DROP_ZONE_HEIGHT, &self.gameworld);
            }
            RunningState::PlayerWon | RunningState::ChatWon => {
                if let Some(credits) = &self.credits {
                    credits.draw(context)?;
                }
            }
        }

        graphics::present(context)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.screen_size = (width, height);

        let _ = graphics::set_screen_coordinates(
            ctx,
            graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: width,
                h: height,
            },
        );

        self.interface.update_screen_size(ctx, width, height);

        self.gamewindow.update_dimensions(
            ctx,
            width - self.interface.sidebar_width,
            height - DROP_ZONE_HEIGHT,
        );
    }
}
