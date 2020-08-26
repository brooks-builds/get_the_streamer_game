use ggez::graphics::Image;
use ggez::{Context, GameResult};

pub struct Sprites {
    pub fire: Image,
}

impl Sprites {
    pub fn new(context: &mut Context) -> GameResult<Sprites> {
        let fire = Image::new(context, "/LargeFlame.png")?;

        Ok(Sprites { fire })
    }
}
