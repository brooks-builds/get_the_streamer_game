use super::DrawSystem;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

const TIMER_WIDTH: f32 = 5.0;
const TIMER_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);

#[derive(Debug)]
pub struct TimerDrawSystem {
    timer: Mesh,
    width: f32,
    height: f32,
}

impl TimerDrawSystem {
    pub fn new(
        (_screen_width, screen_height): (f32, f32),
        context: &mut Context,
    ) -> GameResult<TimerDrawSystem> {
        let timer = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, TIMER_WIDTH, screen_height),
                TIMER_COLOR,
            )
            .build(context)?;

        Ok(TimerDrawSystem {
            timer,
            width: TIMER_WIDTH,
            height: screen_height,
        })
    }
}

impl DrawSystem for TimerDrawSystem {
    fn update(&mut self, _time_since_start: std::time::Duration) {}

    fn draw(
        &self,
        context: &mut Context,
        location: Point2<f32>,
        _rotation: &f32,
    ) -> GameResult<()> {
        graphics::draw(context, &self.timer, DrawParam::new().dest(location))
    }

    fn get_size(&self) -> Option<(f32, f32)> {
        Some((self.width, self.height))
    }
}
