use crate::{
    draw_system::DrawSystem, draw_system::GameObjectDrawSystem, game_object::GameObject,
    game_object_type::GameObjectType, game_world::GameWorld, life_system::FireLifeSystem,
    life_system::HeartLifeSystem, life_system::LifeSystem, life_system::SnakeLifeSystem,
    life_system::SwordLifeSystem, physics::FirePhysics, physics::HeartPhysics,
    physics::PhysicsSystem, physics::SnakePhysics, physics::SwordPhysics, sprites::Sprite,
    sprites::SpriteImageDef,
};
use ggez::{Context, GameError, GameResult};
use rand::prelude::*;
use std::collections::HashMap;

use super::Chatter;

//A mapping between chat command strings and handlers
pub const COMMAND_MAPPING: [(&str, &dyn GameCommandHandler); 8] = [
    ("#fire", GameCommandHandlers::FIRE),
    ("#snake", GameCommandHandlers::SNAKE),
    ("#snek", GameCommandHandlers::SNAKE),
    ("#sword", GameCommandHandlers::SWORD),
    ("#heart", GameCommandHandlers::HEART),
    ("#random", GameCommandHandlers::RANDOM),
    ("#rand", GameCommandHandlers::RANDOM),
    ("#rng", GameCommandHandlers::RANDOM),
];

//A Command Parser takes chat messages, parses them to find commands and
//then constructs command instances to handle them according to a given
//mapping.
pub struct CommandParser {
    command_map: HashMap<&'static str, &'static dyn GameCommandHandler>,
}

impl CommandParser {
    pub fn new(
        command_mapping: &[(&'static str, &'static dyn GameCommandHandler)],
    ) -> CommandParser {
        CommandParser {
            command_map: command_mapping.iter().cloned().collect(),
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

    pub fn get_commandtype(
        &self,
        command_string: &str,
    ) -> Option<&&'static dyn GameCommandHandler> {
        self.command_map.get(command_string)
    }

    pub fn parse_message(
        &self,
        message: &str,
        chatter: Chatter,
    ) -> Result<Option<CommandInstance>, &'static str> {
        if !message.starts_with('#') {
            return Ok(None);
        }

        let mut parts = message.split(' ');
        if let Some(command) = parts.next() {
            let id = Self::get_id_from_message(parts.next())?;
            match self.get_commandtype(command) {
                Some(command_type) => Ok(Some(CommandInstance {
                    command_handler: *command_type,
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

//A command instance is a single occurence of a command with associated
//chatter, parameters, timestamp or other information.
pub struct CommandInstance {
    pub command_handler: &'static dyn GameCommandHandler,
    pub id: u8,
    pub chatter: Chatter,
}

impl CommandInstance {
    pub fn handle(&self, context: &mut Context, gameworld: &mut GameWorld) -> GameResult {
        self.command_handler
            .handle_command(context, gameworld, &self)
    }
}

pub trait GameCommandHandler {
    #[allow(unused_variables)]
    fn handle_command(
        &self,
        context: &mut Context,
        gameworld: &mut GameWorld,
        command_instance: &CommandInstance,
    ) -> GameResult {
        Err(GameError::ConfigError(String::from(
            "Unimplemented Command Handler.",
        )))
    }
}

//struct to hold a command handler and an incidence count for use in
//the RandomCommandHandler
struct CommandCount {
    command_handler: &'static dyn GameCommandHandler,
    count: u8,
}

//RandomCommandHandler fires a random event from the given set of commands.
//(The set is manually defined to avoid inadvertantly including undesirable
//future commands)
#[derive(Clone)]
pub struct RandomCommandHandler {
    choices: &'static [CommandCount],
}

impl GameCommandHandler for RandomCommandHandler {
    fn handle_command(
        &self,
        context: &mut Context,
        gameworld: &mut GameWorld,
        command_instance: &CommandInstance,
    ) -> GameResult {
        //Calc total size of selection pool
        //This could be run once and cached but maybe not worth it.
        let total_choices: i16 = self.choices.iter().fold(0, |acc, e| acc + e.count as i16);

        //pick random pool index
        let mut pool_index: i16 = thread_rng().gen_range(0, total_choices);

        //let's avoid an Option by starting with the first command
        let mut chosen_handler = self.choices[0].command_handler;

        //find command at chosen pool position
        for choice in self.choices {
            pool_index -= choice.count as i16;
            if pool_index < 0 {
                chosen_handler = choice.command_handler;
                break;
            }
        }

        //Create a new command instance for the random selection and execute it
        let gc = CommandInstance {
            command_handler: chosen_handler,
            id: command_instance.id,
            chatter: command_instance.chatter.clone(),
        };
        gc.handle(context, gameworld)
    }
}

//SpawnEntityCommandHandler is the basis for game object spawning commands.
//Note that most of the associated data is about defining the entity. This
//should probably be moved elsewhere leaving just a type identifier/ref in here.
#[derive(Clone)]
pub struct SpawnEntityCommandHandler {
    pub game_object_type: GameObjectType,
    pub scale: f32,
    pub sprite_def: SpriteImageDef,
    pub physics_system: fn() -> Option<Box<dyn PhysicsSystem>>,
    pub life_system: fn() -> Option<Box<dyn LifeSystem>>,
}

impl GameCommandHandler for SpawnEntityCommandHandler {
    fn handle_command(
        &self,
        context: &mut Context,
        gameworld: &mut GameWorld,
        command_instance: &CommandInstance,
    ) -> GameResult {
        let scale = self.scale;
        let def = &self.sprite_def;
        let sprite: Sprite = Sprite::new(context, def.image_path, def.frames_x, def.frames_y);
        let chatter = command_instance.chatter.clone();
        let label_color = if chatter.is_subscriber {
            chatter.get_color()
        } else {
            ggez::graphics::WHITE
        };
        let label = Some((chatter.name.clone(), label_color));
        let draw_system = GameObjectDrawSystem::new(sprite, label, scale);
        let size = draw_system.get_size().unwrap_or((50.0, 50.0));
        let physics_system = (self.physics_system)();
        let drop_zone_location = gameworld.get_column_coordinates_by_index(command_instance.id);
        let game_object = GameObject::new(
            drop_zone_location.x - size.0 / 2.0,
            drop_zone_location.y - size.1 / 2.0,
            Some(Box::new(draw_system)),
            size.0,
            size.1,
            physics_system,
            true,
            Some(chatter),
            self.game_object_type.clone(),
            (self.life_system)(),
        );
        gameworld.add_game_object(game_object);
        Ok(())
    }
}

//Placeholder for possible player effect commands like slowdown, input reverse
//high gravity and so on.
#[derive(Clone)]
pub struct PlayerEffectCommandHandler {
    pub effect_name: &'static str,
    pub effect_time_ms: u32,
}

impl GameCommandHandler for PlayerEffectCommandHandler {}

pub struct GameCommandHandlers {}

impl GameCommandHandlers {
    const FIRE: &'static dyn GameCommandHandler = &SpawnEntityCommandHandler {
        game_object_type: GameObjectType::Enemy,
        scale: 2.0,
        sprite_def: SpriteImageDef::new("/LargeFlame.png", 4, 1),
        physics_system: || Some(Box::new(FirePhysics::new())),
        life_system: || Some(Box::new(FireLifeSystem::new())),
    };
    const SWORD: &'static dyn GameCommandHandler = &SpawnEntityCommandHandler {
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/item1BIT_sword.png", 1, 1),
        physics_system: || Some(Box::new(SwordPhysics::new())),
        life_system: || Some(Box::new(SwordLifeSystem::new())),
    };
    const SNAKE: &'static dyn GameCommandHandler = &SpawnEntityCommandHandler {
        game_object_type: GameObjectType::Enemy,
        scale: 3.0,
        sprite_def: SpriteImageDef::new("/snake.png", 4, 1),
        physics_system: || Some(Box::new(SnakePhysics::new())),
        life_system: || Some(Box::new(SnakeLifeSystem::new())),
    };
    const HEART: &'static dyn GameCommandHandler = &SpawnEntityCommandHandler {
        game_object_type: GameObjectType::Heart,
        scale: 1.5,
        sprite_def: SpriteImageDef::new("/heart.png", 1, 1),
        physics_system: || Some(Box::new(HeartPhysics::new())),
        life_system: || Some(Box::new(HeartLifeSystem::new())),
    };
    const RANDOM: &'static dyn GameCommandHandler = &RandomCommandHandler {
        choices: &[
            CommandCount {
                command_handler: GameCommandHandlers::HEART,
                count: 1,
            },
            CommandCount {
                command_handler: GameCommandHandlers::FIRE,
                count: 3,
            },
            CommandCount {
                command_handler: GameCommandHandlers::SNAKE,
                count: 3,
            },
            CommandCount {
                command_handler: GameCommandHandlers::SWORD,
                count: 3,
            },
        ],
    };
}
