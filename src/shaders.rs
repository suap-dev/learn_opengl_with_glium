pub const VERTEX_SHADER: &str = r#"
#version 150

in vec3 position;
in vec3 normal;
out vec3 v_normal;
uniform mat4 resize;
uniform mat4 rotation;

void main() {
    v_normal = transpose(inverse(mat3(resize))) * normal;
    gl_Position =   resize * rotation * vec4(position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 140

in vec3 v_normal;
out vec4 color;
uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 reg_color = vec3(0.91, 0.608, 0.757);
    vec3 dark_color = vec3(0.612, 0.212, 0.408);
    color = vec4(mix(dark_color, reg_color, brightness), 1.0);
}
"#;
