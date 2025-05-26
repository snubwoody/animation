use ruby_core::{GlobalId, Position, Size};
use crate::{impl_layout, BoxSizing, Layout};

#[derive(Debug)]
pub struct BlockLayout{
    id: GlobalId,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    size: Size<f32>,
    position: Position<f32>,
    child: Box<dyn Layout>
}

impl BlockLayout{
    pub fn new(child: impl Layout + 'static ) -> Self{
        Self { 
            id: GlobalId::new(),
            intrinsic_width: BoxSizing::default(), 
            intrinsic_height: BoxSizing::default(), 
            size: Size::default(), 
            position: Position::default(), 
            child: Box::new(child) 
        }
    }
}

impl Layout for BlockLayout{
    fn solve_max_constraints(&mut self,max_size: Size<f32>){
        match self.intrinsic_width {
            BoxSizing::Flex(factor) => {
                self.size.width = max_size.width;
            },
            BoxSizing::Fit => {},
            BoxSizing::Fixed(width) => {}
        }

        match self.intrinsic_height {
            BoxSizing::Flex(factor) => {
                self.size.height = max_size.height;
            },
            BoxSizing::Fit => {},
            BoxSizing::Fixed(height) => {}
        }
    }

    impl_layout!();
}