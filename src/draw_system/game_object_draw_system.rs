use super::{DrawSystem, Sprite};
use ggez::graphics::{Color, DrawParam, Font, Scale, Text, TextFragment};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

#[derive(Debug)]
pub struct GameObjectDrawSystem {
    sprite: Sprite,
    label: Option<Text>,
    scale_by: f32,
}

impl GameObjectDrawSystem {
    pub fn new(
        sprite: Sprite,
        label: Option<(String, Color)>,
        scale_by: f32,
    ) -> GameObjectDrawSystem {
        let label = label.map(|(text, color)| {
            Text::new(
                TextFragment::new(text)
                    .font(Font::default())
                    .scale(Scale::uniform(35.0))
                    .color(color),
            )
        });
        GameObjectDrawSystem {
            sprite,
            label,
            scale_by,
        }
    }
}

impl DrawSystem for GameObjectDrawSystem {
    fn update(&mut self, time_since_start: std::time::Duration, _velocity_x: f32) {
        self.sprite.update(time_since_start);
    }

    fn draw(&self, context: &mut Context, location: Point2<f32>, rotation: &f32) -> GameResult<()> {
        self.sprite.draw(
                context,
                location,
                [self.scale_by, self.scale_by],
                rotation,
                None,
            )?;

        //This scaling code is to correct what seems to be a bug in ggez where text
        //(and I'm guessing spritebatch) rendering is not affected properly by
        //previously applied transforms.
        //@ootsby - 2020-11-04
        let t = graphics::transform(context);
        let xscale= t.x.x;
        let yscale = t.y.y;

        let size = self.get_size().unwrap_or((50.0, 50.0));
        if let Some(label) = &self.label {
            let label_width = label.width(context) as f32;
            let label_height = label.height(context) as f32;

            graphics::draw(
                context,
                label,
                DrawParam::default().dest(Point2::new(
                    (location.x - label_width / 2.0 + size.0 / 2.0)*xscale,
                    (location.y - label_height - 5.0)*yscale,
                )),
            )?;
        }
        
        Ok(())
    }

    fn get_size(&self) -> Option<(f32, f32)> {
        Some((self.sprite.width * self.scale_by, self.sprite.height * self.scale_by))
    }
}
