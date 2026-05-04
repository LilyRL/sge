#version 140
uniform sampler1D metaball_data;
uniform int num_metaballs;
uniform vec4 color;
in vec2 v_pos;
out vec4 frag_color;

float falloff(float d, float r) {
    float xr = clamp(d / r, 0.0, 1.0);
    return 1.0 - (3.0 * xr * xr) + (2.0 * xr * xr * xr);
}

void main() {
    float v = 0.0;
    for (int i = 0; i < num_metaballs; i++) {
        float u = (float(i) + 0.5) / 64.0;
        vec4 data = texture(metaball_data, u);
        vec2 center = data.xy;
        float r = data.z * 2.0;
        float d = length(v_pos - center);
        v += falloff(d, r);
    }

    float edge_width = length(vec2(dFdx(v), dFdy(v)));
    float alpha = smoothstep(0.5 - edge_width, 0.5 + edge_width, v);

    if (alpha <= 0.0) discard;
    frag_color = vec4(color.rgb, color.a * alpha);
}
