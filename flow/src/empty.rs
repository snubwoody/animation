use ruby_core::GlobalId;
use crate::{impl_layout, BoxSizing, Layout,Size,Position};

#[derive(Debug,Default)]
pub struct EmptyLayout{
    id: GlobalId,
    intrinsic_width: BoxSizing,
    intrinsic_height: BoxSizing,
    size: Size<f32>,
    position: Position<f32>,
}

impl EmptyLayout{
    pub fn new() -> Self{
        Self::default()
    }
}

impl Layout for EmptyLayout{
    impl_layout!();
}