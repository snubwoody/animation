use super::Widget;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};

#[derive(Default, Debug, Clone, Copy)]
pub struct Circle {
    position: [f32; 2],
    radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self {
            position: [x, y],
            radius,
        }
    }
}

impl Widget for Circle {
    fn paint(&self, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(50, 107, 160, 255));
        paint.anti_alias = true;

        let [mut x, mut y] = self.position;
        // Move the origin to the center
        x += self.radius;
        y += self.radius;

        let path = PathBuilder::from_circle(x, y, self.radius).unwrap();
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}
