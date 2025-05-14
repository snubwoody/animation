use tiny_skia::{Color, Paint};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ControlFlow, EventLoop}, window::Window};

pub fn main(){
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
pub struct App{
    window: Option<Window>
}

impl ApplicationHandler for App{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop.create_window(Default::default()).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) 
    {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {

            },
            _ =>{}
        }
    }
}