use ggez::{
    graphics::{self, DrawParam},
    Context, GameError, GameResult,
};

use crate::game_world::GameWorld;

use super::UIComponent;

pub struct GamePlayWindow {
    width: f32,
    height: f32,
}

impl GamePlayWindow {
    pub fn new(width: f32, height: f32) -> GamePlayWindow {
        GamePlayWindow { width, height }
    }

    pub fn draw_gameworld(
        &self,
        context: &mut Context,
        x: f32,
        y: f32,
        gameworld: &GameWorld,
    ) -> GameResult {
        //apply transform from gameworld space to screen space
        //Note that this isn't fully working (Suspected ggez bug) and there's
        //some patch code in the game_object_draw_system for labels.
        let draw_param = DrawParam::default()
            .scale([
                self.width / gameworld.width(),
                self.height / gameworld.height(),
            ])
            .dest([x, y]);
        let m = draw_param.to_matrix();
        graphics::push_transform(context, Some(m));
        let _ = graphics::apply_transformations(context);

        //render all the objects
        for game_object in gameworld.get_game_objects().iter() {
            //get_game_objects() {
            game_object.draw(context)?;
        }

        //pop our window transform and return rendering to previous state
        graphics::pop_transform(context);
        let _ = graphics::apply_transformations(context);
        Ok(())
    }

    pub fn update_screen_size(&mut self, _context: &mut Context, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

//I've implemented the UIComponent trait because in theory this should be one.
//However, right now it's not clear how best to deal with the need for the
//window to use the gameworld when rendering so the draw doesn't do anything.
//@ootsby - 2020-11-04
impl UIComponent for GamePlayWindow {
    fn width(&self) -> f32 {
        self.width
    }
    fn height(&self) -> f32 {
        self.height
    }
    fn draw(&self, _context: &mut Context, _x: f32, _y: f32) -> GameResult {
        Err(GameError::RenderError(String::from(
            "UIComponent draw not implemented for GameWindow. Use draw_gameworld.",
        )))
    }
}
