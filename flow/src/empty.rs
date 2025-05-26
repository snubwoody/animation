use crate::{BoxConstraints, BoxSizing, Layout, Position, Size, impl_layout};
use ruby_core::GlobalId;

#[derive(Debug, Default)]
pub struct EmptyLayout {
    id: GlobalId,
    size: Size<f32>,
    position: Position<f32>,
    constraints: BoxConstraints,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
}

impl EmptyLayout {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Layout for EmptyLayout {
    fn solve_min_contraints(&mut self) -> (f32, f32) {
        match self.intrinsic_width {
            BoxSizing::Fit | BoxSizing::Flex(_) => self.set_min_width(0.0),
            BoxSizing::Fixed(width) => self.set_min_width(width),
        }

        match self.intrinsic_height {
            BoxSizing::Fit | BoxSizing::Flex(_) => self.set_min_height(0.0),
            BoxSizing::Fixed(height) => self.set_min_height(height),
        }

        (self.constraints.min_width, self.constraints.min_height)
    }

    fn update_size(&mut self) {
        match self.intrinsic_width {
            BoxSizing::Fit => self.size.width = self.constraints.min_width,
            BoxSizing::Fixed(width) => self.size.width = width,
            BoxSizing::Flex(_) => self.size.width = self.constraints.max_width,
        }

        match self.intrinsic_height {
            BoxSizing::Fit => self.size.height = self.constraints.min_height,
            BoxSizing::Fixed(height) => self.size.height = height,
            BoxSizing::Flex(_) => self.size.height = self.constraints.max_height,
        }
    }

    impl_layout!();
}
