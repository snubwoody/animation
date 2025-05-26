use ruby::Element;

#[test]
fn horizontal_spacing(){
    let mut element = Element{
        spacing: 20.0,
        children: vec![
            Element::default(),
            Element::default(),
            Element::default(),
        ],
        ..Default::default()
    };

    element.layout();
    assert_eq!(element.children[0].position.x,0.0);
    assert_eq!(element.children[1].position.x,20.0);
    assert_eq!(element.children[2].position.x,40.0);
}