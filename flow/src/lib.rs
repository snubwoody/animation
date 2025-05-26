//! General purpose layout engine
//! 
mod horizontal;
mod block;
mod empty;
use std::fmt::Debug;

use ruby_core::GlobalId;
pub use ruby_core::{Position,Size};
pub use horizontal::HorizontalLayout;
pub use empty::EmptyLayout;
pub use block::BlockLayout;

pub trait Layout: Debug{
    /// Get the layouts id
    fn id(&self) -> GlobalId;
    /// Get the layout's size
    fn size(&self) -> Size<f32>;
    /// Set the layout's size
    fn set_size(&mut self,size: Size<f32>);
    
    /// Get the layout's position
    fn position(&self) -> Position<f32>;
    /// Set the layout's position
    fn set_position(&mut self,position: Position<f32>);

    fn solve_max_constraints(&mut self, max_size: Size<f32>){

    }
}

pub fn solve_layout(layout: &mut impl Layout, max_size: Size<f32>){
    layout.solve_max_constraints(max_size);
}

#[derive(Debug,Clone, Copy,PartialEq, PartialOrd,Default)]
pub enum BoxSizing{
    Flex(u8),
    #[default]
    Fit,
    Fixed(f32)
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



/// Implement the common layout methods
#[macro_export]
macro_rules! impl_layout {
    () => {
        fn id(&self) -> ruby_core::GlobalId{
            self.id
        }

        fn position(&self) -> ruby_core::Position<f32> {
            self.position
        }
    
        fn size(&self) -> ruby_core::Size<f32> {
            self.size
        }
    
        fn set_position(&mut self,position: ruby_core::Position<f32>) {
            self.position = position;
        }
    
        fn set_size(&mut self,size: ruby_core::Size<f32>) {
            self.size = size;
        }
    };
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn default_box_sizing(){
        assert_eq!(BoxSizing::default(),BoxSizing::Fit);
    }
}