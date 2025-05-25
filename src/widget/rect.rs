use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};
use crate::{Position, Size};

use super::Widget;

/// A rectangular primitive
#[derive(Default,Clone, Copy,PartialEq,Debug)]
pub struct Rect{
    pub position: Position<f32>,
    pub size: Size<f32>
}

impl Rect{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn position(mut self,x: f32, y: f32) -> Self{
        self.position = Position{x,y};
        self
    }

    pub fn size(mut self,width: f32,height: f32) -> Self{
        self.size = Size{width,height};
        self
    }

    pub fn width(mut self,width: f32) -> Self{
        self.size.width = width;
        self
    }

    pub fn height(mut self,height: f32) -> Self{
        self.size.height = height;
        self
    }
}

impl Widget for Rect{
    fn paint(&self,pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(50, 107,160,255));
        paint.anti_alias = true;

        let Position { x, y } = self.position;
        let Size { width, height } = self.size;
        
        let rect = tiny_skia::Rect::from_xywh(x, y, width, height).unwrap();
        let path = PathBuilder::from_rect(rect);

        pixmap.fill_path(
            &path, 
            &paint, 
            FillRule::Winding, 
            Transform::identity(), 
            None
        );
    }
}