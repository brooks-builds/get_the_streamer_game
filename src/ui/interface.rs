use std::time::{Duration, Instant};

use crate::running_state::RunningState;

use eyre::Result;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{graphics, timer, Context, GameResult};

use super::{UIComponent, dropzonearea::DropZoneArea, sidebar::SideBar, splash::Splash, uitimer::UITimer};

const DROP_ZONE_HEIGHT: f32 = 50.0;
const TIMER_WIDTH: f32 = 5.0;
const GAME_OVER_FONT_SIZE: f32 = 150.0;

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
            active_timer: Some(UITimer::new(context, start_time, splash_duration, TIMER_WIDTH, screen_height, (0.0, 1.0, 0.0, 1.0))),
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
        self.active_timer = Some(UITimer::new(context, start_time, duration, TIMER_WIDTH, self.height, color));
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
            self.sidebar.get_player_lives(),
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
                self.active_timer.as_ref().unwrap().get_start_time(),
                self.active_timer.as_ref().unwrap().get_duration(),
                self.active_timer.as_ref().unwrap().get_color(),
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
        let sidebar_left:f32 = screen_width - self.sidebar.width();
        let game_area_center_x :f32 = sidebar_left * 0.5;
        let game_area_center_y :f32 = DROP_ZONE_HEIGHT + (screen_coords.h - DROP_ZONE_HEIGHT) * 0.5;

        self.drop_zone_area.draw(context, 0.0, 0.0)?;

        self.sidebar
            .draw(context, sidebar_left, 0.0)?;

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
        let single_drop_zone_width = self.drop_zone_area.width() / self.num_drop_zones as f32;
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
