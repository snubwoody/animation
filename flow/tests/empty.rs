use flow::{BoxSizing, EmptyLayout, Layout, Size};

#[test]
fn fill_window() {
    let mut layout = EmptyLayout::new();
    layout.intrinsic_width = BoxSizing::Flex(1);
    layout.intrinsic_height = BoxSizing::Flex(1);
    flow::solve_layout(&mut layout, Size::new(50.0, 700.0));
    assert_eq!(layout.size(), Size::new(50.0, 700.0));
}

#[test]
fn fit() {
    let mut layout = EmptyLayout::new();
    layout.intrinsic_width = BoxSizing::Fit;
    layout.intrinsic_height = BoxSizing::Fit;
    flow::solve_layout(&mut layout, Size::new(50.0, 700.0));
    assert_eq!(layout.size(), Size::unit(0.0));
}

#[test]
fn fixed_size() {
    let mut layout = EmptyLayout::new();
    layout.intrinsic_width = BoxSizing::Fixed(250.0);
    layout.intrinsic_height = BoxSizing::Fixed(20.0);
    flow::solve_layout(&mut layout, Size::unit(1000.0));
    assert_eq!(layout.size(), Size::new(250.0, 20.0));
}
