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
    assert_eq!(root.size(), Size::new(24.0,230.0));
}

#[test]
fn fill_block_layout(){
    let mut child = EmptyLayout::new();
    child.intrinsic_width = BoxSizing::Flex(1);
    child.intrinsic_height = BoxSizing::Flex(1);

    let mut root = BlockLayout::new(child);
    root.intrinsic_width = BoxSizing::Flex(1);
    root.intrinsic_height = BoxSizing::Flex(1);

    solve_layout(&mut root, Size::unit(1000.0));
    let child = root.child();
    assert_eq!(child.size(),Size::unit(1000.0));
}

#[test]
fn padding_in_max_constraints(){
    let mut child = EmptyLayout::new();
    child.intrinsic_width = BoxSizing::Flex(1);
    child.intrinsic_height = BoxSizing::Flex(1);

    let mut root = BlockLayout::new(child);
    root.intrinsic_width = BoxSizing::Flex(1);
    root.intrinsic_height = BoxSizing::Flex(1);
    
    root.padding.top = 20;
    root.padding.bottom = 10;
    root.padding.left = 30;
    root.padding.right = 50;

    solve_layout(&mut root, Size::unit(1000.0));
    let child = root.child();
    let width = (1000 - root.padding.left - root.padding.right) as f32;
    let height = (1000 - root.padding.top - root.padding.bottom) as f32;
    assert_eq!(child.size(),Size::new(width,height));
}

#[test]
fn padding_in_min_constraints(){
    let child = EmptyLayout::new();
    
    let mut root = BlockLayout::new(child);
    root.padding.right = 24;
    root.padding.left = 10;
    root.padding.top = 13;
    root.padding.bottom = 10;

    root.solve_min_contraints();
    let min_width = (24 + 10) as f32;
    let min_height = (13 + 10) as f32;

    assert_eq!(root.constraints().min_width,min_width);
    assert_eq!(root.constraints().min_height,min_height);
}