use ggez::{nalgebra::Point2, Context, GameResult};

use crate::sprites::Sprite;

use super::DrawSystem;

#[derive(Debug)]
enum Facing {
    Forward,
    Left,
    Right,
}

#[derive(Debug)]
pub struct PlayerDrawSystem {
    left_sprite: Sprite,
    forward_sprite: Sprite,
    scale_by: f32,
    facing: Facing,
}

impl PlayerDrawSystem {
    pub fn new(left_sprite: Sprite, forward_sprite: Sprite, scale_by: f32) -> Self {
        Self {
            left_sprite,
            forward_sprite,
            scale_by,
            facing: Facing::Forward,
        }
    }
}

impl DrawSystem for PlayerDrawSystem {
    fn update(&mut self, _time_since_start: std::time::Duration, velocity_x: f32) {
        dbg!(velocity_x);
        if velocity_x < -0.1 {
            self.facing = Facing::Left;
        } else if velocity_x > 0.1 {
            self.facing = Facing::Right;
        } else {
            self.facing = Facing::Forward;
        }
    }

    fn draw(&self, context: &mut Context, location: Point2<f32>, rotation: &f32) -> GameResult<()> {
        match self.facing {
            Facing::Forward => self.forward_sprite.draw(
                context,
                location,
                [self.scale_by, self.scale_by],
                rotation,
                Some(1.0),
            ),
            Facing::Left => self.left_sprite.draw(
                context,
                location,
                [self.scale_by, self.scale_by],
                rotation,
                Some(1.0),
            ),
            Facing::Right => self.left_sprite.draw(
                context,
                location,
                [-self.scale_by, self.scale_by],
                rotation,
                Some(1.0),
            ),
        }
    }

    fn get_size(&self) -> Option<(f32, f32)> {
        Some((
            self.forward_sprite.width * self.scale_by,
            self.forward_sprite.height * self.scale_by,
        ))
    }
}
