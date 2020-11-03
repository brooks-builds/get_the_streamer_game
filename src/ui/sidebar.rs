use ggez::{
    graphics,
    graphics::{DrawParam, Image},
    nalgebra::Point2,
    Context,
};

pub struct SideBar {
    width: f32,
    height: f32,
    image: Image,
    heart_image: Image,
    player_lives: u8, //TODO - move this to some sort of GameData state object
}

impl SideBar {
    pub fn new(context: &mut Context, width: f32, height: f32, player_lives: u8) -> SideBar {
        let instruction_image: Image =
            crate::get_image_from_assets(context, String::from("/sidebar.png"));
        SideBar {
            width,
            height,
            image: instruction_image,
            heart_image: Self::create_heart(context),
            player_lives,
        }
    }

    fn create_heart(context: &mut Context) -> Image {
      crate::get_image_from_assets(context, String::from("/heart.png"))
    }

    pub fn set_player_lives(&mut self, player_lives: u8) {
        self.player_lives = player_lives;
    }

    pub fn get_player_lives(&self) -> u8{
        return self.player_lives;
    }

    pub fn draw(&mut self, context: &mut Context, x: f32, y: f32) {
        let w_scale: f32 = self.width / self.image.width() as f32;
        let h_scale: f32 = self.height / self.image.height() as f32;

        let _ = graphics::draw(
            context,
            &self.image,
            DrawParam::default()
                .dest(Point2::new(x, y))
                .scale([w_scale, h_scale]),
        );

        let mut heart_x = x + (self.width * 0.5)
            - (self.heart_image.width() as f32 * self.player_lives as f32) * 0.5;

        for _ in 0..self.player_lives {
            let _ = graphics::draw(
                context,
                &self.heart_image,
                DrawParam::new().dest(Point2::new(heart_x, self.height * 0.95)),
            );

            heart_x += self.heart_image.width() as f32 + 5.0;
        }
    }

    pub fn width(&self) -> f32 {
        return self.width;
    }

    pub fn height(&self) -> f32 {
        return self.height;
    }
}
