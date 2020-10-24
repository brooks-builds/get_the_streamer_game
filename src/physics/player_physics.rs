use crate::{game_object_type::GameObjectType, life_system::LifeSystem};

use super::{Chatter, GameObject, PhysicsSystem};
use eyre::Result;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Point2;
use ggez::{input, Context};
use std::sync::mpsc::Sender;

const MOVE_FORCE: f32 = 2.0;
const JUMP_FORCE: f32 = -12.5;
const FRICTION: f32 = 0.15;
const DEFAULT_CHATTER_NAME: &str = "Unknown Player";

#[derive(Debug)]
pub struct PlayerPhysics {
    velocity: Point2<f32>,
    affected_by_gravity: bool,
    player_hit_object: Sender<Chatter>,
}

impl PlayerPhysics {
    pub fn new(player_hit_object: Sender<Chatter>) -> PlayerPhysics {
        PlayerPhysics {
            velocity: Point2::new(0.0, 0.0),
            affected_by_gravity: true,
            player_hit_object,
        }
    }

    fn handle_input(&mut self, context: &mut Context) {
        if input::keyboard::is_key_pressed(context, KeyCode::A) {
            self.velocity.x -= MOVE_FORCE;
        } else if input::keyboard::is_key_pressed(context, KeyCode::S)
            || input::keyboard::is_key_pressed(context, KeyCode::D) {
            self.velocity.x += MOVE_FORCE;
        }

        if input::keyboard::is_key_pressed(context, KeyCode::Space) && self.on_ground() {
            self.velocity.y += JUMP_FORCE;
            self.affected_by_gravity = true;
        }
    }

    fn on_ground(&self) -> bool {
        !self.affected_by_gravity
    }

    fn stay_in_arena(&mut self, location: &mut Rect, (arena_width, arena_height): (f32, f32)) {
        if location.y + location.h > arena_height {
            self.affected_by_gravity = false;
            self.velocity.y = 0.0;
            location.y = arena_height - location.h;
        }

        if location.x < 0.0 {
            location.x = 0.0;
        } else if location.x + location.w > arena_width {
            location.x = arena_width - location.w;
        }
    }

    fn get_colliding_with(
        &self,
        collidable_game_objects: &Vec<GameObject>,
        location: &Rect,
    ) -> Option<GameObject> {
        for other_game_object in collidable_game_objects {
            if other_game_object.location.overlaps(location)
                && other_game_object.my_type != GameObjectType::Player
            {
                return Some(other_game_object.clone());
            }
        }
        None
    }
}

impl PhysicsSystem for PlayerPhysics {
    fn update(
        &mut self,
        location: &mut Rect,
        arena: (f32, f32),
        gravity_force: f32,
        context: &mut Context,
        collidable_game_objects: &Vec<GameObject>,
        _rotation: &mut f32,
        life_system: &mut Option<Box<dyn LifeSystem>>,
    ) -> Result<()> {
        self.handle_input(context);
        self.stay_in_arena(location, arena);

        if let Some(game_object) = self.get_colliding_with(collidable_game_objects, location) {
            if let Some(player_life_system) = life_system.as_deref_mut() {
                if GameObjectType::Heart == game_object.my_type {
                    player_life_system.gain_life();
                } else {
                    if player_life_system.hit() {
                        let chatter = if let Some(chatter) = game_object.chatter {
                            chatter
                        } else {
                            Chatter::new(DEFAULT_CHATTER_NAME.to_owned(), (255, 255, 255), false)
                        };
                        self.player_hit_object.send(chatter)?;
                    }
                }
            }
        }

        if self.affected_by_gravity {
            self.velocity.y += gravity_force;
        }
        location.x += self.velocity.x;
        location.y += self.velocity.y;

        if self.velocity.x != 0.0 {
            let opposite_velocity = self.velocity.x * -1.0;
            let speed_decrease = opposite_velocity * FRICTION;
            self.velocity.x += speed_decrease
        }

        Ok(())
    }

    fn get_velocity_x(&self) -> f32 {
        self.velocity.x
    }
}
