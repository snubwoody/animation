use ruby_core::GlobalId;
use crate::{impl_layout, Layout, Position, Size};

/// A [`Layout`] that arranges it's children horizontally.
#[derive(Debug,Default)]
pub struct HorizontalLayout{
    id: GlobalId,
    size: Size<f32>,
    position: Position<f32>,
    chilren: Vec<Box<dyn Layout>>
}

impl Layout for HorizontalLayout{
    impl_layout!();
}