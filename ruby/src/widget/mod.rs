mod circle;
mod rect;
mod column;
use std::fmt::Debug;
pub use column::{Column};
pub use rect::Rect;
pub use circle::Circle;
use tiny_skia::Pixmap;


pub trait Widget: Debug {
    /// Draw the [`Widget`] onto the screen.
    fn paint(&self,pixmap: &mut Pixmap);
    fn children(&self) -> Vec<Box<dyn Widget>>{
        vec![]
    }
}

pub enum WidgetState{
    Resting,
    Hovered,
    Clicked,
    Focused,
}