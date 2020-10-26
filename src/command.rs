use ggez::{nalgebra::Point2, Context, GameResult};
use rand::prelude::*;

use crate::{
    draw_system::DrawSystem, draw_system::GameObjectDrawSystem, game_object::GameObject,
    game_object_type::GameObjectType, life_system::FireLifeSystem, life_system::HeartLifeSystem,
    life_system::LifeSystem, life_system::SnakeLifeSystem, life_system::SwordLifeSystem,
    physics::FirePhysics, physics::HeartPhysics, physics::PhysicsSystem, physics::SnakePhysics,
    physics::SwordPhysics, sprites::Sprite, sprites::SpriteImageDef,
};

use super::Chatter;

pub struct Command<'a> {
    pub command_type: CommandType<'a>,
    pub id: u8,
    pub chatter: Chatter,
}

impl<'a> Command<'a> {
    pub fn new(message: &str, chatter: Chatter) -> Result<Option<Command>, &'static str> {
        if !message.starts_with('#') {
            return Ok(None);
        }

        let mut parts = message.split(' ');
        if let Some(command) = parts.next() {
            let id = Self::get_id_from_message(parts.next())?;
            match command {
                "#fire" => Ok(Some(Command {
                    command_type: CommandTypes::Fire,
                    id,
                    chatter,
                })),
                "#sword" => Ok(Some(Command {
                    command_type: CommandTypes::Sword,
                    id,
                    chatter,
                })),
                "#snake" | "#snek" => Ok(Some(Command {
                    command_type: CommandTypes::Snake,
                    id,
                    chatter,
                })),
                "#heart" => Ok(Some(Command {
                    command_type: CommandTypes::Heart,
                    id,
                    chatter,
                })),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
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

#[derive(PartialEq, Debug)]
pub enum OldCommandType {
    Fire,
    Sword,
    Snake,
    Heart,
}

pub struct CommandType<'a> {
    pub command_string: &'a str,
    pub game_object_type: GameObjectType,
    pub scale: f32,
    pub sprite_def: SpriteImageDef,
    pub physics_system: fn() -> Option<Box<dyn PhysicsSystem>>,
    pub life_system: fn() -> Option<Box<dyn LifeSystem>>,
}

pub struct CommandTypes {}

impl<'a> CommandTypes {
    const Fire: CommandType<'a> = CommandType {
        command_string: "fire",
        game_object_type: GameObjectType::Enemy,
        scale: 2.0,
        sprite_def: SpriteImageDef::new("/LargeFlame.png", 4, 1),
        physics_system: || Some(Box::new(FirePhysics::new())),
        life_system: || Some(Box::new(FireLifeSystem::new())),
    };
    const Sword: CommandType<'a> = CommandType {
        command_string: "sword",
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/item1BIT_sword.png", 1, 1),
        physics_system: || Some(Box::new(SwordPhysics::new())),
        life_system: || Some(Box::new(SwordLifeSystem::new())),
    };
    const Snake: CommandType<'a> = CommandType {
        command_string: "snake",
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/snake.png", 4, 1),
        physics_system: || Some(Box::new(SnakePhysics::new())),
        life_system: || Some(Box::new(SnakeLifeSystem::new())),
    };
    const Snek: CommandType<'a> = CommandType {
        command_string: "snek",
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/snake.png", 4, 1),
        physics_system: || Some(Box::new(SnakePhysics::new())),
        life_system: || Some(Box::new(SnakeLifeSystem::new())),
    };
    const Heart: CommandType<'a> = CommandType {
        command_string: "heart",
        game_object_type: GameObjectType::Heart,
        scale: 1.5,
        sprite_def: SpriteImageDef::new("/heart.png", 1, 1),
        physics_system: || Some(Box::new(HeartPhysics::new())),
        life_system: || Some(Box::new(HeartLifeSystem::new())),
    };
}

pub static COMMAND_TYPES: CommandTypes = CommandTypes {};
