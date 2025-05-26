use crate::{BoxSizing, Constraints, Layout, impl_layout};
use ruby_core::{GlobalId, Position, Size};

#[derive(Debug)]
pub struct BlockLayout {
    id: GlobalId,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    size: Size<f32>,
    position: Position<f32>,
    child: Box<dyn Layout>,
    constraints: Constraints,
}

impl BlockLayout {
    pub fn new(child: impl Layout + 'static) -> Self {
        Self {
            id: GlobalId::new(),
            intrinsic_width: BoxSizing::default(),
            intrinsic_height: BoxSizing::default(),
            size: Size::default(),
            position: Position::default(),
            child: Box::new(child),
            constraints: Constraints::new(),
        }
    }
}

impl Layout for BlockLayout {
    fn solve_max_constraints(&mut self, max_size: Size<f32>) {}

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

        self.child.update_size();
    }

    impl_layout!();
}
