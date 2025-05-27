use crate::{BoxConstraints, BoxSizing, Layout, Padding, Position, Size, impl_layout};
use ruby_core::GlobalId;

/// A [`Layout`] that arranges it's children horizontally.
#[derive(Debug, Default)]
pub struct HorizontalLayout {
    id: GlobalId,
    size: Size<f32>,
    position: Position<f32>,
    children: Vec<Box<dyn Layout>>,
    pub spacing: u32,
    pub padding: Padding,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    constraints: BoxConstraints,
}

impl HorizontalLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, child: impl Layout + 'static) {
        self.children.push(Box::new(child));
    }
}

impl Layout for HorizontalLayout {
    fn solve_min_contraints(&mut self) -> (f32, f32) {
        let mut min_width = 0.0;
        let mut min_height = 0.0;

        for child in &mut self.children {
            let constraints = child.solve_min_contraints();
            min_width += constraints.0;
            min_height += constraints.1;
        }

        // TODO add padding
        match self.intrinsic_width {
            BoxSizing::Fit | BoxSizing::Flex(_) => self.set_min_width(min_width),
            BoxSizing::Fixed(width) => self.set_min_width(width),
        }

        match self.intrinsic_height {
            BoxSizing::Fit | BoxSizing::Flex(_) => self.set_min_height(min_height),
            BoxSizing::Fixed(height) => self.set_min_height(height),
        }

        (min_width, min_height)
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

        self.children.iter_mut().for_each(|c| c.update_size());
    }

    impl_layout!();
}
