mod circle;
mod rect;
pub use rect::Rect;
pub use circle::Circle;
use tiny_skia::Pixmap;


pub trait Widget {
    /// Draw the [`Widget`] onto the screen.
    fn paint(&self,pixmap: &mut Pixmap);
}

pub enum WidgetState{
    Resting,
    Hovered,
    Clicked,
    Focused,
}