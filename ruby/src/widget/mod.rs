mod circle;
mod column;
mod rect;
pub use circle::Circle;
pub use column::Column;
pub use rect::Rect;
use std::fmt::Debug;
use tiny_skia::Pixmap;

pub trait Widget: Debug {
    /// Draw the [`Widget`] onto the screen.
    fn paint(&self, pixmap: &mut Pixmap);
    fn children(&self) -> Vec<Box<dyn Widget>> {
        vec![]
    }
}

pub enum WidgetState {
    Resting,
    Hovered,
    Clicked,
    Focused,
}
