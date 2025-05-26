use ruby::widget::Rect;

fn main() {
    let rect = Rect::new().size(200.0, 200.0);
    let app = ruby::App::new(rect);
    app.run();
}
