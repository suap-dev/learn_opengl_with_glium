use std::f32::consts::TAU;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

// first triangle
// kinda equilateral
pub fn equilateral_triangle(size: f32) -> Vec<Vertex> {
    let default_vert_pos: [f32; 2] = [0.0, size];
    let v1 = Vertex {
        position: rotated(default_vert_pos, 0.0 * TAU / 3.0),
        tex_coords: [0.0, 0.0],
    };
    let v2 = Vertex {
        position: rotated(default_vert_pos, 1.0 * TAU / 3.0),
        tex_coords: [0.0, 1.0],
    };
    let v3 = Vertex {
        position: rotated(default_vert_pos, 2.0 * TAU / 3.0),
        tex_coords: [1.0, 0.0],
    };
    vec![v1, v2, v3]
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
