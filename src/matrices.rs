#[allow(clippy::suboptimal_flops)]
pub fn view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    // source: https://glium.github.io/glium/book/tuto-12-camera.html
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

// unless we are doing couple of thousand operations of trigonometry each frame we can really do it on CPU
// source: https://www.reddit.com/r/AskComputerScience/comments/22g1dg/how_is_trigonometry_computed_with_cpu_does_gpu/
pub fn rotation(axis: Axis, angle: f32) -> [[f32; 4]; 4] {
    match axis {
        Axis::Y => [
            [angle.cos(), angle.sin(), 0.0, 0.0],
            [-angle.sin(), angle.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
        Axis::X => [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle.cos(), angle.sin(), 0.0],
            [0.0, -angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
        Axis::Z => [
            [angle.cos(), 0.0, angle.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-angle.sin(), 0.0, angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub const fn translation(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, y, z, 1.0],
    ]
}

pub const fn scale(factor: f32) -> [[f32; 4]; 4] {
    [
        [factor, 0.0, 0.0, 0.0],
        [0.0, factor, 0.0, 0.0],
        [0.0, 0.0, factor, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn perspective(aspect_ratio: f32, fov_angle: f32, z_near: f32, z_far: f32) -> [[f32; 4]; 4] {
    let f = 1.0 / (fov_angle / 2.0).tan();
    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (z_far + z_near) / (z_far - z_near), 1.0],
        [0.0, 0.0, -(2.0 * z_far * z_near) / (z_far - z_near), 0.0],
    ]
}

// naive iterating algorithm
pub fn product(mat1: &[[f32; 4]; 4], mat2: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += mat1[k][j] * mat2[i][k];
            }
        }
    }

    result
}

pub fn left_mul(matrices: &mut Vec<&[[f32;4];4]>) -> [[f32;4];4] {
    if matrices.len() > 1 {
        let right = matrices.pop().unwrap();
        product(&left_mul(matrices), right)
    } else {
        *matrices.pop().unwrap()
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}
