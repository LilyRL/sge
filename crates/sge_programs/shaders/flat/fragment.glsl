#version 140

in vec2 v_pos;
in vec4 v_color;
in vec4 v_alt_color;
flat in int v_pattern;
flat in float v_scale;

const int FILL = 0;
const int CHECKER = 1;
const int HORIZONTAL_LINES = 2;
const int VERTICAL_LINES = 3;
const int NW_SE_LINES = 4;
const int NE_SW_LINES = 5;
const int DOTS = 6;
const int GRID = 7;
const int CROSS_HATCH = 8;
const int SPARSE_DOTS = 9;
const int BRICKS = 10;
const int HERRINGBONE = 11;
const int TRIANGLES = 12;
const int CONCENTRIC_SQUARES = 13;
const int WAVES = 14;
const int TEXTURED = 15;
const int CONCENTRIC_RINGS = 16;
const int TRUCHET = 17;
const int RANDOM_TILES = 18;
const int DIAGONAL_WAVES = 19;
const int TOPOLOGY = 20;
const int ZEBRA = 21;
const int FISH_SCALES = 22;
const int MAZE = 23;
const int MOIRE = 24;
const int LEOPARD_SPOTS = 25;
const int RINGS = 26;

out vec4 color;

vec4 fill() {
    return v_color;
}

const float BIAS = 0.0001;

vec4 checker() {
    int x = int(floor(v_pos.x / v_scale + BIAS));
    int y = int(floor(v_pos.y / v_scale + BIAS));
    if (mod(float(x + y), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 horizontal_lines() {
    int y = int(floor(v_pos.y / v_scale + BIAS));
    if (mod(float(y), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 vertical_lines() {
    int x = int(floor(v_pos.x / v_scale + BIAS));
    if (mod(float(x), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 nw_se_lines() {
    float d = (v_pos.x - v_pos.y) / v_scale;
    if (mod(floor(d + BIAS), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 ne_sw_lines() {
    float d = (v_pos.x + v_pos.y) / v_scale;
    if (mod(floor(d + BIAS), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 dots() {
    float scale = v_scale * 2.0;
    float cell_x = mod(v_pos.x, scale);
    float cell_y = mod(v_pos.y, scale);
    float dist = length(vec2(cell_x - scale * 0.5, cell_y - scale * 0.5));
    if (dist < scale * 0.3) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 cross_hatch() {
    // lines in both diagonal directions with same band width
    float d1 = (v_pos.x - v_pos.y) / v_scale;
    float d2 = (v_pos.x + v_pos.y) / v_scale;
    if (mod(floor(d1 + BIAS), 2.0) == 0.0 || mod(floor(d2 + BIAS), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 grid() {
    float cx = mod(v_pos.x, v_scale * 2.0) / (v_scale * 2.0);
    float cy = mod(v_pos.y, v_scale * 2.0) / (v_scale * 2.0);
    if (cx < 0.5 || cy < 0.5) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 herringbone() {
    float s = v_scale;
    int cx = int(floor(v_pos.x / s + BIAS));
    int cy = int(floor(v_pos.y / s + BIAS));
    float lx = mod(v_pos.x, s) / s;
    float ly = mod(v_pos.y, s) / s;
    // match horizontal_lines: stripe occupies half the cell
    float h = 0.5;
    bool colored;
    if (mod(float(cx + cy), 2.0) == 0.0) {
        colored = ly < h;
    } else {
        colored = lx < h;
    }
    if (colored) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 bricks() {
    float brick_w = v_scale * 4.0;
    float brick_h = v_scale * 2.0;
    int row = int(floor(v_pos.y / brick_h + BIAS));
    float offset = mod(float(row), 2.0) == 0.0 ? 0.0 : v_scale;
    float cell_x = mod(v_pos.x + offset, brick_w);
    float cell_y = mod(v_pos.y, brick_h);
    float thickness = max(1.0, v_scale * 0.4);
    if (cell_x < thickness || cell_y < thickness) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 sparse_dots() {
    float scale = v_scale * 2.0;
    int cx = int(floor(v_pos.x / scale + BIAS));
    int cy = int(floor(v_pos.y / scale + BIAS));
    float cell_x = mod(v_pos.x, scale);
    float cell_y = mod(v_pos.y, scale);
    float dist = length(vec2(cell_x - scale * 0.5, cell_y - scale * 0.5));
    if (mod(float(cx + cy), 2.0) == 0.0 && dist < scale * 0.3) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 triangles() {
    int cx = int(floor(v_pos.x / v_scale + BIAS));
    int cy = int(floor(v_pos.y / v_scale + BIAS));
    float lx = mod(v_pos.x, v_scale) / v_scale;
    float ly = mod(v_pos.y, v_scale) / v_scale;
    // alternate diagonal direction per cell
    bool flip = mod(float(cx + cy), 2.0) == 0.0;
    bool upper;
    if (flip) {
        upper = lx + ly < 1.0; // split \ way
    } else {
        upper = lx < ly; // split / way
    }
    if (upper) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 concentric_squares() {
    float dist = max(abs(v_pos.x), abs(v_pos.y)) / v_scale;
    if (mod(floor(dist + BIAS), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 waves() {
    float wave_y = v_pos.y + sin(v_pos.x / v_scale * 3.14159) * v_scale * 0.5;
    int band = int(floor(wave_y / v_scale + BIAS));
    if (mod(float(band), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 textured() {
    float s = v_scale;
    int cx = int(floor(v_pos.x / s + BIAS));
    int cy = int(floor(v_pos.y / s + BIAS));
    float lx = mod(v_pos.x, s) / s;
    float ly = mod(v_pos.y, s) / s;
    float thickness = 0.2;
    bool colored;
    if (mod(float(cx + cy), 2.0) == 0.0) {
        colored = ly > 0.5 - thickness && ly < 0.5 + thickness;
    } else {
        colored = lx > 0.5 - thickness && lx < 0.5 + thickness;
    }
    if (colored) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

float hash(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p + 34.23);
    return fract(p.x * p.y);
}

vec4 concentric_rings() {
    float dist = length(v_pos) / v_scale;
    if (mod(floor(dist + BIAS), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 truchet() {
    float cx = floor(v_pos.x / v_scale + BIAS);
    float cy = floor(v_pos.y / v_scale + BIAS);
    float lx = mod(v_pos.x, v_scale) / v_scale;
    float ly = mod(v_pos.y, v_scale) / v_scale;

    float h = hash(vec2(cx, cy));
    float thickness = 0.2;

    float dist;
    if (h < 0.5) {
        // arc connecting top-left and bottom-right corners
        dist = min(
                length(vec2(lx, ly)), // top-left corner
                length(vec2(lx - 1.0, ly - 1.0)) // bottom-right corner
            );
    } else {
        // arc connecting top-right and bottom-left corners
        dist = min(
                length(vec2(lx - 1.0, ly)), // top-right corner
                length(vec2(lx, ly - 1.0)) // bottom-left corner
            );
    }

    float r = 0.5;
    if (abs(dist - r) < thickness * 0.5) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 random_tiles() {
    float cx = floor(v_pos.x / v_scale + BIAS);
    float cy = floor(v_pos.y / v_scale + BIAS);
    if (hash(vec2(cx, cy)) < 0.5) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 diagonal_waves() {
    // step along the NE diagonal, then wave perpendicular to it
    float diag = (v_pos.x + v_pos.y) / v_scale;
    float perp = (v_pos.x - v_pos.y) / v_scale;
    float wave = diag + sin(perp * 3.14159 * 0.5) * 0.8;
    if (mod(floor(wave + BIAS), 2.0) == 0.0) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 topology() {
    // primary diagonal direction
    float d = (v_pos.x + v_pos.y) / v_scale;

    // perturb using multiple sine waves at different frequencies/directions
    // to create organic wobble along the stripe
    float perp = (v_pos.x - v_pos.y) / v_scale;
    float wobble =
        sin(perp * 0.3) * 1.8 + // large slow bends
            sin(perp * 0.7 + 1.4) * 0.9 + // medium bends
            sin(perp * 1.7 + 2.8) * 0.35; // small wiggles

    float stripe = sin((d + wobble) * 3.14159);
    if (stripe > 0.25) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 zebra() {
    float scale = v_scale * 2.0;
    float d = (v_pos.x + v_pos.y) / scale;
    float perp = (v_pos.x - v_pos.y) / scale;

    // which stripe are we in
    float stripe_id = floor(d);

    // each stripe gets unique random properties
    float r1 = hash(vec2(stripe_id, 0.0));
    float r2 = hash(vec2(stripe_id, 1.0));
    float r3 = hash(vec2(stripe_id + 1.0, 0.0)); // neighbour, for edge wobble

    // wobble the edges using perp position + per-stripe random phase/amplitude
    float wobble = sin(perp * 0.4 + r1 * 6.28) * (0.15 + r2 * 0.25)
            + sin(perp * 0.15 + r3 * 6.28) * 0.2;

    float local = fract(d + wobble);

    // vary stripe width per stripe: dark band takes 55-75% of the cell
    float width = 0.55 + r1 * 0.2;

    if (local < width) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

vec4 fish_scales() {
    float s = v_scale * 2.0;
    float row_h = s * 0.7;

    float row = floor(v_pos.y / row_h + BIAS);
    float x_off = mod(row, 2.0) * s * 0.5;
    float col = floor((v_pos.x + x_off) / s + BIAS);

    vec2 center = vec2(
            col * s - x_off + s * 0.5,
            row * row_h + row_h * 0.5
        );

    vec2 lp = v_pos - center;
    float dist = length(lp);

    if (dist > s * 0.5) {
        return v_alt_color;
    }

    // rings inside scale
    float ring = mod(floor(dist / (s * 0.18) + BIAS), 2.0);
    return ring == 0.0 ? v_color : v_alt_color;
}

vec4 maze() {
    float s = v_scale;
    float cx = floor(v_pos.x / s + BIAS);
    float cy = floor(v_pos.y / s + BIAS);
    float lx = mod(v_pos.x, s) / s;
    float ly = mod(v_pos.y, s) / s;

    // is this fragment in the right wall strip of its cell?
    bool in_right_strip = lx > 0.85;
    // is this fragment in the bottom wall strip of its cell?
    bool in_bottom_strip = ly > 0.85;

    // does this cell have a right wall? check the cell that owns it
    bool has_right_wall = hash(vec2(cx, cy)) > 0.5;
    // does this cell have a bottom wall?
    bool has_bottom_wall = hash(vec2(cx + 17.3, cy + 3.7)) > 0.5;

    // top and left borders are the right/bottom wall of the neighbouring cell
    bool in_left_strip = lx < 0.15;
    bool in_top_strip = ly < 0.15;
    bool neighbour_has_right = hash(vec2(cx - 1.0, cy)) > 0.5;
    bool neighbour_has_bottom = hash(vec2(cx + 17.3 - 0.0, cy - 1.0 + 3.7)) > 0.5;

    bool wall = (in_right_strip && has_right_wall)
            || (in_bottom_strip && has_bottom_wall)
            || (in_left_strip && neighbour_has_right)
            || (in_top_strip && neighbour_has_bottom);

    return wall ? v_color : v_alt_color;
}

vec4 moire() {
    // two clearly separated centers
    float offset = v_scale * 8.0;
    vec2 center1 = vec2(0.0, 0.0);
    vec2 center2 = vec2(offset, offset * 0.3);

    float r1 = sin(length(v_pos - center1) / v_scale * 3.14159);
    float r2 = sin(length(v_pos - center2) / v_scale * 3.14159);

    return r1 * r2 > 0.0 ? v_color : v_alt_color;
}

vec4 leopard_spots() {
    // voronoi — color based on nearest random point
    // search 3x3 neighbourhood of cells for nearest point
    float s = v_scale * 3.0;
    vec2 cell = floor(v_pos / s);
    vec2 lp = mod(v_pos, s);

    float min_dist1 = 1e10; // nearest point distance
    float min_dist2 = 1e10; // second nearest
    vec2 nearest = vec2(0.0);

    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            vec2 nc = cell + vec2(float(dx), float(dy));
            // random point within this cell
            vec2 point = vec2(
                    hash(nc),
                    hash(nc + vec2(43.7, 91.3))
                ) * s + vec2(float(dx), float(dy)) * s;
            float dist = length(lp - point);
            if (dist < min_dist1) {
                min_dist2 = min_dist1;
                min_dist1 = dist;
                nearest = nc;
            } else if (dist < min_dist2) {
                min_dist2 = dist;
            }
        }
    }

    // dark border ring around each spot using difference between
    // nearest and second nearest distances (voronoi edge detection)
    float edge = min_dist2 - min_dist1;
    float border = s * 0.15;
    if (edge < border) {
        return v_color; // dark border
    }

    // spot or background based on per-cell hash
    float spot_chance = hash(nearest + vec2(12.4, 56.7));
    if (spot_chance > 0.45) {
        return v_color; // spot
    } else {
        return v_alt_color; // background
    }
}

vec4 rings() {
    float scale = v_scale * 2.0;
    float cell_x = mod(v_pos.x, scale);
    float cell_y = mod(v_pos.y, scale);
    float dist = length(vec2(cell_x - scale * 0.5, cell_y - scale * 0.5));

    if (dist < scale * 0.2 || dist > scale * 0.4) {
        return v_color;
    } else {
        return v_alt_color;
    }
}

void main() {
    if (v_pattern == FILL) {
        color = fill();
    } else if (v_pattern == CHECKER) {
        color = checker();
    } else if (v_pattern == HORIZONTAL_LINES) {
        color = horizontal_lines();
    } else if (v_pattern == VERTICAL_LINES) {
        color = vertical_lines();
    } else if (v_pattern == NW_SE_LINES) {
        color = nw_se_lines();
    } else if (v_pattern == NE_SW_LINES) {
        color = ne_sw_lines();
    } else if (v_pattern == DOTS) {
        color = dots();
    } else if (v_pattern == GRID) {
        color = grid();
    } else if (v_pattern == CROSS_HATCH) {
        color = cross_hatch();
    } else if (v_pattern == SPARSE_DOTS) {
        color = sparse_dots();
    } else if (v_pattern == BRICKS) {
        color = bricks();
    } else if (v_pattern == HERRINGBONE) {
        color = herringbone();
    } else if (v_pattern == TRIANGLES) {
        color = triangles();
    } else if (v_pattern == CONCENTRIC_SQUARES) {
        color = concentric_squares();
    } else if (v_pattern == WAVES) {
        color = waves();
    } else if (v_pattern == TEXTURED) {
        color = textured();
    } else if (v_pattern == CONCENTRIC_RINGS) {
        color = concentric_rings();
    } else if (v_pattern == TRUCHET) {
        color = truchet();
    } else if (v_pattern == RANDOM_TILES) {
        color = random_tiles();
    } else if (v_pattern == DIAGONAL_WAVES) {
        color = diagonal_waves();
    } else if (v_pattern == TOPOLOGY) {
        color = topology();
    } else if (v_pattern == ZEBRA) {
        color = zebra();
    } else if (v_pattern == FISH_SCALES) {
        color = fish_scales();
    } else if (v_pattern == MAZE) {
        color = maze();
    } else if (v_pattern == MOIRE) {
        color = moire();
    } else if (v_pattern == LEOPARD_SPOTS) {
        color = leopard_spots();
    } else if (v_pattern == RINGS) {
        color = rings();
    } else {
        // default to solid fill if unknown pattern
        color = v_color;
    }
}
