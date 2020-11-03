use std::time::{Duration, Instant};

use crate::running_state::RunningState;

use crate::splash::Splash;
use eyre::Result;
use ggez::graphics::{Color, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, timer, Context, GameResult};
use graphics::Image;

const DROP_ZONE_HEIGHT: f32 = 50.0;
const GAME_OVER_FONT_SIZE: f32 = 150.0;

struct DropZoneArea {
    width: f32,
    height: f32,
    num_zones: u8,
    drop_zones: Vec<Rect>,
    drop_zone_background: Mesh,
    drop_zone_labels: Vec<Text>,
}

impl DropZoneArea {
    fn new(context: &mut Context, num_zones: u8, width: f32, height: f32) -> DropZoneArea {
        let single_drop_zone_width = width / num_zones as f32;
        let drop_zones = Self::create_drop_zones(num_zones, single_drop_zone_width, height);
        let first_zone_bound: Rect = drop_zones[0].clone();

        DropZoneArea {
            width,
            height,
            num_zones,
            drop_zones,
            drop_zone_background: Self::create_drop_zone_background(context, first_zone_bound),
            drop_zone_labels: Self::create_drop_zone_labels(num_zones, single_drop_zone_width),
        }
    }

    fn create_drop_zone_background(context: &mut Context, rect: Rect) -> Mesh {
        MeshBuilder::new()
            .rectangle(DrawMode::stroke(1.0), rect, graphics::WHITE)
            .build(context)
            .unwrap()
    }

    fn create_drop_zone_labels(num_zones: u8, drop_zone_width: f32) -> Vec<Text> {
        let mut labels: Vec<Text> = vec![];
        for count in 0..num_zones {
            let mut label = Text::new(format!("{}", count));

            label.set_bounds(
                Point2::new(drop_zone_width, DROP_ZONE_HEIGHT),
                graphics::Align::Center,
            );
            label.set_font(Font::default(), Scale::uniform(50.0));
            labels.push(label);
        }
        labels
    }

    fn create_drop_zones(num_zones: u8, drop_zone_width: f32, drop_zone_height: f32) -> Vec<Rect> {
        let mut drop_zones = vec![];

        for count in 0..num_zones {
            let drop_zone = Rect::new(
                count as f32 * drop_zone_width,
                0.0,
                drop_zone_width,
                drop_zone_height,
            );
            drop_zones.push(drop_zone);
        }

        return drop_zones;
    }

    fn draw_drop_zones(&self, context: &mut Context) -> GameResult<()> {
        self.drop_zones
            .iter()
            .try_for_each(|drop_zone: &Rect| -> GameResult<()> {
                graphics::draw(
                    context,
                    &self.drop_zone_background,
                    DrawParam::new().dest(Point2::new(drop_zone.x, drop_zone.y)),
                )?;
                Ok(())
            })?;

        let single_drop_zone_width = self.width / self.num_zones as f32;
        self.drop_zone_labels.iter().enumerate().try_for_each(
            |(index, label)| -> GameResult<()> {
                let label_height = label.height(context) as f32;
                graphics::draw(
                    context,
                    label,
                    DrawParam::new().dest(Point2::new(
                        index as f32 * single_drop_zone_width,
                        self.height / 2.0 - label_height / 2.0,
                    )),
                )
            },
        )?;

        Ok(())
    }
}

struct SideBar {
    width: f32,
    height: f32,
    image: Image,
    heart_image: Image,
    player_lives: u8,
}

impl SideBar {
    fn new(context: &mut Context, width: f32, height: f32, player_lives: u8) -> SideBar {
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
        Image::new(context, "/heart.png").unwrap()
    }

    fn set_player_lives(&mut self, player_lives: u8) {
        self.player_lives = player_lives;
    }

    fn draw(&mut self, context: &mut Context, x: f32, y: f32) {
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
}

struct UITimer {
    duration: Duration,
    width: f32,
    height: f32,
    mesh: Mesh,
    start_time: Instant,
    color: (f32, f32, f32, f32),
}

impl UITimer {
    //constructor assumes immediate timer start
    fn new(
        context: &mut Context,
        start_time: Instant,
        duration: Duration,
        width: f32,
        height: f32,
        color: (f32, f32, f32, f32),
    ) -> UITimer {
        UITimer {
            duration,
            width,
            height,
            mesh: Self::create_mesh(context, width, height, color),
            start_time,
            color,
        }
    }

    fn get_remaining(&self) -> Duration {
        self.duration
            .clone()
            .checked_sub(self.start_time.elapsed())
            .unwrap_or(Duration::new(0, 0))
    }

    fn create_mesh(
        context: &mut Context,
        width: f32,
        height: f32,
        color: (f32, f32, f32, f32),
    ) -> Mesh {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, width, height),
                Color::new(color.0, color.1, color.2, color.3),
            )
            .build(context)
            .unwrap();
        mesh
    }

    fn draw(&self, context: &mut Context, x: f32, y: f32) -> GameResult {
        let elapsed_fraction = self.get_remaining().as_secs_f32() / self.duration.as_secs_f32();
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::default()
                .offset([0.0, self.height])
                .dest(Point2::new(x, y))
                .scale([1.0_f32, elapsed_fraction]),
        )
    }

    fn update(&self, _time_since_start: std::time::Duration, _context: &mut Context) {}
}

pub struct Interface {
    width: f32,
    height: f32,
    pub sidebar_width: f32, //TODO - get rid of this public
    num_drop_zones: u8,
    start_time:Instant,
    splash_duration:Duration,
    active_timer: Option<UITimer>,
    drop_zone_area: DropZoneArea,
    sidebar: SideBar,
    full_mask: Mesh,
    pub splash: Splash,
}

impl Interface {
    const SIDEBAR_PCT: f32 = 0.2;

    pub fn new(
        context: &mut Context,
        player_lives: u8,
        num_drop_zones: u8,
        splash_duration: Duration,
    ) -> GameResult<Interface> {
        let screen_coords = ggez::graphics::screen_coordinates(context);
        let screen_width = screen_coords.w;
        let screen_height = screen_coords.h;
        let sidebar_width = (screen_width * Self::SIDEBAR_PCT).floor();
        let drop_zone_area_width = screen_width - sidebar_width;
        let splash = Self::create_splash(context, screen_width, screen_height);
        let start_time = Instant::now();

        let ret = Interface {
            width: screen_width,
            height: screen_height,
            sidebar_width,
            num_drop_zones,
            start_time,
            splash_duration,
            drop_zone_area: DropZoneArea::new(
                context,
                num_drop_zones,
                drop_zone_area_width,
                DROP_ZONE_HEIGHT,
            ),
            active_timer: Some(UITimer::new(context, start_time, splash_duration,5.0, screen_height, (0.0, 1.0, 0.0, 1.0))),
            sidebar: SideBar::new(context, sidebar_width, screen_height, player_lives),
            full_mask: Self::create_full_mask(context, screen_width, screen_height),
            splash,
        };
        Ok(ret)
    }

    pub fn splash_is_done(&self) -> bool {
        self.start_time.elapsed() > self.splash_duration
    }

    pub fn set_timer(
        &mut self,
        context: &mut Context,
        start_time: Instant,
        duration: Duration,
        color: (f32, f32, f32, f32),
    ) {
        self.active_timer = Some(UITimer::new(context, start_time, duration, 5.0, self.height, color));
    }

    pub fn update_screen_size(
        &mut self,
        context: &mut Context,
        screen_width: f32,
        screen_height: f32,
    ) {
        self.width = screen_width;
        self.height = screen_height;
        self.sidebar_width = (screen_width * Self::SIDEBAR_PCT).floor();
        let drop_zone_area_width = screen_width - self.sidebar_width;

        self.drop_zone_area = DropZoneArea::new(
            context,
            self.num_drop_zones,
            drop_zone_area_width,
            DROP_ZONE_HEIGHT,
        );

        self.sidebar = SideBar::new(
            context,
            self.sidebar_width,
            screen_height,
            self.sidebar.player_lives,
        );

        if !self.splash_is_done(){
            self.splash =  Self::create_splash(context, screen_width, screen_height);
        }

        //TODO - this is pretty ugly. Look into avoiding all the as_ref and unwrapping.
        //Perhaps a default elapsed UITimer would be better than an Option

        let replace_timer = match self.active_timer {
            Some(_) => true,
            None => false,
        };

        if replace_timer {
            self.set_timer(
                context,
                self.active_timer.as_ref().unwrap().start_time.clone(),
                self.active_timer.as_ref().unwrap().duration.clone(),
                self.active_timer.as_ref().unwrap().color,
            )
        }

        self.full_mask = Self::create_full_mask(context, screen_width, screen_height);

        // let mut game_over_text = Text::new("Game Over!");
        // game_over_text.set_font(Font::default(), Scale::uniform(GAME_OVER_FONT_SIZE));
        // game_over_text.set_bounds(Point2::new(screen_width, screen_height), Align::Center);

        // let mut attack_subtitle = Text::new("Attack the Streamer with following commands:");
        // attack_subtitle.set_font(Font::default(), Scale::uniform(30.0));
        // attack_subtitle.set_bounds(Point2::new(width, 60.0), Align::Center);

        // let mut help_subtitle = Text::new("Help the Streamer with following commands:");
        // help_subtitle.set_font(Font::default(), Scale::uniform(30.0));
        // help_subtitle.set_bounds(Point2::new(width, 60.0), Align::Center);

        // let mut player_lives_left_subtitle = Text::new("Streamer Lives Left:");
        // player_lives_left_subtitle.set_font(Font::default(), Scale::uniform(30.0));
        // player_lives_left_subtitle.set_bounds(Point2::new(width, 60.0), Align::Center);

        // let mut game_over_title = Text::new("Game Over!");
        // game_over_title.set_font(Font::default(), Scale::uniform(50.0));
    }

    fn create_splash(context: &mut Context, screen_width: f32, screen_height: f32)-> Splash{
        Splash::new(context, screen_width*0.6, screen_height*0.2)
    }

    fn create_full_mask(context: &mut Context, screen_width: f32, screen_height: f32) -> Mesh {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, screen_width, screen_height),
                Color::new(0.0, 0.0, 0.0, 0.9),
            )
            .build(context)
            .unwrap()
    }

    pub fn draw(&mut self, context: &mut Context, running_state: &RunningState) -> GameResult<()> {
        let screen_coords = ggez::graphics::screen_coordinates(context);
        let screen_width = screen_coords.w;
        let sidebar_left:f32 = screen_width - self.sidebar.width;
        let game_area_center_x :f32 = sidebar_left * 0.5;
        let game_area_center_y :f32 = DROP_ZONE_HEIGHT + (screen_coords.h - DROP_ZONE_HEIGHT) * 0.5;

        self.drop_zone_area.draw_drop_zones(context)?;

        self.sidebar
            .draw(context, sidebar_left, 0.0);

        let _ = match &self.active_timer {
            Some(timer) => timer.draw(context, sidebar_left, 0.0),
            None => Ok(()),
        };
        if *running_state == RunningState::StartingSoon {
            let _ = self.splash.draw(context, game_area_center_x, game_area_center_y);
        }

        if running_state.is_game_over() {
            graphics::draw(context, &self.full_mask, DrawParam::new())?;
        }

        Ok(())
    }

    /// Take in an index like 3
    /// which should return the middle x,y coordinates of the corresponding drop zone
    pub fn get_column_coordinates_by_index(&self, index: u8) -> Point2<f32> {
        let single_drop_zone_width = self.drop_zone_area.width / self.num_drop_zones as f32;
        Point2::new(
            index as f32 * single_drop_zone_width + single_drop_zone_width / 2.0,
            DROP_ZONE_HEIGHT / 2.0,
        )
    }

    pub fn update(&mut self, context: &mut Context, player_lives: u8) -> Result<()> {
        let time_since_start = timer::time_since_start(context);
        self.sidebar.set_player_lives(player_lives);

        let _ = match &self.active_timer {
            Some(timer) => timer.update(time_since_start, context),
            None => (),
        };

        Ok(())
    }
}
