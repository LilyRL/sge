#version 140
in vec2 position;
in vec2 a;
in vec2 b;
in vec2 c;
in vec2 d;
in vec4 color;
in float thickness;
out vec2 v_A;
out vec2 v_B;
out vec2 v_C;
out vec2 v_D;
out vec4 v_color;
out float v_thickness;
out vec2 v_pos;
uniform mat4 transform;

void cubicExtrema(float a, float b, float c, float d, out float lo, out float hi) {
    float da = 3.0*(b - a);
    float db = 6.0*(a - 2.0*b + c);
    float dc = 3.0*(-a + 3.0*b - 3.0*c + d);

    lo = min(a, d);
    hi = max(a, d);

    float disc = db*db - 4.0*dc*da;
    if (abs(dc) < 1e-6) {
        if (abs(db) > 1e-6) {
            float t = -da / db;
            if (t > 0.0 && t < 1.0) {
                float v = a + t*(b - a);
                float mt = 1.0 - t;
                float val = mt*mt*mt*a + 3.0*mt*mt*t*b + 3.0*mt*t*t*c + t*t*t*d;
                lo = min(lo, val);
                hi = max(hi, val);
            }
        }
    } else if (disc >= 0.0) {
        float sq = sqrt(disc);
        float t1 = (-db + sq) / (2.0*dc);
        float t2 = (-db - sq) / (2.0*dc);
        for (int i = 0; i < 2; i++) {
            float t = (i == 0) ? t1 : t2;
            if (t > 0.0 && t < 1.0) {
                float mt = 1.0 - t;
                float val = mt*mt*mt*a + 3.0*mt*mt*t*b + 3.0*mt*t*t*c + t*t*t*d;
                lo = min(lo, val);
                hi = max(hi, val);
            }
        }
    }
}

void main() {
    float pad = thickness + 1.0;

    float min_x, max_x, min_y, max_y;
    cubicExtrema(a.x, b.x, c.x, d.x, min_x, max_x);
    cubicExtrema(a.y, b.y, c.y, d.y, min_y, max_y);

    min_x -= pad; max_x += pad;
    min_y -= pad; max_y += pad;

    vec2 p;
    p.x = mix(min_x, max_x, (position.x * 0.5 + 0.5));
    p.y = mix(min_y, max_y, (position.y * 0.5 + 0.5));

    v_A = a;
    v_B = b;
    v_C = c;
    v_D = d;
    v_color = color;
    v_thickness = thickness;
    v_pos = p;
    gl_Position = transform * vec4(p, 0.0, 1.0);
}
