use super::{DrawSystem, Sprite};
use ggez::graphics::{DrawParam, Font, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

#[derive(Debug)]
pub struct GameObjectDrawSystem {
    sprite: Option<Sprite>,
    label: Option<Text>,
    scale_by: f32,
}

impl GameObjectDrawSystem {
    pub fn new(
        sprite: Option<Sprite>,
        label: Option<&'static str>,
        scale_by: f32,
    ) -> GameObjectDrawSystem {
        let label = match label {
            Some(text) => {
                let mut text = Text::new(text);
                text.set_font(Font::default(), Scale::uniform(35.0));
                Some(text)
            }
            None => None,
        };
        GameObjectDrawSystem {
            sprite,
            label,
            scale_by,
        }
    }
}

impl DrawSystem for GameObjectDrawSystem {
    fn update(&mut self, time_since_start: std::time::Duration) {
        if let Some(sprite) = &mut self.sprite {
            sprite.update(time_since_start);
        }
    }

    fn draw(&self, context: &mut Context, location: Point2<f32>) -> GameResult<()> {
        if let Some(sprite) = &self.sprite {
            sprite.draw(context, location, self.scale_by)?;
        }

        if let Some(label) = &self.label {
            let label_width = label.width(context) as f32;
            let label_height = label.height(context) as f32;
            let size = self.get_size().unwrap_or((50.0, 50.0));

            graphics::draw(
                context,
                label,
                DrawParam::new().dest(Point2::new(
                    location.x - label_width / 2.0 + size.0 / 2.0,
                    location.y - label_height - 5.0,
                )),
            )?;
        }

        Ok(())
    }

    fn get_size(&self) -> Option<(f32, f32)> {
        if let Some(sprite) = &self.sprite {
            Some((sprite.width, sprite.height))
        } else {
            None
        }
    }
}
