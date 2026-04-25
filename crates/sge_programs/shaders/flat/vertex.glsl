#version 140
in vec2 pos;
in vec4 color;
in vec4 alt_color;
in int pattern;
in float scale;

out vec4 v_color;
out vec4 v_alt_color;
flat out int v_pattern;
flat out float v_scale;
out vec2 v_pos;

uniform mat4 transform;

void main() {
    v_color = color;
    v_pattern = pattern;
    v_alt_color = alt_color;
    v_scale = scale;
    v_pos = pos;
    gl_Position = transform * vec4(pos, 0.0, 1.0);
}
