use ggez::{nalgebra::Point2, Context, GameResult};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{
    draw_system::DrawSystem, draw_system::GameObjectDrawSystem, game_object::GameObject,
    game_object_type::GameObjectType, life_system::FireLifeSystem, life_system::HeartLifeSystem,
    life_system::LifeSystem, life_system::SnakeLifeSystem, life_system::SwordLifeSystem,
    physics::FirePhysics, physics::HeartPhysics, physics::PhysicsSystem, physics::SnakePhysics,
    physics::SwordPhysics, sprites::Sprite,
};

use super::Chatter;

pub struct Command {
    pub command_type: CommandType,
    pub id: u8,
    pub chatter: Chatter,
}

impl Command {
    pub fn new(message: &str, chatter: Chatter) -> Result<Option<Command>, &'static str> {
        if !message.starts_with('#') {
            return Ok(None);
        }

        let mut parts = message.split(' ');
        if let Some(command) = parts.next() {
            let id = Self::get_id_from_message(parts.next())?;
            match command {
                "#fire" => Ok(Some(Command {
                    command_type: CommandType::Fire,
                    id,
                    chatter,
                })),
                "#sword" => Ok(Some(Command {
                    command_type: CommandType::Sword,
                    id,
                    chatter,
                })),
                "#snake" | "#snek" => Ok(Some(Command {
                    command_type: CommandType::Snake,
                    id,
                    chatter,
                })),
                "#heart" => Ok(Some(Command {
                    command_type: CommandType::Heart,
                    id,
                    chatter,
                })),
                "#random" | "#rand" | "#rng" => Ok(Some(Command {
                    command_type: CommandType::random(),
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
                Err(_error) => Err("I couldn't tell what column to drop into"),
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
        let sprite = self.get_sprite(context)?;
        let label_color = if self.chatter.is_subscriber {
            self.chatter.get_color()
        } else {
            ggez::graphics::WHITE
        };
        let label = Some((self.chatter.name.clone(), label_color));
        let draw_system = GameObjectDrawSystem::new(Some(sprite), label, scale);
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
            self.get_game_object_type(),
            self.get_life_system(),
        );
        Ok(game_object)
    }

    fn get_scale(&self) -> f32 {
        match self.command_type {
            CommandType::Fire => 2.0,
            CommandType::Sword => 3.0,
            CommandType::Snake => 3.0,
            CommandType::Heart => 1.5,
        }
    }

    fn get_sprite(&self, context: &mut Context) -> GameResult<Sprite> {
        match self.command_type {
            CommandType::Fire => Sprite::new(context, "/LargeFlame.png", 4, 1),
            CommandType::Sword => Sprite::new(context, "/item1BIT_sword.png", 1, 1),
            CommandType::Snake => Sprite::new(context, "/snake.png", 4, 1),
            CommandType::Heart => Sprite::new(context, "/heart.png", 1, 1),
        }
    }

    fn get_physics(&self) -> Option<Box<dyn PhysicsSystem>> {
        match self.command_type {
            CommandType::Fire => Some(Box::new(FirePhysics::new())),
            CommandType::Sword => Some(Box::new(SwordPhysics::new())),
            CommandType::Snake => Some(Box::new(SnakePhysics::new())),
            CommandType::Heart => Some(Box::new(HeartPhysics::new())),
        }
    }

    fn get_life_system(&self) -> Option<Box<dyn LifeSystem>> {
        match self.command_type {
            CommandType::Fire => Some(Box::new(FireLifeSystem::new())),
            CommandType::Sword => Some(Box::new(SwordLifeSystem::new())),
            CommandType::Snake => Some(Box::new(SnakeLifeSystem::new())),
            CommandType::Heart => Some(Box::new(HeartLifeSystem::new())),
        }
    }

    fn get_game_object_type(&self) -> GameObjectType {
        match self.command_type {
            CommandType::Heart => GameObjectType::Heart,
            _ => GameObjectType::Enemy,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CommandType {
    Fire,
    Sword,
    Snake,
    Heart,
}

impl CommandType {
    pub fn random() -> Self {
        rand::random()
    }
}

impl Distribution<CommandType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CommandType {
        let random: f32 = rng.gen();

        if random < 0.1 {
            CommandType::Heart
        } else if random >= 0.1 && random < 0.4 {
            CommandType::Fire
        } else if random >= 0.4 && random < 0.7 {
            CommandType::Sword
        } else {
            CommandType::Snake
        }
    }
}
