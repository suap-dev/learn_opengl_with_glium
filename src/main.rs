#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

// mod shapes;
mod shaders;
mod teapot;

#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use std::f32::consts::TAU;

#[allow(clippy::too_many_lines)]
fn main() {
    // init Display
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        // .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(600, 600))
        .with_always_on_top(true)
        // .with_decorations(false)
        .with_position(glutin::dpi::LogicalPosition::new(53.0, 420.0));
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let teapot_positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let teapot_normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let teapot_indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();
    let mut rotation: f32 = 0.0;

    // SHADERS
    let vertex_shader_src: &str = shaders::VERTEX_SHADER;
    let fragment_shader_src: &str = shaders::FRAGMENT_SHADER;

    // program
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

        let rotation_per_sec: f32 = TAU / 10.0;
        let rotation_per_frame: f32 = rotation_per_sec * frame_time.as_secs_f32();

        // resize, because teapot big.
        let scale: [[f32; 4]; 4] = {
            let scale: f32 = 0.008;

            [
                [scale, 0.0, 0.0, 0.0],
                [0.0, scale, 0.0, 0.0],
                [0.0, 0.0, scale, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        let translation: [[f32; 4];4] = {
            let x: f32 = 0.0;
            let y: f32 = -1.0;
            let z: f32 = 4.0;

            [
                [1.0,   0.0,    0.0,    0.0],
                [0.0,   1.0,    0.0,    0.0],
                [0.0,   0.0,    1.0,    0.0],
                [x,     y,      z,      1.0]
            ]
        };

        // rotate, because fun.

        // unless we are doing couple of thousand operations of trigonometry each frame we can really do it on CPU
        // source: https://www.reddit.com/r/AskComputerScience/comments/22g1dg/how_is_trigonometry_computed_with_cpu_does_gpu/
        let rotation: [[f32; 4]; 4] = {
            rotation += rotation_per_frame;
            if rotation > TAU {
                rotation -= TAU;
            }

            [
                [rotation.cos(),    0.0,    rotation.sin(),    0.0],
                [0.0,               1.0,    0.0,               0.0],
                [-rotation.sin(),   0.0,    rotation.cos(),    0.0],
                [0.0,               0.0,    0.0,               1.0],
            ]
        };

        // clear screen with a nice blue color
        let mut target = display.draw();

        let perspective: [[f32; 4]; 4] = {
            let (width, height) = target.get_dimensions();

            #[allow(clippy::cast_precision_loss)]
            let aspect_ratio: f32 = height as f32 / width as f32;

            let fov: f32 = TAU / 6.0;
            let z_far: f32 = 1024.0;
            let z_near: f32 = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio,  0.0,    0.0,                                        0.0],
                [0.0,               f,      0.0,                                        0.0],
                [0.0,               0.0,    (z_far + z_near) / (z_far - z_near),        1.0],
                [0.0,               0.0,    -(2.0 * z_far * z_near) / (z_far - z_near), 0.0],
            ]
        };

        let light: [f32; 3] = [-0.9, 1.0, -0.2];

        // clear screen with a nice blue color
        // let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.4, 0.7, 1.0), 1.0);

        // draw prepared triangle with prepared program
        target
            .draw(
                (&teapot_positions, &teapot_normals),
                &teapot_indices,
                &program,
                &uniform! {u_scale: scale, u_rotation: rotation, u_light: light, u_perspective: perspective, u_translation: translation},
                &glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..glium::Depth::default()
                    },
                    ..glium::DrawParameters::default()
                },
            )
            .unwrap();
        target.finish().unwrap();
    }); // event_loop::run(..)
}
