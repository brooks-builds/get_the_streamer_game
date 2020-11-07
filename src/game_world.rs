use ggez::{nalgebra::Point2, timer, Context, GameResult};

use crate::{game_object::GameObject, game_object_type::GameObjectType};

pub struct GameWorld {
    width: f32,
    height: f32,
    game_objects: Vec<GameObject>,
    drop_zone_height: f32,
}

impl GameWorld {
    pub fn new(width: f32, height: f32, drop_zone_height: f32) -> GameWorld {
        GameWorld {
            width,
            height,
            game_objects: Vec::new(),
            drop_zone_height,
        }
    }

    pub fn add_game_object(&mut self, obj: GameObject) {
        self.game_objects.push(obj);
    }

    pub fn get_player(&self) -> Option<&GameObject> {
        self.game_objects
            .iter()
            .find(|game_object| game_object.my_type == GameObjectType::Player)
    }

    //@ootsby - 2020-11-04
    //TODO - Have a think about how to best expose the game objects to other systems
    //for inspection or modification. This just returns a ref to the owned vector for now
    //but that seems potentially unwise. Clone won't work because GameObject currently
    //doesn't implement Clone fully.
    pub fn get_game_objects(&self) -> &Vec<GameObject> {
        &self.game_objects
    }

    pub fn get_collidable_game_objects(&self) -> Vec<GameObject> {
        self.game_objects
            .clone()
            .into_iter()
            .filter(|game_object| game_object.collidable)
            .collect()
    }

    pub fn update(&mut self, context: &mut Context) -> GameResult {
        //get the collidables once to save collecting them for each game object
        let collidables = self.get_collidable_game_objects();

        //TODO - This is to keep the borrow checker happy. Perhaps there is a better way.
        let mut game_objects = std::mem::replace(&mut self.game_objects, Vec::new());

        for game_object in game_objects.iter_mut() {
            //I suspect we really want to just pass the GameWorld reference down
            //rather than the list of collidables considering how many entities want
            //to make other queries like find the player.

            if let Err(error) = game_object.update(
                timer::time_since_start(context),
                (self.width, self.height),
                context,
                &collidables,
            ) {
                eprintln!("error running update: {}", error)
            }
        }

        game_objects.retain(|game_object| game_object.is_alive());

        self.game_objects = game_objects;
        Ok(())
    }

    /// Take in an index like 3
    /// which should return the middle x,y coordinates of the corresponding drop zone
    pub fn get_column_coordinates_by_index(&self, index: u8) -> Point2<f32> {
        let single_drop_zone_width = self.width / crate::DROP_ZONE_COUNT as f32;
        Point2::new(
            index as f32 * single_drop_zone_width + single_drop_zone_width / 2.0,
            self.drop_zone_height * 0.5,
        )
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}
