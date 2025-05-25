mod circle;
mod rect;
mod column;
use std::fmt::Debug;
pub use column::Column;
pub use rect::Rect;
pub use circle::Circle;
use tiny_skia::Pixmap;


pub trait Widget: Debug {
    /// Draw the [`Widget`] onto the screen.
    fn paint(&self,pixmap: &mut Pixmap);
}

pub enum MainAxisAlignment{
    Start,
    Center,
    End,
    SpaceEvenly,
    SpaceBetween
}

pub enum CrossAxisAlignment{
    Start,
    Center,
    End
}

pub enum WidgetState{
    Resting,
    Hovered,
    Clicked,
    Focused,
}