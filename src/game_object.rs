use super::{Chatter, DrawSystem, PhysicsSystem};
use eyre::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

const GRAVITY_FORCE: f32 = 0.3;

#[derive(Debug)]
pub struct GameObject {
    pub location: Rect,
    draw_system: Option<DrawSystem>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
    birth_time: Instant,
    live_for: Option<Duration>,
    pub collidable: bool,
    pub chatter: Option<Chatter>,
}

impl GameObject {
    pub fn new(
        x: f32,
        y: f32,
        draw_system: Option<DrawSystem>,
        width: f32,
        height: f32,
        physics_system: Option<Box<dyn PhysicsSystem>>,
        live_for: Option<Duration>,
        collidable: bool,
        chatter: Option<Chatter>,
    ) -> GameObject {
        let _live_until = if let Some(live_for) = live_for {
            Some(Instant::now() + live_for)
        } else {
            None
        };

        GameObject {
            location: Rect::new(x, y, width, height),
            draw_system,
            physics_system,
            live_for,
            birth_time: Instant::now(),
            collidable,
            chatter,
        }
    }

    pub fn update(
        &mut self,
        time_since_start: std::time::Duration,
        screen_size: (f32, f32),
        context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
    ) -> Result<()> {
        if let Some(physics_system) = &mut self.physics_system {
            physics_system.update(
                &mut self.location,
                screen_size,
                GRAVITY_FORCE,
                context,
                collidable_game_objects,
            )?;
        }

        if let Some(draw_system) = &mut self.draw_system {
            draw_system.update(time_since_start);
        }

        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        if let Some(draw_system) = &self.draw_system {
            draw_system.draw(context, Point2::new(self.location.x, self.location.y))?;
        }

        return Ok(());
    }

    pub fn is_alive(&self) -> bool {
        if let Some(live_for) = self.live_for {
            self.birth_time.elapsed() < live_for
        } else {
            true
        }
    }
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        GameObject {
            location: self.location.clone(),
            draw_system: None,
            physics_system: None,
            birth_time: self.birth_time,
            live_for: None,
            collidable: self.collidable,
            chatter: self.chatter.clone(),
        }
    }
}
