#version 140

// taken from: https://www.shadertoy.com/view/4sKyzW

in vec2 v_A;
in vec2 v_B;
in vec2 v_C;
in vec2 v_D;
in vec4 v_color;
in float v_thickness;
in vec2 v_pos;

out vec4 color;

const float eps = 0.000005;
const int halley_iterations = 8;

float upper_bound_lagrange5(float a0, float a1, float a2, float a3, float a4) {
    vec4 coeffs1 = vec4(a0, a1, a2, a3);
    vec4 neg1 = max(-coeffs1, vec4(0));
    float neg2 = max(-a4, 0.);
    const vec4 indizes1 = vec4(0, 1, 2, 3);
    vec4 bounds1 = pow(neg1, 1. / (5. - indizes1));
    float bounds2 = pow(neg2, 1. / (5. - 4.));
    vec2 min1_2 = min(bounds1.xz, bounds1.yw);
    vec2 max1_2 = max(bounds1.xz, bounds1.yw);
    float maxmin = max(min1_2.x, min1_2.y);
    float minmax = min(max1_2.x, max1_2.y);
    float max3 = max(max1_2.x, max1_2.y);
    float max_max = max(max3, bounds2);
    float max_max2 = max(min(max3, bounds2), max(minmax, maxmin));
    return max_max + max_max2;
}

float lower_bound_lagrange5(float a0, float a1, float a2, float a3, float a4) {
    vec4 coeffs1 = vec4(-a0, a1, -a2, a3);
    vec4 neg1 = max(-coeffs1, vec4(0));
    float neg2 = max(-a4, 0.);
    const vec4 indizes1 = vec4(0, 1, 2, 3);
    vec4 bounds1 = pow(neg1, 1. / (5. - indizes1));
    float bounds2 = pow(neg2, 1. / (5. - 4.));
    vec2 min1_2 = min(bounds1.xz, bounds1.yw);
    vec2 max1_2 = max(bounds1.xz, bounds1.yw);
    float maxmin = max(min1_2.x, min1_2.y);
    float minmax = min(max1_2.x, max1_2.y);
    float max3 = max(max1_2.x, max1_2.y);
    float max_max = max(max3, bounds2);
    float max_max2 = max(min(max3, bounds2), max(minmax, maxmin));
    return -max_max - max_max2;
}

float eval_poly5(float a0, float a1, float a2, float a3, float a4, float x) {
    return ((((x + a4) * x + a3) * x + a2) * x + a1) * x + a0;
}

float halley_iteration5(float a0, float a1, float a2, float a3, float a4, float x) {
    float f  = ((((x + a4) * x + a3) * x + a2) * x + a1) * x + a0;
    float f1 = (((5. * x + 4. * a4) * x + 3. * a3) * x + 2. * a2) * x + a1;
    float f2 = ((20. * x + 12. * a4) * x + 6. * a3) * x + 2. * a2;
    return x - (2. * f * f1) / (2. * f1 * f1 - f * f2);
}

float halley_iteration4(vec4 coeffs, float x) {
    float f  = (((x + coeffs[3]) * x + coeffs[2]) * x + coeffs[1]) * x + coeffs[0];
    float f1 = ((4. * x + 3. * coeffs[3]) * x + 2. * coeffs[2]) * x + coeffs[1];
    float f2 = (12. * x + 6. * coeffs[3]) * x + 2. * coeffs[2];
    return x - (2. * f * f1) / (2. * f1 * f1 - f * f2);
}

int solve_quadric(vec2 coeffs, inout vec2 roots) {
    float p = coeffs[1] / 2.;
    float q = coeffs[0];
    float D = p * p - q;
    if (D < 0.) return 0;
    else if (D > 0.) {
        roots[0] = -sqrt(D) - p;
        roots[1] =  sqrt(D) - p;
        return 2;
    }
    return 0;
}

int solve_cubic(vec3 coeffs, inout vec3 r) {
    float a = coeffs[2];
    float b = coeffs[1];
    float c = coeffs[0];
    float p = b - a*a / 3.0;
    float q = a * (2.0*a*a - 9.0*b) / 27.0 + c;
    float p3 = p*p*p;
    float d = q*q + 4.0*p3 / 27.0;
    float offset = -a / 3.0;
    if (d >= 0.0) {
        float z = sqrt(d);
        float u = (-q + z) / 2.0;
        float v = (-q - z) / 2.0;
        u = sign(u)*pow(abs(u), 1.0/3.0);
        v = sign(v)*pow(abs(v), 1.0/3.0);
        r[0] = offset + u + v;
        float f  = ((r[0] + a) * r[0] + b) * r[0] + c;
        float f1 = (3. * r[0] + 2. * a) * r[0] + b;
        r[0] -= f / f1;
        return 1;
    }
    float u = sqrt(-p / 3.0);
    float v = acos(-sqrt(-27.0 / p3) * q / 2.0) / 3.0;
    float m = cos(v), n = sin(v)*1.732050808;
    r[0] = offset + u * (m + m);
    r[1] = offset - u * (n + m);
    r[2] = offset + u * (n - m);
    vec3 f  = ((r + a) * r + b) * r + c;
    vec3 f1 = (3. * r + 2. * a) * r + b;
    r -= f / f1;
    return 3;
}

int solve_quartic(vec4 coeffs, inout vec4 s) {
    float a = coeffs[3], b = coeffs[2], c = coeffs[1], d = coeffs[0];
    float sq_a = a * a;
    float p = -3./8. * sq_a + b;
    float q =  1./8. * sq_a * a - 1./2. * a * b + c;
    float r = -3./256.*sq_a*sq_a + 1./16.*sq_a*b - 1./4.*a*c + d;
    int num;
    vec3 cubic_coeffs;
    cubic_coeffs[0] = 1./2. * r * p - 1./8. * q * q;
    cubic_coeffs[1] = -r;
    cubic_coeffs[2] = -1./2. * p;
    solve_cubic(cubic_coeffs, s.xyz);
    float z = s[0];
    float u = z * z - r;
    float v = 2. * z - p;
    if (u > -eps) u = sqrt(abs(u)); else return 0;
    if (v > -eps) v = sqrt(abs(v)); else return 0;
    vec2 quad_coeffs;
    quad_coeffs[0] = z - u;
    quad_coeffs[1] = q < 0. ? -v : v;
    num = solve_quadric(quad_coeffs, s.xy);
    quad_coeffs[0] = z + u;
    quad_coeffs[1] = q < 0. ? v : -v;
    vec2 tmp = vec2(1e38);
    int old_num = num;
    num += solve_quadric(quad_coeffs, tmp);
    if (old_num != num) {
        if (old_num == 0) { s[0] = tmp[0]; s[1] = tmp[1]; }
        else              { s[2] = tmp[0]; s[3] = tmp[1]; }
    }
    float sub = 1./4. * a;
    for (int i = 0; i < 4; i += 2) {
        if (i < num) {
            s[i]   -= sub; s[i]   = halley_iteration4(coeffs, s[i]);
            s[i+1] -= sub; s[i+1] = halley_iteration4(coeffs, s[i+1]);
        }
    }
    return num;
}

void sort_roots3(inout vec3 roots) {
    vec3 tmp;
    tmp[0] = min(roots[0], min(roots[1], roots[2]));
    tmp[1] = max(roots[0], min(roots[1], roots[2]));
    tmp[2] = max(roots[0], max(roots[1], roots[2]));
    roots = tmp;
}

void sort_roots4(inout vec4 roots) {
    vec2 min1_2 = min(roots.xz, roots.yw);
    vec2 max1_2 = max(roots.xz, roots.yw);
    float maxmin = max(min1_2.x, min1_2.y);
    float minmax = min(max1_2.x, max1_2.y);
    roots[0] = min(min1_2.x, min1_2.y);
    roots[1] = min(maxmin, minmax);
    roots[2] = max(minmax, maxmin);
    roots[3] = max(max1_2.x, max1_2.y);
}

vec2 parametric_cub_bezier(float t, vec2 p0, vec2 p1, vec2 p2, vec2 p3) {
    vec2 a0 = (-p0 + 3.*p1 - 3.*p2 + p3);
    vec2 a1 = (3.*p0 - 6.*p1 + 3.*p2);
    vec2 a2 = (-3.*p0 + 3.*p1);
    vec2 a3 = p0;
    return (((a0 * t) + a1) * t + a2) * t + a3;
}

float cubic_bezier_dis(vec2 uv, vec2 p0, vec2 p1, vec2 p2, vec2 p3) {
    vec2 a3 = (-p0 + 3.*p1 - 3.*p2 + p3);
    vec2 a2 = (3.*p0 - 6.*p1 + 3.*p2);
    vec2 a1 = (-3.*p0 + 3.*p1);
    vec2 a0 = p0 - uv;

    float bc6 = dot(a3, a3);
    float bc5 = 2.*dot(a3, a2);
    float bc4 = dot(a2, a2) + 2.*dot(a1, a3);
    float bc3 = 2.*(dot(a1, a2) + dot(a0, a3));
    float bc2 = dot(a1, a1) + 2.*dot(a0, a2);
    float bc1 = 2.*dot(a0, a1);

    bc5 /= bc6; bc4 /= bc6; bc3 /= bc6; bc2 /= bc6; bc1 /= bc6;

    float b0 = bc1 / 6.;
    float b1 = 2.*bc2 / 6.;
    float b2 = 3.*bc3 / 6.;
    float b3 = 4.*bc4 / 6.;
    float b4 = 5.*bc5 / 6.;

    vec4 c1 = vec4(b1, 2.*b2, 3.*b3, 4.*b4) / 5.;
    vec3 c2 = vec3(c1[1], 2.*c1[2], 3.*c1[3]) / 4.;
    vec2 c3 = vec2(c2[1], 2.*c2[2]) / 3.;

    vec4 roots_drv = vec4(1e38);
    int num_roots_drv = solve_quartic(c1, roots_drv);
    sort_roots4(roots_drv);

    float ub = upper_bound_lagrange5(b0, b1, b2, b3, b4);
    float lb = lower_bound_lagrange5(b0, b1, b2, b3, b4);

    vec3 a = vec3(1e38), b = vec3(1e38), roots = vec3(1e38);
    int num_roots = 0;

    if (num_roots_drv == 4) {
        if (eval_poly5(b0,b1,b2,b3,b4, roots_drv[0]) > 0.) {
            a[0]=lb; b[0]=roots_drv[0]; num_roots=1;
        }
        if (sign(eval_poly5(b0,b1,b2,b3,b4, roots_drv[1])) != sign(eval_poly5(b0,b1,b2,b3,b4, roots_drv[2]))) {
            if (num_roots==0) { a[0]=roots_drv[1]; b[0]=roots_drv[2]; num_roots=1; }
            else              { a[1]=roots_drv[1]; b[1]=roots_drv[2]; num_roots=2; }
        }
        if (eval_poly5(b0,b1,b2,b3,b4, roots_drv[3]) < 0.) {
            if      (num_roots==0) { a[0]=roots_drv[3]; b[0]=ub; num_roots=1; }
            else if (num_roots==1) { a[1]=roots_drv[3]; b[1]=ub; num_roots=2; }
            else                   { a[2]=roots_drv[3]; b[2]=ub; num_roots=3; }
        }
    } else {
        if (num_roots_drv == 2) {
            if      (eval_poly5(b0,b1,b2,b3,b4, roots_drv[0]) < 0.) { num_roots=1; a[0]=roots_drv[1]; b[0]=ub; }
            else if (eval_poly5(b0,b1,b2,b3,b4, roots_drv[1]) > 0.) { num_roots=1; a[0]=lb; b[0]=roots_drv[0]; }
            else { num_roots=2; a[0]=lb; b[0]=roots_drv[0]; a[1]=roots_drv[1]; b[1]=ub; }
        } else {
            num_roots=1; a[0]=lb; b[0]=ub;
        }

        vec3 roots_snd_drv = vec3(1e38);
        int num_roots_snd_drv = solve_cubic(c2, roots_snd_drv);
        sort_roots3(roots_snd_drv);

        vec2 roots_trd_drv = vec2(1e38);
        int num_roots_trd_drv = 0;
        if (num_roots_snd_drv != 3)
            num_roots_trd_drv = solve_quadric(c3, roots_trd_drv);

        for (int i = 0; i < 3; i++) {
            if (i < num_roots) {
                for (int j = 0; j < 3; j += 2) {
                    if (j < num_roots_snd_drv) {
                        if (a[i] < roots_snd_drv[j] && b[i] > roots_snd_drv[j]) {
                            if (eval_poly5(b0,b1,b2,b3,b4, roots_snd_drv[j]) > 0.) b[i]=roots_snd_drv[j];
                            else a[i]=roots_snd_drv[j];
                        }
                    }
                }
                for (int j = 0; j < 2; j++) {
                    if (j < num_roots_trd_drv) {
                        if (a[i] < roots_trd_drv[j] && b[i] > roots_trd_drv[j]) {
                            if (eval_poly5(b0,b1,b2,b3,b4, roots_trd_drv[j]) > 0.) b[i]=roots_trd_drv[j];
                            else a[i]=roots_trd_drv[j];
                        }
                    }
                }
            }
        }
    }

    float d0 = 1e38;
    for (int i = 0; i < 3; i++) {
        if (i < num_roots) {
            roots[i] = .5 * (a[i] + b[i]);
            for (int j = 0; j < halley_iterations; j++)
                roots[i] = halley_iteration5(b0,b1,b2,b3,b4, roots[i]);
            roots[i] = clamp(roots[i], 0., 1.);
            vec2 to_curve = uv - parametric_cub_bezier(roots[i], p0, p1, p2, p3);
            d0 = min(d0, dot(to_curve, to_curve));
        }
    }
    return sqrt(d0);
}

void main() {
    float d = cubic_bezier_dis(v_pos, v_A, v_B, v_C, v_D);
    float half_t = v_thickness * 0.5;
    float alpha = 1.0 - smoothstep(half_t - 1.0, half_t + 1.0, d);
    if (alpha < 0.001) discard;
    color = vec4(v_color.rgb, v_color.a * alpha);
}
