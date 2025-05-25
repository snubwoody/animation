use super::Widget;

/// A [`Column`] is a [`Widget`] that arranges it children
/// vertically, one after the other.
#[derive(Debug,Default)]
pub struct Column{
    children: Vec<Box<dyn Widget>>,
    spacing: f32
}

impl Column{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn add_child(mut self,widget: impl Widget + 'static) -> Self{
        self.children.push(Box::new(widget));
        self
    }

    pub fn spacing(mut self,spacing: f32) -> Self{
        self.spacing = spacing;
        self
    }
}

macro_rules! column {
    ($($child: expr),*) => {
        {
            let mut column = Column::new();

        }
    };
}

#[cfg(test)]
mod tests{
    #[test]
    fn spacing(){

    }
}