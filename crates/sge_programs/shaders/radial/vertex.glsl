#version 140

in vec2 position;
in vec3 center;
in vec2 radius;
in float outline_thickness;
in vec4 inner_color;
in vec4 outer_color;
in vec4 outline_color;
in vec2 gradient_offset; // offset of the gradient center from shape center, in world units

out vec2 v_center;
out vec2 v_radius;
out float v_outline_thickness;
out vec4 v_inner_color;
out vec4 v_outer_color;
out vec4 v_outline_color;
out vec2 v_gradient_center;
out vec2 frag_position;

uniform mat4 transform;

void main() {
    float max_radius = max(radius.x, radius.y) + outline_thickness;
    vec3 scaled_position = vec3(position * max_radius, 0.0) + center;
    frag_position = scaled_position.xy;
    v_center = center.xy;
    v_radius = radius;
    v_outline_thickness = outline_thickness;
    v_inner_color = inner_color;
    v_outer_color = outer_color;
    v_outline_color = outline_color;
    v_gradient_center = center.xy + gradient_offset;
    gl_Position = transform * vec4(scaled_position, 1.0);
}
