use flow::{BlockLayout, BoxSizing, EmptyLayout, Layout, Size, solve_layout};

#[test]
fn fill_window() {
    let child = EmptyLayout::new();
    let mut root = BlockLayout::new(child);
    root.intrinsic_height = BoxSizing::Flex(1);
    root.intrinsic_width = BoxSizing::Flex(1);
    solve_layout(&mut root, Size::unit(500.0));
    assert_eq!(root.size(), Size::unit(500.0));
}

#[test]
fn fit_child(){
    let mut child = EmptyLayout::new();
    child.intrinsic_width = BoxSizing::Fixed(24.0);
    child.intrinsic_height = BoxSizing::Fixed(230.0);
    
    let mut root = BlockLayout::new(child);
    
    solve_layout(&mut root, Size::unit(500.0));
    dbg!(&root);
    assert_eq!(root.size(), Size::new(24.0,230.0));
}