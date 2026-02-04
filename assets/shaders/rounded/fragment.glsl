#version 140
in vec2 v_frag_pos;
in vec2 v_dimensions;
in float v_corner_radius;
in float v_outline_thickness;
in vec4 v_fill_color;
in vec4 v_outline_color;

out vec4 color;

float sdRoundedBox(vec2 p, vec2 halfSize, float radius) {
    vec2 q = abs(p) - halfSize + radius;
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - radius;
}

void main() {
    vec2 halfSize = v_dimensions / 2.0;
    float dist = sdRoundedBox(v_frag_pos, halfSize, v_corner_radius);

    float edge_softness = 1.0;

    if (v_outline_thickness > 0.0) {
        float outer_alpha = 1.0 - smoothstep(-edge_softness, edge_softness, dist);
        float inner_alpha = 1.0 - smoothstep(-v_outline_thickness - edge_softness,
                                             -v_outline_thickness + edge_softness,
                                             dist);

        color = mix(v_outline_color, v_fill_color, inner_alpha);
        color.a *= outer_alpha;
    } else {
        float alpha = 1.0 - smoothstep(-edge_softness, edge_softness, dist);
        color = v_fill_color;
        color.a *= alpha;
    }
}
