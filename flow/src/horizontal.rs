use ruby_core::GlobalId;
use crate::{impl_layout, BoxSizing, Constraints, Layout, Position, Size};

/// A [`Layout`] that arranges it's children horizontally.
#[derive(Debug,Default)]
pub struct HorizontalLayout{
    id: GlobalId,
    size: Size<f32>,
    position: Position<f32>,
    children: Vec<Box<dyn Layout>>,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    constraints: Constraints
}

impl Layout for HorizontalLayout{
    impl_layout!();
}