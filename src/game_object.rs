use super::{Chatter, DrawSystem, GameObjectType, LifeSystem, PhysicsSystem};
use eyre::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

const GRAVITY_FORCE: f32 = 0.3;

#[derive(Debug)]
pub struct GameObject {
    pub location: Rect,
    draw_system: Option<Box<dyn DrawSystem>>,
    physics_system: Option<Box<dyn PhysicsSystem>>,
    // birth_time: Instant,
    // live_for: Option<Duration>,
    // life_left: u8,
    life_system: Option<Box<dyn LifeSystem>>,
    pub collidable: bool,
    pub chatter: Option<Chatter>,
    rotation: f32,
    pub my_type: GameObjectType,
}

impl GameObject {
    pub fn new(
        x: f32,
        y: f32,
        draw_system: Option<Box<dyn DrawSystem>>,
        width: f32,
        height: f32,
        physics_system: Option<Box<dyn PhysicsSystem>>,
        collidable: bool,
        chatter: Option<Chatter>,
        my_type: GameObjectType,
        life_system: Option<Box<dyn LifeSystem>>,
    ) -> GameObject {
        GameObject {
            location: Rect::new(x, y, width, height),
            draw_system,
            physics_system,
            life_system,
            collidable,
            chatter,
            rotation: 0.0,
            my_type,
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
                &mut self.rotation,
                &mut self.life_system,
            )?;
        }

        if let Some(draw_system) = &mut self.draw_system {
            draw_system.update(time_since_start);
        }

        Ok(())
    }

    pub fn draw(&self, context: &mut Context, iframes: bool) -> GameResult<()> {
        if let Some(draw_system) = &self.draw_system {
            draw_system.draw(
                context,
                Point2::new(self.location.x, self.location.y),
                &self.rotation,
                iframes,
            )?;
        }

        return Ok(());
    }
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        GameObject {
            location: self.location.clone(),
            draw_system: None,
            physics_system: None,
            collidable: self.collidable,
            chatter: self.chatter.clone(),
            rotation: self.rotation.clone(),
            my_type: self.my_type.clone(),
            life_system: None,
        }
    }
}
