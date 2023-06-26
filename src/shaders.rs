pub const VERTEX_SHADER: &str = r#"
#version 140

in vec3 position;
in vec3 normal;
// in vec2 tex_coords;

// out vec4 new_color;
// out vec2 v_tex_coords;

uniform mat4 transform;
uniform mat4 rotation;

void main() {
    gl_Position =  transform * rotation * vec4(position, 1.0);
    // new_color = gl_Position;
    // v_tex_coords = tex_coords;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 140

// in vec4 new_color;
// in vec2 v_tex_coords;

out vec4 color;

// uniform sampler2D tex;

void main() {
    color = vec4(0.0, 0.4, 0.7, 1.0);
    // color = new_color;
    // color = texture(tex, v_tex_coords);
}
"#;
