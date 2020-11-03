use std::time::{Duration, Instant};

use ggez::{Context, GameResult, graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect}, nalgebra::Point2, graphics};

pub struct UITimer {
    duration: Duration,
    width: f32,
    height: f32,
    mesh: Mesh,
    start_time: Instant,
    color: (f32, f32, f32, f32),
}

impl UITimer {
    //constructor assumes immediate timer start
    pub fn new(
        context: &mut Context,
        start_time: Instant,
        duration: Duration,
        width: f32,
        height: f32,
        color: (f32, f32, f32, f32),
    ) -> UITimer {
        UITimer {
            duration,
            width,
            height,
            mesh: Self::create_mesh(context, width, height, color),
            start_time,
            color,
        }
    }

    fn get_remaining(&self) -> Duration {
        self.duration
            .clone()
            .checked_sub(self.start_time.elapsed())
            .unwrap_or(Duration::new(0, 0))
    }

    fn create_mesh(
        context: &mut Context,
        width: f32,
        height: f32,
        color: (f32, f32, f32, f32),
    ) -> Mesh {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, width, height),
                Color::new(color.0, color.1, color.2, color.3),
            )
            .build(context)
            .unwrap();
        mesh
    }

    pub fn draw(&self, context: &mut Context, x: f32, y: f32) -> GameResult {
        let elapsed_fraction = self.get_remaining().as_secs_f32() / self.duration.as_secs_f32();
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::default()
                .offset([0.0, self.height])
                .dest(Point2::new(x, y))
                .scale([1.0_f32, elapsed_fraction]),
        )
    }

    pub fn update(&self, _time_since_start: std::time::Duration, _context: &mut Context) {}

    pub fn width(&self) -> f32 {
        return self.width;
    }

    pub fn height(&self) -> f32 {
        return self.height;
    }

    pub fn get_start_time(&self) -> Instant{
      return self.start_time;
    }

    pub fn get_duration(&self) -> Duration{
      return self.duration;
    }

    pub fn get_color(&self) -> (f32,f32,f32,f32){
      return self.color;
    }
}
