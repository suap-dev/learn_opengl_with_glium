#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    use glium::{glutin, Surface};
    use std::f32::consts::TAU;

    // init Display
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // first triangle
    // kinda equilateral
    const DEFAULT_VERT_POS: [f32; 2] = [0.0, 0.5];
    let v1 = Vertex {
        position: rotated(&DEFAULT_VERT_POS, 0.0 * TAU / 3.0),
    };
    let v2 = Vertex {
        position: rotated(&DEFAULT_VERT_POS, 1.0 * TAU / 3.0),
    };
    let v3 = Vertex {
        position: rotated(&DEFAULT_VERT_POS, 2.0 * TAU / 3.0),
    };
    let triangle = vec![v1, v2, v3];
    let mut rotation: f32 = 0.0;

    // vertex buffer (with our triangle)
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();

    // dummy marker - no idea...
    // as I understood it's very important if you have more triangles. ?
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // SHADERS
    // Vertex Shader
    // in vec2 position; - declare that we are expected to be passed an attribute named position
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform float rotation;

        void main() {
            float x = position.x * cos(rotation) - position.y * sin(rotation);
            float y = position.x * sin(rotation) + position.y * cos(rotation);

            gl_Position = vec4(x, y, 0.0, 1.0);
        }
    "#;

    // Fragment Shader

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.4, 0.7, 1.0);
        }
    "#;

    // program
    // send shaders to glium
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // event loop (game loop?)
    event_loop.run(move |event, _, control_flow| {
        #[allow(clippy::collapsible_match)]
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }        
        
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        rotation += TAU/360.0;
        if rotation > TAU {
            rotation -= TAU;
        } 
        
        // clear screen with a nice orange color
        let mut target = display.draw();
        target.clear_color(1.0, 59.0 / 255.0, 0.0, 0.0);

        // draw prepared triangle with prepared program
        target
            .draw(
                &vertex_buffer,
                indices,
                &program,
                &uniform! {rotation: rotation},
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

fn rotated(vec2: &[f32; 2], angle: f32) -> [f32; 2] {
    let x = vec2[0];
    let y = vec2[1];

    let new_x = x * angle.cos() - y * angle.sin();
    let new_y = x * angle.sin() + y * angle.cos();

    [new_x, new_y]
}
