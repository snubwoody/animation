use flow::{BoxSizing, EmptyLayout, HorizontalLayout, Layout, Size, solve_layout};

#[test]
fn fit_children() {
    let mut child1 = EmptyLayout::new();
    let mut child2 = EmptyLayout::new();

    child1.intrinsic_width = BoxSizing::Fixed(90.0);
    child1.intrinsic_height = BoxSizing::Fixed(24.0);
    child2.intrinsic_width = BoxSizing::Fixed(350.0);
    child2.intrinsic_height = BoxSizing::Fixed(240.0);

    let mut root = HorizontalLayout::new();
    root.push(child1);
    root.push(child2);

    solve_layout(&mut root, Size::unit(500.0));
    let width = 90.0 + 350.0;
    let height = 24.0 + 240.0;

    assert_eq!(root.size(), Size::new(width, height))
}

#[test]
fn fill_window() {
    let mut root = HorizontalLayout::new().fill();
    solve_layout(&mut root, Size::new(500.0,100.0));
    assert_eq!(root.size(), Size::new(500.0, 100.0))
}
