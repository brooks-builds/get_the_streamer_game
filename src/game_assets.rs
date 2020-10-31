use ggez::Context;
use ggez::graphics::Image;
use std::collections::HashMap;

pub struct GameAssets {
    img_cache: HashMap<String, Image>,
}

impl GameAssets {
    pub fn new() -> GameAssets {
        Self {
            img_cache: HashMap::new(),
        }
    }

    pub fn get_image(&mut self, context: &mut Context, path: String) -> Image {
        let Self { img_cache } = self;
        let img_entry = img_cache.entry(path.clone());
        //Image::new(get_game_context().unwrap(), path)
        return img_entry
            .or_insert_with(|| Image::new(context, path.clone()).unwrap())
            .clone();
    }
}
