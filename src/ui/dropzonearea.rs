use ggez::{
    graphics::{self, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text},
    nalgebra::Point2,
    Context, GameResult,
};

use super::UIComponent;

pub struct DropZoneArea {
    width: f32,
    height: f32,
    num_zones: u8,
    drop_zones: Vec<Rect>,
    drop_zone_background: Mesh,
    drop_zone_labels: Vec<Text>,
}

impl DropZoneArea {
    pub fn new(context: &mut Context, num_zones: u8, width: f32, height: f32) -> DropZoneArea {
        let single_drop_zone_width = width / num_zones as f32;
        let drop_zones = Self::create_drop_zones(num_zones, single_drop_zone_width, height);
        let first_zone_bound: Rect = drop_zones[0];

        DropZoneArea {
            width,
            height,
            num_zones,
            drop_zones,
            drop_zone_background: Self::create_drop_zone_background(context, first_zone_bound),
            drop_zone_labels: Self::create_drop_zone_labels(
                num_zones,
                single_drop_zone_width,
                height,
            ),
        }
    }

    fn create_drop_zone_background(context: &mut Context, rect: Rect) -> Mesh {
        MeshBuilder::new()
            .rectangle(DrawMode::stroke(1.0), rect, graphics::WHITE)
            .build(context)
            .unwrap()
    }

    fn create_drop_zone_labels(
        num_zones: u8,
        drop_zone_width: f32,
        drop_zone_height: f32,
    ) -> Vec<Text> {
        let mut labels: Vec<Text> = vec![];
        for count in 0..num_zones {
            let mut label = Text::new(format!("{}", count));

            label.set_bounds(
                Point2::new(drop_zone_width, drop_zone_height),
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

        drop_zones
    }
}

impl UIComponent for DropZoneArea {
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }
    fn draw(&self, context: &mut Context, x: f32, y: f32) -> GameResult {
        self.drop_zones
            .iter()
            .try_for_each(|drop_zone: &Rect| -> GameResult<()> {
                graphics::draw(
                    context,
                    &self.drop_zone_background,
                    DrawParam::new().dest(Point2::new(drop_zone.x + x, drop_zone.y + y)),
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
                        x + index as f32 * single_drop_zone_width,
                        y + self.height / 2.0 - label_height / 2.0,
                    )),
                )
            },
        )?;

        Ok(())
    }
}
