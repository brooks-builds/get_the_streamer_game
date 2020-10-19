use ggez::graphics::{DrawParam, Image, Rect};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use graphics::Color;
use std::time::Duration;

#[derive(Debug)]
pub struct Sprite {
    image: Image,
    individual_sprite_rects: Vec<Rect>,
    rect_index: usize,
    next_time_to_change: Duration,
    index_change_duration: Duration,
    pub width: f32,
    pub height: f32,
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
            width: single_sprite_width as f32,
            height: single_sprite_height as f32,
        })
    }

    // handle mirroring x here

    pub fn draw(
        &self,
        context: &mut Context,
        location: Point2<f32>,
        scale_by: [f32; 2],
        rotation: &f32,
        opacity: Option<f32>,
        mirror_x: bool,
    ) -> GameResult<()> {
        let opacity = opacity.unwrap_or(1.0);
        let scale_by_y = if *rotation > 3.0 {
            -scale_by[1]
        } else {
            scale_by[1]
        };
        graphics::draw(
            context,
            &self.image,
            DrawParam::new()
                .src(self.individual_sprite_rects[self.rect_index])
                .dest(Point2::new(
                    location.x + (self.width * scale_by[0] / 2.0),
                    location.y + (self.height * scale_by[1] / 2.0),
                ))
                .offset(Point2::new(0.5, 0.5))
                .scale([scale_by[0], scale_by_y])
                .rotation(*rotation)
                .color(Color::new(1.0, 1.0, 1.0, opacity)),
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
