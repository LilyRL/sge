#version 140

in vec2 position;
in vec2 a;
in vec2 b;
in vec2 c;
in vec4 color;
in float thickness;

out vec2 v_A;
out vec2 v_B;
out vec2 v_C;
out vec4 v_color;
out float v_thickness;
out vec2 v_pos;

uniform mat4 transform;

void main() {
    float pad = thickness + 1.0;

    float min_x = min(min(a.x, b.x), c.x) - pad;
    float max_x = max(max(a.x, b.x), c.x) + pad;
    float min_y = min(min(a.y, b.y), c.y) - pad;
    float max_y = max(max(a.y, b.y), c.y) + pad;

    vec2 p;
    p.x = mix(min_x, max_x, (position.x * 0.5 + 0.5));
    p.y = mix(min_y, max_y, (position.y * 0.5 + 0.5));

    v_A = a;
    v_B = b;
    v_C = c;
    v_color = color;
    v_thickness = thickness;
    v_pos = p;

    gl_Position = transform * vec4(p, 0.0, 1.0);
}
