#version 140
in vec2 position;
in vec2 dimensions;
in vec3 center;
in float corner_radius;
in float outline_thickness;
in vec4 fill_color;
in vec4 outline_color;

out vec2 v_center;
out vec2 v_dimensions;
out vec2 v_frag_pos;
out float v_corner_radius;
out float v_outline_thickness;
out vec4 v_fill_color;
out vec4 v_outline_color;

uniform mat4 transform;

void main() {
    vec2 local = position * dimensions / 2.0;

    v_center = center.xy;
    v_dimensions = dimensions;
    v_frag_pos = local;  
    v_corner_radius = corner_radius;
    v_outline_thickness = outline_thickness;
    v_fill_color = fill_color;
    v_outline_color = outline_color;

    vec3 world_pos = vec3(local + center.xy, center.z);
    gl_Position = transform * vec4(world_pos, 1.0);
}
