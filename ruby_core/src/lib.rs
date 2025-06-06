#![feature(random)]
use std::{fmt::Debug, random::random};
mod position;
mod size;
pub use position::Position;
pub use size::Size;

// Should this be copy? or even clone?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalId(i32);

impl GlobalId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GlobalId {
    fn default() -> Self {
        // TODO replace with rand crate
        let id = random();
        Self(id)
    }
}
