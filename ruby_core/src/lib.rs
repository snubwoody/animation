use std::fmt::Debug;
mod position;
mod size;
pub use position::Position;
pub use size::Size;

// Should this be copy? or even clone?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId(i32);

impl GlobalId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GlobalId {
    fn default() -> Self {
        // TODO replace with rand crate
        let id = rand::random();
        Self(id)
    }
}
