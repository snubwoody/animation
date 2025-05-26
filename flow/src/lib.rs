//! General purpose layout engine
//!
//! ## Constraints
//! [`Constraints`] control the max and min sizing that a layout is
//! allowed to be. Constraints are set by the parent and are respected
//! by the widget when doing the final layout calculation.
//!
mod block;
mod empty;
mod horizontal;
use std::fmt::Debug;

pub use block::BlockLayout;
pub use empty::EmptyLayout;
pub use horizontal::HorizontalLayout;
use ruby_core::GlobalId;
pub use ruby_core::{Position, Size};

pub trait Layout: Debug {
    /// Get the layouts id
    fn id(&self) -> GlobalId;

    /// Get the layout's size
    fn size(&self) -> Size<f32>;
    /// Set the layout's size
    fn set_size(&mut self, size: Size<f32>);

    /// Get the layout's position
    fn position(&self) -> Position<f32>;
    /// Set the layout's position
    fn set_position(&mut self, position: Position<f32>);

    /// Get the layouts instrinsic width
    fn intrinsic_width(&self) -> BoxSizing;
    /// Get the layouts instrinsic height
    fn intrinsic_height(&self) -> BoxSizing;

    /// Set the layout's max width
    fn set_max_width(&mut self, width: f32);
    /// Set the layout's max height
    fn set_max_height(&mut self, height: f32);
    /// Set the layout's min width
    fn set_min_width(&mut self, width: f32);
    /// Set the layout's min height
    fn set_min_height(&mut self, height: f32);

    fn solve_max_constraints(&mut self, max_size: Size<f32>) {}

    /// Update the size of the layout after the contraints have been
    /// solved, and any child layouts
    fn update_size(&mut self) {}
}

pub fn solve_layout(layout: &mut impl Layout, max_size: Size<f32>) {
    // Solve the max constraints for the root layout
    match layout.intrinsic_width() {
        BoxSizing::Fit | BoxSizing::Flex(_) => {
            layout.set_max_width(max_size.width);
        }
        BoxSizing::Fixed(width) => {
            layout.set_max_width(width);
        }
    }

    match layout.intrinsic_height() {
        BoxSizing::Fit | BoxSizing::Flex(_) => {
            layout.set_max_height(max_size.height);
        }
        BoxSizing::Fixed(height) => {
            layout.set_max_height(height);
        }
    }

    // FIXME set the max size to the root constraints
    layout.solve_max_constraints(max_size);
    layout.update_size();
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Constraints {
    max_width: f32,
    max_height: f32,
    min_height: f32,
    min_width: f32,
}

impl Constraints {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum BoxSizing {
    Flex(u8),
    #[default]
    Fit,
    Fixed(f32),
}

pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceEvenly,
    SpaceBetween,
}

pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
}

/// Implement the common layout methods
#[macro_export]
macro_rules! impl_layout {
    () => {
        fn id(&self) -> ruby_core::GlobalId {
            self.id
        }

        fn position(&self) -> ruby_core::Position<f32> {
            self.position
        }

        fn size(&self) -> ruby_core::Size<f32> {
            self.size
        }

        fn set_position(&mut self, position: ruby_core::Position<f32>) {
            self.position = position;
        }

        fn set_size(&mut self, size: ruby_core::Size<f32>) {
            self.size = size;
        }

        fn intrinsic_height(&self) -> crate::BoxSizing {
            self.intrinsic_height
        }

        fn intrinsic_width(&self) -> crate::BoxSizing {
            self.intrinsic_width
        }

        fn set_max_height(&mut self, height: f32) {
            self.constraints.max_height = height;
        }

        fn set_max_width(&mut self, width: f32) {
            self.constraints.max_width = width;
        }

        fn set_min_height(&mut self, height: f32) {
            self.constraints.min_height = height;
        }

        fn set_min_width(&mut self, width: f32) {
            self.constraints.min_width = width;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_box_sizing() {
        assert_eq!(BoxSizing::default(), BoxSizing::Fit);
    }
}
