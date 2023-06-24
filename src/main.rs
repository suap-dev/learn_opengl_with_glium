#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();

    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();

    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    
    event_loop.run(move |event, _, control_flow| {
        if let glutin::event::Event::WindowEvent { event, .. } = event {
            if event == glutin::event::WindowEvent::CloseRequested {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        } else {
            return;
        }

        let mut target = display.draw();
        target.clear_color(1.0, 59.0/255.0, 0.0, 0.0);
        target.finish().unwrap();
    });            
}
