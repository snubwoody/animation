mod circle;
mod rect;
mod column;
use std::{fmt::Debug, random::random};
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

#[derive(Debug,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct WidgetId(pub i64);

impl WidgetId{
    pub fn new() -> Self { 
        Self::default() 
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        // TODO replace with rand crate
        let id: i64 = random();
        Self(id)
    }
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