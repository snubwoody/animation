use crate::{BoxSizing, Constraints, Layout, Position, Size, impl_layout};
use ruby_core::GlobalId;

/// A [`Layout`] that arranges it's children horizontally.
#[derive(Debug, Default)]
pub struct HorizontalLayout {
    id: GlobalId,
    size: Size<f32>,
    position: Position<f32>,
    children: Vec<Box<dyn Layout>>,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    constraints: Constraints,
}

impl Layout for HorizontalLayout {
    fn solve_min_contraints(&mut self) -> (f32,f32) {
        (0.0,0.0)
    }
    impl_layout!();
}
