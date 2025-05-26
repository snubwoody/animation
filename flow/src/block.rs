use crate::{impl_layout, BoxSizing, BoxConstraints, Layout, Padding};
use ruby_core::{GlobalId, Position, Size};

#[derive(Debug)]
pub struct BlockLayout {
    id: GlobalId,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    pub padding: Padding,
    size: Size<f32>,
    position: Position<f32>,
    child: Box<dyn Layout>,
    constraints: BoxConstraints,
}

impl BlockLayout {
    pub fn new(child: impl Layout + 'static) -> Self {
        Self {
            id: GlobalId::new(),
            intrinsic_width: BoxSizing::default(),
            intrinsic_height: BoxSizing::default(),
            padding: Padding::default(),
            size: Size::default(),
            position: Position::default(),
            child: Box::new(child),
            constraints: BoxConstraints::new(),
        }
    }

    pub fn child(&self) -> &dyn Layout{
        self.child.as_ref()
    }
}

impl Layout for BlockLayout {
    fn solve_max_constraints(&mut self) {
        match self.child.intrinsic_width(){
            BoxSizing::Fixed(width) => {
                self.child.set_max_width(width);
            },
            BoxSizing::Fit | BoxSizing::Flex(_) => {
                let padding = self.padding.left + self.padding.right;
                let width = self.constraints.max_width - padding as f32;
                self.child.set_max_width(width);
            }
        }

        match self.child.intrinsic_height(){
            BoxSizing::Fixed(height) => {
                self.child.set_max_height(height);
            },
            BoxSizing::Fit | BoxSizing::Flex(_) => {
                let padding = self.padding.top + self.padding.bottom;
                let height = self.constraints.max_height - padding as f32;
                self.child.set_max_height(height);
            }
        }

        self.child.solve_max_constraints();
    }

    fn solve_min_contraints(&mut self) -> (f32,f32) {
        let (min_width,min_height) = self.child.solve_min_contraints();

        match self.intrinsic_width(){
            BoxSizing::Flex(_) => self.set_min_width(min_width),
            BoxSizing::Fixed(width) => self.set_min_width(width),
            BoxSizing::Fit => {
                let padding = self.padding.left + self.padding.right;
                let width = min_width + padding as f32;
                self.set_min_width(width);
            }
        }

        match self.intrinsic_height(){
            BoxSizing::Flex(_) => self.set_min_height(min_height),
            BoxSizing::Fixed(height) => self.set_min_height(height),
            BoxSizing::Fit => {
                let padding = self.padding.top + self.padding.bottom;
                let height = min_height + padding as f32;
                self.set_min_height(height);
            }
        }

        (self.constraints.min_width,self.constraints.min_height)
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

        self.child.update_size();
    }

    impl_layout!();
}
