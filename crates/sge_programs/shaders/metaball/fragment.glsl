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
    // frag_color = vec4(mod(v_pos / 100.0, 1.0), 0.0, 1.0);
    // return;

    float v = 0.0;
    for (int i = 0; i < num_metaballs; i++) {
        float u = (float(i) + 0.5) / 32.0;
        vec4 data = texture(metaball_data, u);
        vec2 center = data.xy;
        float r = data.z * 2.0;
        float d = length(v_pos - center);

        v += falloff(d, r);
    }

    if (v < 0.5) discard;

    frag_color = color;
}
