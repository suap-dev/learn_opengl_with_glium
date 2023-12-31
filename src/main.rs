#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

// mod shapes;
mod matrices;
mod shaders;
mod teapot;

#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use std::f32::consts::TAU;

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

    // shaders
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
        // frame time        
        let fps: u128 = 60;
        #[allow(clippy::cast_possible_truncation)]
        let nanos_between_frames: u64 = (std::time::Duration::from_secs(1).as_nanos() / fps) as u64;
        let frame_time = std::time::Duration::from_nanos(nanos_between_frames);
        let next_frame_time = std::time::Instant::now() + frame_time;
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // clear screen with a nice blue color
        let mut target = display.draw();

        // get current aspect ratio
        let (width, height) = target.get_dimensions();
        #[allow(clippy::cast_precision_loss)]
        let aspect_ratio = height as f32 / width as f32;

        // rotation angle
        let rotation_per_sec = TAU / 10.0;
        let rotation_per_frame = rotation_per_sec * frame_time.as_secs_f32();
        rotation += rotation_per_frame;
        if rotation > TAU {
            rotation -= TAU;
        }

        // light vector (or position?)
        let light: [f32; 3] = [-0.9, 1.0, -0.2];

        // transforms
        let view_matrix = matrices::view(&[0.0, 0.0, -2.0], &[0.0, 0.0, 2.0], &[0.0, 1.0, 0.0]);
        let translation_matrix = matrices::translation(0.0, 0.0, 0.8);
        let scale_matrix = matrices::scale(0.008);
        let rotation_matrix = matrices::rotation(matrices::Axis::Z, rotation);

        let model_view_matrix = matrices::left_mul(&mut vec![
            &view_matrix,
            &translation_matrix,
            &scale_matrix,
            &rotation_matrix,
        ]);

        let perspective_matrix = matrices::perspective(aspect_ratio, TAU / 6.0, 0.1, 1024.0);

        // clear screen with a nice blue color
        target.clear_color_and_depth((0.0, 0.4, 0.7, 1.0), 1.0);

        // draw
        target
            .draw(
                (&teapot_positions, &teapot_normals),
                &teapot_indices,
                &program,
                &uniform! {
                    u_light: light,
                    u_perspective: perspective_matrix,
                    // u_rotation: rotation_matrix,
                    // u_scale: scale_matrix,
                    // u_translation: translation_matrix,
                    // u_view: view_matrix,
                    model_view: model_view_matrix,
                },
                &glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..glium::Depth::default()
                    },
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    ..glium::DrawParameters::default()
                },
            )
            .unwrap();
        target.finish().unwrap();
    }); // event_loop::run(..)
}
