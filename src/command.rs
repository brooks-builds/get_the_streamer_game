use crate::{
    draw_system::DrawSystem, draw_system::GameObjectDrawSystem, game_object::GameObject,
    game_object_type::GameObjectType, life_system::FireLifeSystem, life_system::HeartLifeSystem,
    life_system::LifeSystem, life_system::SnakeLifeSystem, life_system::SwordLifeSystem,
    physics::FirePhysics, physics::HeartPhysics, physics::PhysicsSystem, physics::SnakePhysics,
    physics::SwordPhysics, sprites::Sprite, sprites::SpriteImageDef,
};
use ggez::{nalgebra::Point2, Context, GameResult};
use rand::prelude::*;
use std::collections::HashMap;

use super::Chatter;

pub const COMMAND_MAPPING: [(&'static str, CommandType); 5] = [
    ("#fire", CommandTypes::FIRE),
    ("#snake", CommandTypes::SNAKE),
    ("#snek", CommandTypes::SNAKE),
    ("#sword", CommandTypes::SWORD),
    ("#heart", CommandTypes::HEART),
];

pub struct CommandParser {
    command_map: HashMap<&'static str, CommandType>,
}

impl CommandParser {
    pub fn new(command_mapping: &[(&'static str, CommandType)]) -> CommandParser {
        return CommandParser {
            command_map: command_mapping.iter().cloned().collect(),
        };
    }

    fn get_id_from_message(message_part: Option<&str>) -> Result<u8, &'static str> {
        if let Some(id) = message_part {
            match id.parse::<u8>() {
                Ok(number) => {
                    if number < crate::DROP_ZONE_COUNT {
                        Ok(number)
                    } else {
                        Err("The given column is outside of the arena")
                    }
                }
                Err(_error) => return Err("I couldn't tell what column to drop into"),
            }
        } else {
            let mut rng = rand::thread_rng();
            Ok(rng.gen_range(0, crate::DROP_ZONE_COUNT))
        }
    }

    pub fn get_commandtype(&self, command_string: &str) -> Option<&CommandType> {
        self.command_map.get(command_string)
    }

    pub fn parse_message(
        &self,
        message: &str,
        chatter: Chatter,
    ) -> Result<Option<Command>, &'static str> {
        if !message.starts_with('#') {
            return Ok(None);
        }

        let mut parts = message.split(' ');
        if let Some(command) = parts.next() {
            let id = Self::get_id_from_message(parts.next())?;
            match self.get_commandtype(command) {
                Some(command_type) => Ok(Some(Command {
                    command_type: command_type.clone(),
                    chatter,
                    id,
                })),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

pub struct Command {
    pub command_type: CommandType,
    pub id: u8,
    pub chatter: Chatter,
}

impl Command {
    pub fn handle(
        &self,
        drop_zone_location: Point2<f32>,
        context: &mut Context,
    ) -> GameResult<GameObject> {
        let scale = self.get_scale();
        let sprite = self.get_sprite(context);
        let label_color = if self.chatter.is_subscriber {
            self.chatter.get_color()
        } else {
            ggez::graphics::WHITE
        };
        let label = Some((self.chatter.name.clone(), label_color));
        let draw_system = GameObjectDrawSystem::new(sprite, label, scale);
        let size = draw_system.get_size().unwrap_or((50.0, 50.0));
        let physics_system = self.get_physics();
        let game_object = GameObject::new(
            drop_zone_location.x - size.0 / 2.0,
            drop_zone_location.y - size.1 / 2.0,
            Some(Box::new(draw_system)),
            size.0,
            size.1,
            physics_system,
            true,
            Some(self.chatter.clone()),
            self.command_type.game_object_type.clone(),
            self.get_life_system(),
        );
        Ok(game_object)
    }

    fn get_scale(&self) -> f32 {
        return self.command_type.scale;
    }

    fn get_sprite(&self, context: &mut Context) -> Sprite {
        let def = &self.command_type.sprite_def;
        return Sprite::new(context, def.image_path, def.frames_x, def.frames_y);
    }

    fn get_physics(&self) -> Option<Box<dyn PhysicsSystem>> {
        return (self.command_type.physics_system)();
    }

    fn get_life_system(&self) -> Option<Box<dyn LifeSystem>> {
        return (self.command_type.life_system)();
    }
}

#[derive(Clone)]
pub struct CommandType {
    pub game_object_type: GameObjectType,
    pub scale: f32,
    pub sprite_def: SpriteImageDef,
    pub physics_system: fn() -> Option<Box<dyn PhysicsSystem>>,
    pub life_system: fn() -> Option<Box<dyn LifeSystem>>,
}

pub struct CommandTypes {}

impl CommandTypes {
    const FIRE: CommandType = CommandType {
        game_object_type: GameObjectType::Enemy,
        scale: 2.0,
        sprite_def: SpriteImageDef::new("/LargeFlame.png", 4, 1),
        physics_system: || Some(Box::new(FirePhysics::new())),
        life_system: || Some(Box::new(FireLifeSystem::new())),
    };
    const SWORD: CommandType = CommandType {
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/item1BIT_sword.png", 1, 1),
        physics_system: || Some(Box::new(SwordPhysics::new())),
        life_system: || Some(Box::new(SwordLifeSystem::new())),
    };
    const SNAKE: CommandType = CommandType {
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/snake.png", 4, 1),
        physics_system: || Some(Box::new(SnakePhysics::new())),
        life_system: || Some(Box::new(SnakeLifeSystem::new())),
    };
    const HEART: CommandType = CommandType {
        game_object_type: GameObjectType::Heart,
        scale: 1.5,
        sprite_def: SpriteImageDef::new("/heart.png", 1, 1),
        physics_system: || Some(Box::new(HeartPhysics::new())),
        life_system: || Some(Box::new(HeartLifeSystem::new())),
    };
}

pub static COMMAND_TYPES: CommandTypes = CommandTypes {};
