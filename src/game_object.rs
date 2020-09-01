use super::{DrawSystem, PhysicsSystem};
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

pub struct GameObject {
    location: Rect,
    draw_system: Option<DrawSystem>,
    children: Vec<GameObject>,
    physics_system: Option<PhysicsSystem>,
    birth_time: Instant,
    live_for: Option<Duration>,
}

impl GameObject {
    pub fn new(
        x: f32,
        y: f32,
        draw_system: Option<DrawSystem>,
        width: f32,
        height: f32,
        physics_system: Option<PhysicsSystem>,
        live_for: Option<Duration>,
    ) -> GameObject {
        let live_until = if let Some(live_for) = live_for {
            Some(Instant::now() + live_for)
        } else {
            None
        };

        GameObject {
            location: Rect::new(x, y, width, height),
            draw_system,
            children: vec![],
            physics_system,
            live_for,
            birth_time: Instant::now(),
        }
    }

    pub fn update(&mut self, time_since_start: std::time::Duration, screen_height: f32) {
        if let Some(physics_system) = &mut self.physics_system {
            physics_system.update(&mut self.location, screen_height);
        }

        if let Some(draw_system) = &mut self.draw_system {
            draw_system.update(time_since_start);
        }

        self.children
            .iter_mut()
            .for_each(|game_object| game_object.update(time_since_start, screen_height));
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        if let Some(draw_system) = &self.draw_system {
            draw_system.draw(context, Point2::new(self.location.x, self.location.y))?;
        }

        for game_object in &self.children {
            game_object.draw(context)?;
        }

        return Ok(());
    }

    pub fn add_child(&mut self, game_object: GameObject) {
        self.children.push(game_object);
    }

    pub fn is_alive(&self) -> bool {
        if let Some(live_for) = self.live_for {
            self.birth_time.elapsed() < live_for
        } else {
            true
        }
    }
}
