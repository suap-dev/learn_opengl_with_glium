pub const VERTEX_SHADER: &str = r#"
#version 150

in vec3 position;
in vec3 normal;
out vec3 v_normal;

uniform mat4 u_scale;
uniform mat4 u_rotation;
uniform mat4 u_perspective;
uniform mat4 u_translation;
uniform mat4 u_view;

void main() {
    // mat4 model_view = u_view * model;
    mat4 model_view = u_view * (u_translation * u_scale * u_rotation);
    
    // v_normal should be scaled if we non-uniformely scale positions.
    v_normal = transpose(inverse(mat3(model_view))) * normal;

    gl_Position = u_perspective * model_view *  vec4(position, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 140

in vec3 v_normal;
out vec4 color;

uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 reg_color = vec3(0.80, 0.50, 0.60);
    vec3 dark_color = vec3(0.30, 0.10, 0.20);
    color = vec4(mix(dark_color, reg_color, brightness), 1.0);
}
"#;
