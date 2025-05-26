use ruby_core::GlobalId;

use super::Widget;

/// A [`Column`] is a [`Widget`] that arranges it children
/// vertically, one after the other.
#[derive(Debug, Default)]
pub struct Column {
    id: GlobalId,
    children: Vec<Box<dyn Widget>>,
    spacing: f32,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_child(&mut self, widget: impl Widget + 'static) {
        self.children.push(Box::new(widget));
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for Column {
    fn paint(&self, pixmap: &mut tiny_skia::Pixmap) {
        for child in &self.children {
            child.paint(pixmap);
        }
    }
}

/// A [`Column`] is a [`Widget`] that aligns it's children
/// vertically one after another.
///
/// # Example
/// ```
/// use ruby::{column,widget::Rect};
///
/// let column = column![
///     Rect::new(),
///     Rect::new(),
/// ];
/// ```
///
#[macro_export]
macro_rules! column {
    ($($widget: expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut column = crate::widget::Column::new();
            $(
                column.add_child($widget);
            )*
            column
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::widget::Rect;

    #[test]
    fn macro_expansion() {
        let column = column![Rect::new(), Rect::new(), Rect::new(),];

        assert_eq!(column.children.len(), 3)
    }
}
