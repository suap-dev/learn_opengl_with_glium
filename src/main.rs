#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod shapes;
mod shaders;

#[macro_use]
extern crate glium;

use glium::{glutin, DrawParameters, Surface};
use std::{f32::consts::TAU, io::Cursor};

fn main() {
    // init Display
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    
    let triangle = shapes::equilateral_triangle(0.8);
    let mut triangle_rotation: f32 = 0.0;
    
    // load texture png
    let image = image::load(
        Cursor::new(&include_bytes!("../assets/texture.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    // vertex buffer (with our triangle)
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();

    // dummy marker - no idea...
    // as I understood it's very important if you have more triangles. ?
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // SHADERS
    let vertex_shader_src = shaders::VERTEX_SHADER;
    let fragment_shader_src = shaders::FRAGMENT_SHADER;

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
                &uniform! {transform: transform, tex: &texture},
                &DrawParameters::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
