//! GUI library.
//!
//! A [`Widget`] is a high level structure that can contain any data and
//! state. A [`Layout`] is an object that contains layout definitions
//! and BoxConstraints. An [`Element`] is a low level object that contains
//! color, size and position information used for rendering.
mod color;
mod element;
pub mod widget;
pub use color::{Color, Rgba};
pub use element::Element;
use pixels::{Pixels, SurfaceTexture};
pub use ruby_core::{Position, Size};
use std::sync::Arc;
use tiny_skia::Pixmap;
use widget::Widget;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

/// An [`App`] is your entire program
pub struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    pixmap: Option<Pixmap>,
    size: Size<u32>,
    widget: Box<dyn Widget>,
}

impl App<'_> {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            size: Size::default(),
            window: None,
            pixels: None,
            pixmap: None,
            widget: Box::new(widget),
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop.create_window(Default::default()).unwrap();
        let window = Arc::new(window);
        let PhysicalSize { width, height } = window.inner_size();

        let surface = SurfaceTexture::new(width, height, Arc::clone(&window));
        let pixels = Pixels::new(width, height, surface).unwrap();
        let drawing = Pixmap::new(width, height).unwrap();

        self.size = window.inner_size().into();
        self.window = Some(window);
        self.pixels = Some(pixels);
        self.pixmap = Some(drawing);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        let pixels = self.pixels.as_mut().unwrap();
        let Size { width, height } = self.size;
        let mut pixmap = Pixmap::new(width, height).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.widget.paint(&mut pixmap);
                pixels.frame_mut().copy_from_slice(pixmap.data());
                pixels.render().unwrap();
                window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.size = Size::from(size);
                let Size { width, height } = self.size;
                pixels.resize_buffer(width, height).unwrap();
                pixels.resize_surface(width, height).unwrap();
            }
            _ => {}
        }
    }
}
