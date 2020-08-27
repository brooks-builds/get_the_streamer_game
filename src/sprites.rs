use ggez::graphics::{DrawParam, Image, Rect};
use ggez::{graphics, Context, GameResult};
use std::time::Duration;

pub struct Sprite {
    image: Image,
    individual_sprite_rects: Vec<Rect>,
    rect_index: usize,
    next_time_to_change: Duration,
    index_change_duration: Duration,
}

impl Sprite {
    pub fn new(
        context: &mut Context,
        path: &'static str,
        sprites_accross: u16,
        sprites_down: u16,
    ) -> GameResult<Sprite> {
        let image = Image::new(context, path)?;
        let mut individual_sprite_rects = vec![];
        let image_width = image.width();
        let image_height = image.height();
        let single_sprite_width = image_width / sprites_accross;
        let single_sprite_height = image_height / sprites_down;
        let single_sprite_width_fraction = single_sprite_width as f32 / image_width as f32;
        let single_sprite_height_fraction = single_sprite_height as f32 / image_height as f32;
        let rect_index = 0;
        let index_change_duration = Duration::from_millis(100);
        let next_time_to_change = index_change_duration.clone();

        for y_index in 0..sprites_down {
            for x_index in 0..sprites_accross {
                let rect = Rect::new(
                    x_index as f32 * single_sprite_width_fraction,
                    y_index as f32 * single_sprite_height_fraction,
                    single_sprite_width_fraction,
                    single_sprite_height_fraction,
                );
                individual_sprite_rects.push(rect);
            }
        }

        Ok(Sprite {
            image,
            individual_sprite_rects,
            rect_index,
            next_time_to_change,
            index_change_duration,
        })
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::draw(
            context,
            &self.image,
            DrawParam::new().src(self.individual_sprite_rects[self.rect_index]),
        )
    }
    pub fn update(&mut self, time_since_start: std::time::Duration) {
        if time_since_start >= self.next_time_to_change {
            self.rect_index += 1;
            if self.rect_index == self.individual_sprite_rects.len() {
                self.rect_index = 0;
            }
            self.next_time_to_change = time_since_start + self.index_change_duration;
        }
    }
}
