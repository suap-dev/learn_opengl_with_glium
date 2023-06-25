#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use glium::DrawParameters;

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
    let default_vert_pos: [f32; 2] = [0.0, 0.8];
    let v1 = Vertex {
        position: rotated(default_vert_pos, 0.0 * TAU / 3.0),
    };
    let v2 = Vertex {
        position: rotated(default_vert_pos, 1.0 * TAU / 3.0),
    };
    let v3 = Vertex {
        position: rotated(default_vert_pos, 2.0 * TAU / 3.0),
    };
    let triangle = vec![v1, v2, v3];
    let mut triangle_rotation: f32 = 0.0;

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
        out vec4 new_color;

        uniform mat4 transform; 

        void main() {
            gl_Position = transform * vec4(position, 0.0, 1.0);
            new_color = gl_Position;
        }
    "#;

    // Fragment Shader
    let fragment_shader_src = r#"
        #version 140

        in vec4 new_color;
        out vec4 color;

        void main() {
            // color = vec4(0.0, 0.4, 0.7, 1.0);
            color = new_color;
        }
    "#;

    // program
    // send shaders to glium
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // event loop (game loop?)
    event_loop.run(move |event, _, control_flow| {
        #[allow(clippy::collapsible_match, clippy::match_same_arms)]
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

        let nanos_between_frames: u64 = 16_666_667;
        let frame_time = std::time::Duration::from_nanos(nanos_between_frames);

        let next_frame_time = std::time::Instant::now() + frame_time;
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(std::time::Instant::now() + std::time::Duration::from_nanos(16));

        let rotation_per_sec: f32 = TAU / 6.0;
        let rotation_per_frame = rotation_per_sec * frame_time.as_secs_f32();

        triangle_rotation += rotation_per_frame;
        if triangle_rotation > TAU {
            triangle_rotation -= TAU;
        }
        
        // unless we are doing couple of thousand operations of trigonometry each frame we can really do it on CPU
        // source: https://www.reddit.com/r/AskComputerScience/comments/22g1dg/how_is_trigonometry_computed_with_cpu_does_gpu/
        let transform: [[f32; 4]; 4] = [
            [triangle_rotation.cos(), triangle_rotation.sin(), 0.0, 0.0],
            [-triangle_rotation.sin(), triangle_rotation.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        // clear screen with a nice blue color
        let mut target = display.draw();
        target.clear_color(0.0, 0.4, 0.7, 1.0);

        // draw prepared triangle with prepared program
        target
            .draw(
                &vertex_buffer,
                indices,
                &program,
                &uniform! {transform: transform},
                &DrawParameters::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

fn rotated(vec2: [f32; 2], angle: f32) -> [f32; 2] {
    let x = vec2[0];
    let y = vec2[1];

    // let new_x = x * angle.cos() - y * angle.sin();
    let new_x = x.mul_add(angle.cos(), y * angle.sin());
    // let new_y = x * angle.sin() + y * angle.cos();
    let new_y = x.mul_add(angle.sin(), y * angle.cos());

    [new_x, new_y]
}
