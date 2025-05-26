use flow::{solve_layout, BlockLayout, BoxSizing, EmptyLayout, Layout, Size};

#[test]
fn fill_window(){
    let child = EmptyLayout::new();
    let mut root = BlockLayout::new(child);
    root.intrinsic_height = BoxSizing::Flex(1);
    root.intrinsic_width = BoxSizing::Flex(1);
    solve_layout(&mut root, Size::unit(500.0));
    assert_eq!(root.size(),Size::unit(500.0));
}