#![feature(random)]
use std::{fmt::Debug, random::random};
mod size;
mod position;
pub use position::Position;
pub use size::Size;

#[derive(Debug,Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct WidgetId(pub i64);

impl WidgetId{
    pub fn new() -> Self { 
        Self::default() 
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        // TODO replace with rand crate
        let id: i64 = random();
        Self(id)
    }
}