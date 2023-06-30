pub const VERTEX_SHADER: &str = r#"
#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 u_perspective;
uniform mat4 model_view;

void main() {    
    // v_normal should be scaled if we non-uniformely scale positions.
    v_normal = transpose(inverse(mat3(model_view))) * normal;    

    gl_Position = u_perspective * model_view *  vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light;

const vec3 ambient_color = vec3(0.30, 0.10, 0.20);
const vec3 diffuse_color = vec3(0.80, 0.50, 0.60);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    vec3 light = normalize(u_light);
    vec3 normal = normalize(v_normal);

    vec3 position_direction = normalize(v_position);
    vec3 camera_direction = -position_direction;

    vec3 half_direction = normalize(light + camera_direction);

    // dot product of 2 vectors is a cosine of the angle between them
    float diffuse_intensity = max(dot(normal, light), -0.35);
    float specular_intensity = pow(max(dot(half_direction, normal), 0.0), 16.0);

    // vec3 scaled_ambient_color = ambient_color;
    vec3 scaled_diffuse_color = diffuse_intensity * diffuse_color;
    vec3 scaled_specular_color = specular_intensity * specular_color;

    color = vec4(ambient_color + scaled_diffuse_color + scaled_specular_color, 1.0);
}
"#;
