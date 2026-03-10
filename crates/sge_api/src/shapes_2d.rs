use bevy_math::Vec2;
use sge_color::Color;
use sge_math::collision::{self, HasBounds2D, Polygon};
use sge_rendering::{
    d2::DrawQueue2D,
    pipeline::{draw_queue_2d, world_draw_queue_2d},
};
use sge_shapes::d2::*;
use sge_types::Vertex2D;

pub trait Shape2DExt: Shape2D {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D);
    fn draw(&self) {
        self.add_to_draw_queue(draw_queue_2d())
    }
    fn draw_world(&self) {
        if self.is_visible_in_world() {
            self.add_to_draw_queue(world_draw_queue_2d())
        }
    }
}

impl Shape2DExt for Circle {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_circle(self.center, self.radius, self.color);
    }
}
impl Shape2DExt for CircleOutline {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_circle_with_outline(
            self.center,
            self.radius,
            Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
            self.thickness,
            self.color,
        );
    }
}
impl Shape2DExt for Rect {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}
impl Shape2DExt for Triangle {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}
impl Shape2DExt for Line2D {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}
impl Shape2DExt for Poly {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}
impl Shape2DExt for CustomShape {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}
impl Shape2DExt for RoundedRectangle {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_rounded_rectangle(
            self.center(),
            self.size,
            self.corner_radius,
            self.fill_color,
            self.outline_thickness,
            self.outline_color,
        );
    }
}

macro_rules! draw_variants {
    (
        fn $name:ident ( $($param:ident : $ptype:ty),* $(,)? ) {
            screen { $($sbody:tt)* }
            world  { $($wbody:tt)* }
        }
    ) => {
        paste::paste! {
            pub fn [<draw_ $name>]($($param: $ptype),*) {
                $($sbody)*
            }
            pub fn [<draw_ $name _world>]($($param: $ptype),*) {
                $($wbody)*
            }
            pub fn [<draw_ $name _to>]($($param: $ptype,)* world: bool) {
                if world { [<draw_ $name _world>]($($param),*) }
                else     { [<draw_ $name>]($($param),*) }
            }
        }
    };

    (
        fn $name:ident ( $($param:ident : $ptype:ty),* $(,)? ) { $($body:tt)* }
    ) => {
        paste::paste! {
            pub fn [<draw_ $name>]($($param: $ptype),*) {
                let __q = draw_queue_2d();
                { let draw_queue = __q; $($body)* }
            }
            pub fn [<draw_ $name _world>]($($param: $ptype),*) {
                let __q = world_draw_queue_2d();
                { let draw_queue = __q; $($body)* }
            }
            pub fn [<draw_ $name _to>]($($param: $ptype,)* world: bool) {
                if world { [<draw_ $name _world>]($($param),*) }
                else     { [<draw_ $name>]($($param),*) }
            }
        }
    };
}

macro_rules! draw_shape_variants {
    (
        $( $name:ident : $($param:ident : $ptype:ty),* => $constructor:expr ),* $(,)?
    ) => {
        $(
            paste::paste! {
                pub fn [<draw_ $name>]($($param: $ptype),*) {
                    draw_shape(&$constructor);
                }
                pub fn [<draw_ $name _world>]($($param: $ptype),*) {
                    draw_shape_world(&$constructor);
                }
                pub fn [<draw_ $name _to>]($($param: $ptype,)* world: bool) {
                    if world { [<draw_ $name _world>]($($param),*) }
                    else     { [<draw_ $name>]($($param),*) }
                }
            }
        )*
    };
}

#[rustfmt::skip]
draw_shape_variants! {
    rect:              top_left: Vec2, size: Vec2, color: Color
        => Rect { top_left, size, color, rot: 0.0 },

    rect_rotation:     top_left: Vec2, size: Vec2, color: Color, rot: f32
        => Rect { top_left, size, color, rot },

    square:            top_left: Vec2, size: f32, color: Color
        => Rect { top_left, size: Vec2::splat(size), color, rot: 0.0 },

    square_rotation:   top_left: Vec2, size: f32, color: Color, rot: f32
        => Rect { top_left, size: Vec2::splat(size), color, rot },

    tri:               a: Vec2, b: Vec2, c: Vec2, color: Color
        => Triangle { points: [a, b, c], color, rot: 0.0 },

    tri_rotation:      a: Vec2, b: Vec2, c: Vec2, color: Color, rot: f32
        => Triangle { points: [a, b, c], color, rot },

    line:              start: Vec2, end: Vec2, thickness: f32, color: Color
        => Line2D { start, end, thickness, color, rot: 0.0 },

    line_rotation:     start: Vec2, end: Vec2, thickness: f32, color: Color, rot: f32
        => Line2D { start, end, thickness, color, rot },

    poly:              center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color
        => Poly { center, sides, radius, rotation, color },

    custom_shape:      points: Vec<Vec2>, color: Color
        => CustomShape { points, color },

    hexagon:           center: Vec2, radius: f32, color: Color
        => Poly { center, sides: 6, radius, rotation: 0.0, color },

    hexagon_pointy:    center: Vec2, radius: f32, color: Color
        => Poly { center, sides: 6, radius, rotation: std::f32::consts::FRAC_PI_6, color },
}

draw_variants! {
    fn circle(center: Vec2, radius: f32, color: Color) {
        screen { draw_queue_2d().add_circle(center, Vec2::splat(radius), color); }
        world  {
            let shape = Circle { center, radius: Vec2::splat(radius), color };
            if shape.bounds().is_visible_in_world() {
                world_draw_queue_2d().add_circle(center, Vec2::splat(radius), color);
            }
        }
    }
}

draw_variants! {
    fn ellipse(center: Vec2, radius: Vec2, color: Color) {
        screen { draw_queue_2d().add_circle(center, radius, color); }
        world  {
            let shape = Circle { center, radius, color };
            if shape.bounds().is_visible_in_world() {
                world_draw_queue_2d().add_circle(center, radius, color);
            }
        }
    }
}

draw_variants! {
    fn circle_outline(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
        screen {
            draw_queue_2d().add_circle_with_outline(
                center, Vec2::splat(radius),
                outline_color.with_alpha(0.0), thickness, outline_color,
            );
        }
        world {
            let shape = Circle { center, radius: Vec2::splat(radius + thickness), color: outline_color };
            if shape.bounds().is_visible_in_world() {
                world_draw_queue_2d().add_circle_with_outline(
                    center, Vec2::splat(radius),
                    Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                );
            }
        }
    }
}

draw_variants! {
    fn ellipse_outline(center: Vec2, radius: Vec2, outline_color: Color, thickness: f32) {
        screen {
            draw_queue_2d().add_circle_with_outline(
                center, radius,
                Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
            );
        }
        world {
            let shape = Circle { center, radius: radius + Vec2::splat(thickness), color: outline_color };
            if shape.bounds().is_visible_in_world() {
                world_draw_queue_2d().add_circle_with_outline(
                    center, radius,
                    Color::new(0.0, 0.0, 0.0).with_alpha(0.0), thickness, outline_color,
                );
            }
        }
    }
}

draw_variants! {
    fn circle_with_outline(center: Vec2, radius: f32, fill: Color, outline: Color, thickness: f32) {
        screen {
            draw_queue_2d().add_circle_with_outline(center, Vec2::splat(radius), fill, thickness, outline);
        }
        world {
            let shape = Circle { center, radius: Vec2::splat(radius + thickness), color: fill };
            if shape.bounds().is_visible_in_world() {
                world_draw_queue_2d().add_circle_with_outline(
                    center, Vec2::splat(radius), fill, thickness, outline,
                );
            }
        }
    }
}

draw_variants! {
    fn ellipse_with_outline(center: Vec2, radius: Vec2, fill: Color, outline: Color, thickness: f32) {
        screen {
            draw_queue_2d().add_circle_with_outline(center, radius, fill, thickness, outline);
        }
        world {
            let shape = Circle { center, radius: radius + Vec2::splat(thickness), color: fill };
            if shape.bounds().is_visible_in_world() {
                world_draw_queue_2d().add_circle_with_outline(center, radius, fill, thickness, outline);
            }
        }
    }
}

draw_variants! {
    fn path(points: &[Vec2], thickness: f32, color: Color) {
        screen { points.windows(2).for_each(|p| draw_line(p[0], p[1], thickness, color)); }
        world  { points.windows(2).for_each(|p| draw_line_world(p[0], p[1], thickness, color)); }
    }
}

draw_variants! {
    fn tri_outline(a: Vec2, b: Vec2, c: Vec2, thickness: f32, color: Color) {
        screen {
            draw_line(a, b, thickness, color);
            draw_line(b, c, thickness, color);
            draw_line(c, a, thickness, color);
            let r = thickness / 2.0;
            draw_circle(a, r, color); draw_circle(b, r, color); draw_circle(c, r, color);
        }
        world {
            draw_line_world(a, b, thickness, color);
            draw_line_world(b, c, thickness, color);
            draw_line_world(c, a, thickness, color);
            let r = thickness / 2.0;
            draw_circle_world(a, r, color); draw_circle_world(b, r, color); draw_circle_world(c, r, color);
        }
    }
}

draw_variants! {
    fn rect_outline(top_left: Vec2, size: Vec2, thickness: f32, color: Color) {
        screen {
            let ht = thickness / 2.0;
            let tr = top_left + Vec2::new(size.x, 0.0);
            let bl = top_left + Vec2::new(0.0, size.y);
            let br = top_left + size;
            draw_line(top_left - Vec2::new(ht, 0.0), tr + Vec2::new(ht, 0.0),           thickness, color);
            draw_line(tr + Vec2::new(0.0, -ht),      br + Vec2::new(0.0, ht),             thickness, color);
            draw_line(br + Vec2::new(ht, 0.0),       bl - Vec2::new(ht, 0.0),             thickness, color);
            draw_line(bl + Vec2::new(0.0, ht),       top_left - Vec2::new(0.0, -ht),      thickness, color);
        }
        world {
            let ht = thickness / 2.0;
            let tr = top_left + Vec2::new(size.x, 0.0);
            let bl = top_left + Vec2::new(0.0, size.y);
            let br = top_left + size;
            draw_line_world(top_left - Vec2::new(ht, 0.0), tr + Vec2::new(ht, 0.0),      thickness, color);
            draw_line_world(tr + Vec2::new(0.0, -ht),      br + Vec2::new(0.0, ht),       thickness, color);
            draw_line_world(br + Vec2::new(ht, 0.0),       bl - Vec2::new(ht, 0.0),       thickness, color);
            draw_line_world(bl + Vec2::new(0.0, ht),       top_left - Vec2::new(0.0,-ht), thickness, color);
        }
    }
}

draw_variants! {
    fn square_outline(top_left: Vec2, size: f32, thickness: f32, color: Color) {
        screen { draw_rect_outline(top_left, Vec2::splat(size), thickness, color); }
        world  { draw_rect_outline_world(top_left, Vec2::splat(size), thickness, color); }
    }
}

draw_variants! {
    fn poly_outline(center: Vec2, sides: usize, radius: f32, rotation: f32, thickness: f32, color: Color) {
        screen {
            let points = Poly { sides, radius, center, rotation, color }.gen_points();
            let ht = thickness / 2.0;
            for i in 0..points.len() {
                let (s, e) = (points[i], points[(i + 1) % points.len()]);
                let dir = (e - s).normalize();
                draw_line(s - dir * ht, e + dir * ht, thickness, color);
            }
        }
        world {
            let points = Poly { sides, radius, center, rotation, color }.gen_points();
            let ht = thickness / 2.0;
            for i in 0..points.len() {
                let (s, e) = (points[i], points[(i + 1) % points.len()]);
                let dir = (e - s).normalize();
                draw_line_world(s - dir * ht, e + dir * ht, thickness, color);
            }
        }
    }
}

draw_variants! {
    fn arrow(start: Vec2, end: Vec2, thickness: f32, color: Color) {
        screen {
            draw_line(start, end, thickness, color);
            let dir = (end - start).normalize();
            let perp = Vec2::new(-dir.y, dir.x);
            let h = thickness * 4.0;
            draw_line(end, end - dir * h + perp * h / 2.0, thickness, color);
            draw_line(end, end - dir * h - perp * h / 2.0, thickness, color);
        }
        world {
            draw_line_world(start, end, thickness, color);
            let dir = (end - start).normalize();
            let perp = Vec2::new(-dir.y, dir.x);
            let h = thickness * 4.0;
            draw_line_world(end, end - dir * h + perp * h / 2.0, thickness, color);
            draw_line_world(end, end - dir * h - perp * h / 2.0, thickness, color);
        }
    }
}

draw_variants! {
    fn rounded_rect(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) {
        screen { draw_shape(&RoundedRectangle::new(top_left, size, color, corner_radius)); }
        world  { draw_shape_world(&RoundedRectangle::new(top_left, size, color, corner_radius)); }
    }
}

draw_variants! {
    fn rounded_square(top_left: Vec2, size: f32, color: Color, corner_radius: f32) {
        screen { draw_shape(&RoundedRectangle::new(top_left, Vec2::splat(size), color, corner_radius)); }
        world  { draw_shape_world(&RoundedRectangle::new(top_left, Vec2::splat(size), color, corner_radius)); }
    }
}

draw_variants! {
    fn rounded_rect_with_outline(
        top_left: Vec2, size: Vec2, color: Color, corner_radius: f32,
        outline_thickness: f32, outline_color: Color,
    ) {
        screen {
            draw_shape(&RoundedRectangle { top_left, size, fill_color: color, corner_radius,
                outline_thickness, outline_color });
        }
        world {
            draw_shape_world(&RoundedRectangle { top_left, size, fill_color: color, corner_radius,
                outline_thickness, outline_color });
        }
    }
}

fn rect_gradient_mesh(
    top_left: Vec2,
    size: Vec2,
    c_tl: Color,
    c_tr: Color,
    c_bl: Color,
    c_br: Color,
) -> [Vertex2D; 4] {
    let tl = top_left;
    let tr = top_left + Vec2::new(size.x, 0.0);
    let bl = top_left + Vec2::new(0.0, size.y);
    let br = top_left + size;
    [
        Vertex2D::new(tl.x, tl.y, c_tl),
        Vertex2D::new(tr.x, tr.y, c_tr),
        Vertex2D::new(bl.x, bl.y, c_bl),
        Vertex2D::new(br.x, br.y, c_br),
    ]
}

draw_variants! {
    fn triangle_gradient(a: Vec2, b: Vec2, c: Vec2, ca: Color, cb: Color, cc: Color) {
        screen {
            draw_queue_2d().add_mesh(
                &[Vertex2D::new(a.x, a.y, ca), Vertex2D::new(b.x, b.y, cb), Vertex2D::new(c.x, c.y, cc)],
                &[0, 1, 2],
            );
        }
        world {
            let tri = Triangle { points: [a, b, c], color: Color::TRANSPARENT, rot: 0.0 };
            if !tri.bounds().is_visible_in_world() { return; }
            world_draw_queue_2d().add_mesh(
                &[Vertex2D::new(a.x, a.y, ca), Vertex2D::new(b.x, b.y, cb), Vertex2D::new(c.x, c.y, cc)],
                &[0, 1, 2],
            );
        }
    }
}

draw_variants! {
    fn rect_gradient(
        top_left: Vec2, size: Vec2,
        c_tl: Color, c_tr: Color, c_bl: Color, c_br: Color,
    ) {
        screen {
            draw_queue_2d().add_mesh(&rect_gradient_mesh(top_left, size, c_tl, c_tr, c_bl, c_br), &QUAD_INDICES);
        }
        world {
            let rect = Rect { top_left, size, color: Color::TRANSPARENT, rot: 0.0 };
            if !rect.bounds().is_visible_in_world() { return; }
            world_draw_queue_2d().add_mesh(&rect_gradient_mesh(top_left, size, c_tl, c_tr, c_bl, c_br), &QUAD_INDICES);
        }
    }
}

draw_variants! {
    fn square_gradient_all(
        top_left: Vec2, size: f32,
        c_tl: Color, c_tr: Color, c_bl: Color, c_br: Color,
    ) {
        screen { draw_rect_gradient(top_left, Vec2::splat(size), c_tl, c_tr, c_bl, c_br); }
        world  { draw_rect_gradient_world(top_left, Vec2::splat(size), c_tl, c_tr, c_bl, c_br); }
    }
}

fn line_gradient_vertices(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    c_start_left: Color,
    c_start_right: Color,
    c_end_left: Color,
    c_end_right: Color,
) -> Vec<Vertex2D> {
    Line2D {
        start,
        end,
        thickness,
        color: Color::TRANSPARENT,
        rot: 0.0,
    }
    .points(0)
    .1
    .into_iter()
    .zip([c_start_right, c_end_right, c_start_left, c_end_left])
    .map(|(mut v, c)| {
        v.color = c.for_gpu();
        v
    })
    .collect()
}

draw_variants! {
    fn line_gradient(start: Vec2, end: Vec2, thickness: f32, start_color: Color, end_color: Color) {
        screen {
            draw_line_gradient_extra(start, end, thickness, start_color, start_color, end_color, end_color);
        }
        world {
            draw_line_gradient_extra_world(start, end, thickness, start_color, start_color, end_color, end_color);
        }
    }
}

draw_variants! {
    fn line_gradient_extra(
        start: Vec2, end: Vec2, thickness: f32,
        c_start_left: Color, c_start_right: Color,
        c_end_left: Color, c_end_right: Color,
    ) {
        screen {
            let verts = line_gradient_vertices(start, end, thickness, c_start_left, c_start_right, c_end_left, c_end_right);
            draw_queue_2d().add_mesh(&verts, &QUAD_INDICES);
        }
        world {
            let line = Line2D { start, end, thickness, color: Color::TRANSPARENT, rot: 0.0 };
            if !line.bounds().is_visible_in_world() { return; }
            let verts = line_gradient_vertices(start, end, thickness, c_start_left, c_start_right, c_end_left, c_end_right);
            if verts.is_empty() { return; }
            world_draw_queue_2d().add_mesh(&verts, &QUAD_INDICES);
        }
    }
}

draw_variants! {
    fn gradient_path(points: &[Vec2], thickness: f32, start: Color, end: Color) {
        screen {
            draw_gradient_path_internal(points, thickness, start, end, draw_line_gradient);
        }
        world {
            draw_gradient_path_internal(points, thickness, start, end, draw_line_gradient_world);
        }
    }
}

fn draw_gradient_path_internal<F>(
    points: &[Vec2],
    thickness: f32,
    start: Color,
    end: Color,
    mut draw_seg: F,
) where
    F: FnMut(Vec2, Vec2, f32, Color, Color),
{
    if points.len() < 2 {
        return;
    }
    let total_len: f32 = points.windows(2).map(|p| (p[1] - p[0]).length()).sum();
    if total_len == 0.0 {
        return;
    }
    let mut acc = 0.0;
    for seg in points.windows(2) {
        let len = (seg[1] - seg[0]).length();
        let t0 = acc / total_len;
        let t1 = (acc + len) / total_len;
        draw_seg(
            seg[0],
            seg[1],
            thickness,
            start.blend(end, t0),
            start.blend(end, t1),
        );
        acc += len;
    }
}

macro_rules! draw_rect_gradient_dirs {
    ($stem:ident, $size_ty:ty, $to_vec2:expr) => {
        paste::paste! {
            draw_variants! {
                fn [<$stem _gradient_vertical>](top_left: Vec2, size: $size_ty, top: Color, bottom: Color) {
                    screen { draw_rect_gradient(top_left, $to_vec2(size), top, top, bottom, bottom); }
                    world  { draw_rect_gradient_world(top_left, $to_vec2(size), top, top, bottom, bottom); }
                }
            }
            draw_variants! {
                fn [<$stem _gradient_horizontal>](top_left: Vec2, size: $size_ty, left: Color, right: Color) {
                    screen { draw_rect_gradient(top_left, $to_vec2(size), left, right, left, right); }
                    world  { draw_rect_gradient_world(top_left, $to_vec2(size), left, right, left, right); }
                }
            }
            draw_variants! {
                fn [<$stem _gradient_tl_br>](top_left: Vec2, size: $size_ty, tl: Color, br: Color) {
                    screen {
                        let m = tl.blend_halfway(br);
                        draw_rect_gradient(top_left, $to_vec2(size), tl, m, m, br);
                    }
                    world {
                        let m = tl.blend_halfway(br);
                        draw_rect_gradient_world(top_left, $to_vec2(size), tl, m, m, br);
                    }
                }
            }
            draw_variants! {
                fn [<$stem _gradient_tr_bl>](top_left: Vec2, size: $size_ty, tr: Color, bl: Color) {
                    screen {
                        let m = tr.blend_halfway(bl);
                        draw_rect_gradient(top_left, $to_vec2(size), m, tr, bl, m);
                    }
                    world {
                        let m = tr.blend_halfway(bl);
                        draw_rect_gradient_world(top_left, $to_vec2(size), m, tr, bl, m);
                    }
                }
            }
        }
    };
}

draw_rect_gradient_dirs!(square, f32, Vec2::splat);
draw_rect_gradient_dirs!(rect, Vec2, |s: Vec2| s);

macro_rules! gen_radial_gradient_variants {
    () => {
        gen_radial_gradient_variants!(@shape circle,  radius: f32,  Vec2::splat(radius));
        gen_radial_gradient_variants!(@shape ellipse, radius: Vec2, radius);
    };

    (@shape $name:ident, $radius_param:ident: $radius_ty:ty, $radius_expr:expr) => {
        // no outline, no offset
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [],
            outline_params: [],
            outline_val:    [0.0, Color::TRANSPARENT],
            offset_params:  [],
            offset_val:     [Vec2::ZERO],
            vis_radius:     [$radius_expr]
        );
        // outline only
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [_with_outline],
            outline_params: [outline_thickness: f32, outline_color: Color],
            outline_val:    [outline_thickness, outline_color],
            offset_params:  [],
            offset_val:     [Vec2::ZERO],
            vis_radius:     [$radius_expr + Vec2::splat(outline_thickness)]
        );
        // offset only
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [_offset],
            outline_params: [],
            outline_val:    [0.0, Color::TRANSPARENT],
            offset_params:  [gradient_offset: Vec2],
            offset_val:     [gradient_offset],
            vis_radius:     [$radius_expr]
        );
        // outline + offset
        gen_radial_gradient_variants!(@emit $name, $radius_param: $radius_ty, $radius_expr,
            fn_suffix:      [_with_outline_offset],
            outline_params: [outline_thickness: f32, outline_color: Color],
            outline_val:    [outline_thickness, outline_color],
            offset_params:  [gradient_offset: Vec2],
            offset_val:     [gradient_offset],
            vis_radius:     [$radius_expr + Vec2::splat(outline_thickness)]
        );
    };

    (
        @emit $name:ident, $radius_param:ident: $radius_ty:ty, $radius_expr:expr,
        fn_suffix:      [$($suffix:tt)*],
        outline_params: [$($outline_param:ident: $outline_ty:ty),*],
        outline_val:    [$outline_thickness_val:expr, $outline_color_val:expr],
        offset_params:  [$($offset_param:ident: $offset_ty:ty),*],
        offset_val:     [$offset_val:expr],
        vis_radius:     [$vis_radius:expr]
    ) => {
        paste::paste! {
            draw_variants! {
                fn [<radial_gradient_ $name $($suffix)*>](
                    center: Vec2,
                    $radius_param: $radius_ty,
                    inner: Color,
                    outer: Color,
                    $($outline_param: $outline_ty,)*
                    $($offset_param: $offset_ty,)*
                ) {
                    screen {
                        draw_queue_2d().add_radial_gradient(
                            center, $radius_expr, inner, outer,
                            $outline_thickness_val, $outline_color_val, $offset_val,
                        );
                    }
                    world {
                        let shape = Circle { center, radius: $vis_radius, color: outer };
                        if shape.bounds().is_visible_in_world() {
                            world_draw_queue_2d().add_radial_gradient(
                                center, $radius_expr, inner, outer,
                                $outline_thickness_val, $outline_color_val, $offset_val,
                            );
                        }
                    }
                }
            }
        }
    };
}

gen_radial_gradient_variants!();

pub fn draw_shape(shape: &impl Shape2DExt) {
    shape.draw();
}

pub fn draw_shape_world(shape: &impl Shape2DExt) {
    shape.draw_world();
}

pub trait ToCollider<T> {
    fn to_collider(&self) -> T;
}

impl ToCollider<collision::Circle> for Circle {
    fn to_collider(&self) -> collision::Circle {
        collision::Circle {
            center: self.center,
            radius: self.encompassing_radius(),
        }
    }
}

impl ToCollider<Polygon> for Poly {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.gen_points(),
        }
    }
}

impl ToCollider<Polygon> for CustomShape {
    fn to_collider(&self) -> Polygon {
        Polygon {
            vertices: self.points.clone(),
        }
    }
}
