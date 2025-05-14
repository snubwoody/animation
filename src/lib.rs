use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::{ControlFlow, EventLoop}, window::Window};


struct Animation{

}

#[derive(Default)]
pub struct App<'a>{
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    pixmap: Option<Pixmap>,
    width: u32,
    height: u32
}

impl<'a> ApplicationHandler for App<'a>{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop.create_window(Default::default()).unwrap();
        let window = Arc::new(window);
        let PhysicalSize{width,height} = window.inner_size();
        
        let surface = SurfaceTexture::new(width, height, Arc::clone(&window));
        let pixels = Pixels::new(width, height, surface).unwrap();
        let drawing = Pixmap::new(width, height).unwrap();
        
        self.window = Some(window);
        self.pixels = Some(pixels);
        self.pixmap = Some(drawing);
        self.width = width;
        self.height = height;
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        let pixels = self.pixels.as_mut().unwrap();
        let mut pixmap = Pixmap::new(self.width, self.height).unwrap();
        pixmap.fill(Color::WHITE);
       

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let circle = Circle::new(100.0, 100.0, 200.0);
                circle.paint(&mut pixmap);
                pixels.frame_mut().copy_from_slice(pixmap.data());
                pixels.render().unwrap();
                window.request_redraw();
            },
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                self.width = width;
                self.height = height;
                pixels.resize_buffer(self.width, self.height).unwrap();
                pixels.resize_surface(self.width, self.height).unwrap();
            }
            _ =>{}
        }
    }
}

pub fn main(){
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

pub fn draw(pixmap: &mut Pixmap){
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(50, 107,160,255));
    paint.anti_alias = true;

    let path = PathBuilder::from_circle(0.0, 0.0, 150.0).unwrap();
    pixmap.fill(Color::WHITE);
    pixmap.fill_path(
        &path, 
        &paint, 
        FillRule::Winding, 
        Transform::identity(), 
        None
    );
}

struct Circle{
    position: [f32;2],
    radius: f32
}

impl Circle{
    pub fn new(x:f32,y:f32,radius: f32) -> Self{
        Self { position: [x,y], radius }
    }

    /// Draw the circle onto a pixelmap
    pub fn paint(&self,pixmap: &mut Pixmap){
        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(50, 107,160,255));
        paint.anti_alias = true;
        
        let [x,y] = self.position;
        
        let path = PathBuilder::from_circle(x, y, self.radius).unwrap();
        pixmap.fill_path(
            &path, 
            &paint, 
            FillRule::Winding, 
            Transform::identity(), 
            None
        );
    }
}