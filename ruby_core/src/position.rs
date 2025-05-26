
/// Represents the `x` and `y` position of any structure
#[derive(Debug,Clone, Copy,Default,PartialEq, PartialOrd)]
pub struct Position<N>{
    pub x: N,
    pub y: N
}

impl<N> Position<N>{
    pub fn new(x: N, y: N) -> Self{
        Self{x, y}
    }
}