use get_the_streamer_game::GameState;
use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder};

fn main() {
    let (context, event_loop) = &mut match ContextBuilder::new("Get the Streamer", "Brooks Patton")
        .window_mode(WindowMode::default().dimensions(1920.0, 1080.0))
        .build()
    {
        Ok((context, event_loop)) => (context, event_loop),
        Err(error) => panic!(error),
    };

    let game_state = &mut GameState::new();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };
}
