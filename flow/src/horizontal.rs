use crate::{
    BoxConstraints, BoxSizing, Layout, Padding, Position, Size, impl_layout, impl_padding,
    impl_size,
};
use ruby_core::GlobalId;

/// A [`Layout`] that arranges it's children horizontally.
///
/// # Example
/// ```
/// use flow::{HorizontalLayout,EmptyLayout,Size,BoxSizing,Layout};
///
/// let mut child1 = EmptyLayout::new();
/// let mut child2 = EmptyLayout::new();
///
/// child1.intrinsic_width = BoxSizing::Fixed(200.0);
/// child2.intrinsic_width = BoxSizing::Fixed(300.0);
///
/// let mut layout = HorizontalLayout::new();
/// layout.push(child1);
/// layout.push(child2);
///
/// flow::solve_layout(&mut layout,Size::unit(1000.0));
/// assert_eq!(layout.size().width,500.0);
/// ```
///
#[derive(Debug, Default)]
pub struct HorizontalLayout {
    id: GlobalId,
    size: Size<f32>,
    position: Position<f32>,
    children: Vec<Box<dyn Layout>>,
    pub spacing: u32,
    pub padding: Padding,
    pub intrinsic_width: BoxSizing,
    pub intrinsic_height: BoxSizing,
    constraints: BoxConstraints,
}

impl HorizontalLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn children(&self) -> &[Box<dyn Layout>]{
        self.children.as_slice()
    }

    pub fn push(&mut self, child: impl Layout + 'static) {
        self.children.push(Box::new(child));
    }

    /// Append a list of layouts
    /// 
    /// # Example
    /// ```
    /// use flow::{HorizontalLayout,EmptyLayout};
    /// 
    /// let mut layout = HorizontalLayout::new();
    /// layout.append([EmptyLayout::new(),HorizontalLayout::new()]);
    /// 
    /// assert_eq!(layout.children().len(),3);
    /// ```
    pub fn append<I>(&mut self, children:I)
    where 
        I: IntoIterator<Item: Layout + 'static>, 
    {
        for child in children{
            self.children.push(Box::new(child));
        }
    }

    /// Calculate the total width of the children with
    /// fixed `instrinsic_width`
    fn sum_fixed_width(&self) -> f32 {
        let mut sum = 0.0;
        for child in &self.children {
            if let BoxSizing::Fixed(width) = child.intrinsic_width() {
                sum += width
            }
        }
        sum
    }

    /// Calculate the total flex factor across all the
    /// children
    fn flex_total(&self) -> u8 {
        // TODO handle overflow?
        let mut flex_total = 0;
        for child in &self.children {
            if let BoxSizing::Flex(flex) = child.intrinsic_width() {
                flex_total += flex
            }
        }
        flex_total
    }

    impl_size!();
    impl_padding!();
}

impl Layout for HorizontalLayout {
    fn solve_max_constraints(&mut self) {
        let flex_total = self.flex_total();
        let total_width = self.sum_fixed_width();
        let max_width = self.constraints.max_width - total_width;

        for child in &mut self.children {
            match child.intrinsic_width() {
                BoxSizing::Fit => {
                    child.set_max_width(child.constraints().min_width);
                }
                BoxSizing::Flex(flex) => {
                    let factor = flex as f32 / flex_total as f32;
                    let width = max_width * factor;
                    child.set_max_width(width);
                }
                BoxSizing::Fixed(width) => child.set_max_width(width),
            }

            match child.intrinsic_height() {
                BoxSizing::Fit => {
                    child.set_max_height(child.constraints().min_height);
                }
                BoxSizing::Flex(_) => {
                    child.set_max_height(self.constraints.max_height);
                }
                BoxSizing::Fixed(height) => child.set_max_height(height),
            }

            child.solve_max_constraints();
        }
    }

    fn solve_min_constraints(&mut self) -> (f32, f32) {
        let mut min_width = 0.0;
        let mut min_height = 0.0;

        for child in &mut self.children {
            let constraints = child.solve_min_constraints();
            min_width += constraints.0;
            min_height += constraints.1;
        }

        // TODO add padding
        match self.intrinsic_width {
            BoxSizing::Fit | BoxSizing::Flex(_) => self.set_min_width(min_width),
            BoxSizing::Fixed(width) => self.set_min_width(width),
        }

        match self.intrinsic_height {
            BoxSizing::Fit | BoxSizing::Flex(_) => self.set_min_height(min_height),
            BoxSizing::Fixed(height) => self.set_min_height(height),
        }

        (min_width, min_height)
    }

    fn update_size(&mut self) {
        match self.intrinsic_width {
            BoxSizing::Fit => self.size.width = self.constraints.min_width,
            BoxSizing::Fixed(width) => self.size.width = width,
            BoxSizing::Flex(_) => self.size.width = self.constraints.max_width,
        }

        match self.intrinsic_height {
            BoxSizing::Fit => self.size.height = self.constraints.min_height,
            BoxSizing::Fixed(height) => self.size.height = height,
            BoxSizing::Flex(_) => self.size.height = self.constraints.max_height,
        }

        self.children.iter_mut().for_each(|c| c.update_size());
    }

    impl_layout!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EmptyLayout;

    #[test]
    fn sum_fixed_width() {
        let child1 = EmptyLayout::new().fixed(200.0, 300.0);
        let child2 = EmptyLayout::new().fixed(50.0, 670.0);

        let mut layout = HorizontalLayout::new();
        layout.append([child1,child2]);
        let total_width = layout.sum_fixed_width();

        assert_eq!(total_width, 250.0);
    }

    #[test]
    fn subtract_fixed_with_from_flex() {
        let child1 = EmptyLayout::new().fixed_width(200.0);
        let child2 = EmptyLayout::new().fixed_width(50.0);
        let child3 = EmptyLayout::new().fill();

        let mut layout = HorizontalLayout::new();
        layout.push(child1);
        layout.push(child2);
        layout.push(child3);
        layout.set_max_width(500.0);
        layout.solve_max_constraints();

        let child3 = &layout.children[2];

        assert_eq!(child3.constraints().max_width, 250.0);
    }

    #[test]
    fn fixed_max_constraints() {
        let child1 = EmptyLayout::new().fixed(200.0, 300.0);
        let child2 = EmptyLayout::new().fixed(50.0, 670.0);

        let mut layout = HorizontalLayout::new();
        layout.push(child1);
        layout.push(child2);
        layout.solve_max_constraints();

        let child1 = &layout.children[0];
        let child2 = &layout.children[1];

        assert_eq!(child1.constraints().max_width, 200.0);
        assert_eq!(child1.constraints().max_height, 300.0);
        assert_eq!(child2.constraints().max_width, 50.0);
        assert_eq!(child2.constraints().max_height, 670.0);
    }

    #[test]
    fn fill_max_constraints() {
        let child1 = EmptyLayout::new().fill();
        let child2 = EmptyLayout::new().fill();

        let mut layout = HorizontalLayout::new();
        layout.set_max_width(1000.0);
        layout.set_max_height(1000.0);
        layout.push(child1);
        layout.push(child2);
        layout.solve_max_constraints();

        let child1 = &layout.children[0];
        let child2 = &layout.children[1];

        assert_eq!(child1.constraints().max_width, 500.0);
        assert_eq!(child2.constraints().max_width, 500.0);
        assert_eq!(child1.constraints().max_height, 1000.0);
        assert_eq!(child2.constraints().max_height, 1000.0);
    }

    #[test]
    fn fit_max_constraints() {
        let mut child1 = EmptyLayout::new();
        let mut child2 = EmptyLayout::new();
        child1.set_min_width(150.0);
        child1.set_min_height(50.0);
        child2.set_min_width(300.0);
        child2.set_min_height(349.0);

        let mut layout = HorizontalLayout::new();
        layout.push(child1);
        layout.push(child2);
        layout.solve_max_constraints();

        let child1 = &layout.children[0];
        let child2 = &layout.children[1];

        assert_eq!(
            child1.constraints().max_width,
            child1.constraints().min_width
        );
        assert_eq!(
            child2.constraints().max_width,
            child2.constraints().min_width
        );
        assert_eq!(
            child1.constraints().max_height,
            child1.constraints().min_height
        );
        assert_eq!(
            child2.constraints().max_height,
            child2.constraints().min_height
        );
    }

    #[test]
    fn update_children_size() {
        let mut child1 = EmptyLayout::new();
        let mut child2 = EmptyLayout::new();
        child1.intrinsic_width = BoxSizing::Fixed(250.0);
        child2.intrinsic_height = BoxSizing::Fixed(90.0);

        let mut layout = HorizontalLayout::new();
        layout.push(child1);
        layout.push(child2);

        layout.update_size();

        let child1 = &layout.children[0];
        let child2 = &layout.children[1];

        assert_eq!(child1.size().width, 250.0);
        assert_eq!(child2.size().height, 90.0);
    }
}
