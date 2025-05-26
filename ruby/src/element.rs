use crate::{Color, Position, Rgba, Size};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Element {
    pub padding: u32,
    pub spacing: f32,
    pub size: Size<f32>,
    pub position: Position<f32>,
    pub color: Color<Rgba>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn layout(&mut self) {
        self.spacing();
    }

    /// Apply the spacing to the children
    fn spacing(&mut self) {
        if self.children.is_empty() {
            return;
        }
        let mut x = self.children[0].position.x;

        for element in &mut self.children {
            element.position.x = x;
            x += element.size.width;
            x += self.spacing;
        }
    }
}

struct BoxConstraints {
    min: Size<f32>,
    max: Size<f32>,
}

enum InstrisicSize {
    Flex(u8),
    Fit,
    Fixed(Size<f32>),
}
