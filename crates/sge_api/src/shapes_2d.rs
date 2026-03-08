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

macro_rules! define_draw_functions {
    ($($name:ident: $($param:ident: $ptype:ty),* => $constructor:expr),*,) => {
        $(
            pub fn $name($($param: $ptype),*) {
                let shape = $constructor;
                draw_shape(&shape);
            }

            paste::item! {
                pub fn [<$name _world>]($($param: $ptype),*) {
                    let shape = $constructor;
                    draw_shape_world(&shape);
                }
            }
        )*
    };
}

#[rustfmt::skip]
define_draw_functions!(
    draw_rect: top_left: Vec2, size: Vec2, color: Color => Rect { top_left, size, color, rot: 0.0, },
    draw_rect_rotation: top_left: Vec2, size: Vec2, color: Color, rot: f32 => Rect { top_left, size, color, rot },
    draw_square_rotation: top_left: Vec2, size: f32, color: Color, rot: f32 => Rect { top_left, size: Vec2::splat(size), color, rot },
    draw_tri: a: Vec2, b: Vec2, c: Vec2, color: Color => Triangle { points: [a, b, c], color, rot: 0.0 },
    draw_tri_rotation: a: Vec2, b: Vec2, c: Vec2, color: Color, rot: f32 => Triangle { points: [a, b, c], color, rot },
    draw_line: start: Vec2, end: Vec2, thickness: f32, color: Color => Line2D { start, end, thickness, color, rot: 0.0 },
    draw_line_rotation: start: Vec2, end: Vec2, thickness: f32, color: Color, rot: f32 => Line2D { start, end, thickness, color, rot },
    draw_poly: center: Vec2, sides: usize, radius: f32, rotation: f32, color: Color => Poly { center, sides, radius, rotation, color },
    draw_custom_shape: points: Vec<Vec2>, color: Color => CustomShape { points, color },
    draw_hexagon: center: Vec2, radius: f32, color: Color => Poly { center, sides: 6, radius, rotation: 0.0, color },
    draw_hexagon_pointy: center: Vec2, radius: f32, color: Color => Poly { center, sides: 6, radius, rotation: std::f32::consts::FRAC_PI_6, color },
);

pub fn draw_path(points: &[Vec2], thickness: f32, color: Color) {
    points
        .windows(2)
        .for_each(|p| draw_line(p[0], p[1], thickness, color));
}

pub fn draw_path_world(points: &[Vec2], thickness: f32, color: Color) {
    points
        .windows(2)
        .for_each(|p| draw_line_world(p[0], p[1], thickness, color));
}

pub fn draw_circle(center: Vec2, radius: f32, color: Color) {
    draw_queue_2d().add_circle(center, Vec2::splat(radius), color);
}

pub fn draw_circle_world(center: Vec2, radius: f32, color: Color) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius),
        color,
    };
    if circle.bounds().is_visible_in_world() {
        world_draw_queue_2d().add_circle(center, Vec2::splat(radius), color);
    }
}

pub fn draw_ellipse(center: Vec2, radius: Vec2, color: Color) {
    draw_queue_2d().add_circle(center, radius, color);
}

pub fn draw_ellipse_world(center: Vec2, radius: Vec2, color: Color) {
    let circle = Circle {
        center,
        radius,
        color,
    };
    if circle.bounds().is_visible_in_world() {
        world_draw_queue_2d().add_circle(center, radius, color);
    }
}

pub fn draw_circle_outline(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
    draw_queue_2d().add_circle_with_outline(
        center,
        Vec2::splat(radius),
        outline_color.with_alpha(0.0),
        thickness,
        outline_color,
    );
}

pub fn draw_circle_outline_world(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius + thickness),
        color: outline_color,
    };
    if circle.bounds().is_visible_in_world() {
        world_draw_queue_2d().add_circle_with_outline(
            center,
            Vec2::splat(radius),
            Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
            thickness,
            outline_color,
        );
    }
}

pub fn draw_ellipse_outline(center: Vec2, radius: Vec2, outline_color: Color, thickness: f32) {
    draw_queue_2d().add_circle_with_outline(
        center,
        radius,
        Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
        thickness,
        outline_color,
    );
}

pub fn draw_ellipse_outline_world(
    center: Vec2,
    radius: Vec2,
    outline_color: Color,
    thickness: f32,
) {
    let circle = Circle {
        center,
        radius: radius + Vec2::splat(thickness),
        color: outline_color,
    };
    if circle.bounds().is_visible_in_world() {
        world_draw_queue_2d().add_circle_with_outline(
            center,
            radius,
            Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
            thickness,
            outline_color,
        );
    }
}

pub fn draw_circle_with_outline(
    center: Vec2,
    radius: f32,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    draw_queue_2d().add_circle_with_outline(center, Vec2::splat(radius), fill, thickness, outline);
}

pub fn draw_circle_with_outline_world(
    center: Vec2,
    radius: f32,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius + thickness),
        color: fill,
    };
    if circle.bounds().is_visible_in_world() {
        world_draw_queue_2d().add_circle_with_outline(
            center,
            Vec2::splat(radius),
            fill,
            thickness,
            outline,
        );
    }
}

pub fn draw_ellipse_with_outline(
    center: Vec2,
    radius: Vec2,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    draw_queue_2d().add_circle_with_outline(center, radius, fill, thickness, outline);
}

pub fn draw_ellipse_with_outline_world(
    center: Vec2,
    radius: Vec2,
    fill: Color,
    outline: Color,
    thickness: f32,
) {
    let circle = Circle {
        center,
        radius: radius + Vec2::splat(thickness),
        color: fill,
    };
    if circle.bounds().is_visible_in_world() {
        world_draw_queue_2d().add_circle_with_outline(center, radius, fill, thickness, outline);
    }
}

pub fn draw_shape(shape: &impl Shape2DExt) {
    shape.draw();
}

pub fn draw_shape_world(shape: &impl Shape2DExt) {
    shape.draw_world();
}

pub fn draw_triangle_gradient(a: Vec2, b: Vec2, c: Vec2, ca: Color, cb: Color, cc: Color) {
    let vertices = [
        Vertex2D::new(a.x, a.y, ca),
        Vertex2D::new(b.x, b.y, cb),
        Vertex2D::new(c.x, c.y, cc),
    ];

    let indices = [0, 1, 2];

    draw_queue_2d().add_mesh(&vertices, &indices);
}

pub fn draw_rect_gradient(
    top_left: Vec2,
    size: Vec2,
    c_tl: Color,
    c_tr: Color,
    c_bl: Color,
    c_br: Color,
) {
    let tl = top_left;
    let tr = top_left + Vec2::new(size.x, 0.0);
    let bl = top_left + Vec2::new(0.0, size.y);
    let br = top_left + size;

    let vertices = [
        Vertex2D::new(tl.x, tl.y, c_tl),
        Vertex2D::new(tr.x, tr.y, c_tr),
        Vertex2D::new(bl.x, bl.y, c_bl),
        Vertex2D::new(br.x, br.y, c_br),
    ];

    draw_queue_2d().add_mesh(&vertices, &QUAD_INDICES);
}

pub fn draw_square_gradient_all(
    top_left: Vec2,
    size: f32,
    c_tl: Color,
    c_tr: Color,
    c_bl: Color,
    c_br: Color,
) {
    draw_rect_gradient(top_left, Vec2::splat(size), c_tl, c_tr, c_bl, c_br);
}

pub fn draw_line_gradient(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    start_color: Color,
    end_color: Color,
) {
    draw_line_gradient_extra(
        start,
        end,
        thickness,
        start_color,
        start_color,
        end_color,
        end_color,
    );
}

pub fn draw_line_gradient_extra(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    c_start_left: Color,
    c_start_right: Color,
    c_end_left: Color,
    c_end_right: Color,
) {
    let vertices: Vec<Vertex2D> = Line2D {
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
    .collect();

    draw_queue_2d().add_mesh(&vertices, &QUAD_INDICES);
}

pub fn draw_triangle_gradient_world(a: Vec2, b: Vec2, c: Vec2, ca: Color, cb: Color, cc: Color) {
    let tri = Triangle {
        points: [a, b, c],
        color: Color::TRANSPARENT,
        rot: 0.0,
    };

    if !tri.bounds().is_visible_in_world() {
        return;
    }

    let vertices = [
        Vertex2D::new(a.x, a.y, ca),
        Vertex2D::new(b.x, b.y, cb),
        Vertex2D::new(c.x, c.y, cc),
    ];

    let indices = [0, 1, 2];

    world_draw_queue_2d().add_mesh(&vertices, &indices);
}

pub fn draw_rect_gradient_world(
    top_left: Vec2,
    size: Vec2,
    c_tl: Color,
    c_tr: Color,
    c_bl: Color,
    c_br: Color,
) {
    let rect = Rect {
        top_left,
        size,
        color: Color::TRANSPARENT,
        rot: 0.0,
    };

    if !rect.bounds().is_visible_in_world() {
        return;
    }

    let tl = top_left;
    let tr = top_left + Vec2::new(size.x, 0.0);
    let bl = top_left + Vec2::new(0.0, size.y);
    let br = top_left + size;

    let vertices = [
        Vertex2D::new(tl.x, tl.y, c_tl),
        Vertex2D::new(tr.x, tr.y, c_tr),
        Vertex2D::new(bl.x, bl.y, c_bl),
        Vertex2D::new(br.x, br.y, c_br),
    ];

    world_draw_queue_2d().add_mesh(&vertices, &QUAD_INDICES);
}

pub fn draw_square_gradient_world(
    top_left: Vec2,
    size: f32,
    c_tl: Color,
    c_tr: Color,
    c_bl: Color,
    c_br: Color,
) {
    draw_rect_gradient_world(top_left, Vec2::splat(size), c_tl, c_tr, c_bl, c_br);
}

pub fn draw_line_gradient_world(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    start_color: Color,
    end_color: Color,
) {
    draw_line_gradient_extra_world(
        start,
        end,
        thickness,
        start_color,
        start_color,
        end_color,
        end_color,
    );
}

pub fn draw_line_gradient_extra_world(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    c_start_left: Color,
    c_start_right: Color,
    c_end_left: Color,
    c_end_right: Color,
) {
    let line = Line2D {
        start,
        end,
        thickness,
        color: Color::TRANSPARENT,
        rot: 0.0,
    };

    if !line.bounds().is_visible_in_world() {
        return;
    }

    let vertices: Vec<Vertex2D> = line
        .points(0)
        .1
        .into_iter()
        .zip([c_start_right, c_end_right, c_start_left, c_end_left])
        .map(|(mut v, c)| {
            v.color = c.for_gpu();
            v
        })
        .collect();

    if vertices.is_empty() {
        return;
    }

    world_draw_queue_2d().add_mesh(&vertices, &QUAD_INDICES);
}

#[inline]
pub fn draw_rect_gradient_vertical(top_left: Vec2, size: Vec2, top: Color, bottom: Color) {
    draw_rect_gradient(top_left, size, top, top, bottom, bottom);
}

#[inline]
pub fn draw_rect_gradient_horizontal(top_left: Vec2, size: Vec2, left: Color, right: Color) {
    draw_rect_gradient(top_left, size, left, right, left, right);
}

#[inline]
pub fn draw_rect_gradient_tl_br(top_left: Vec2, size: Vec2, tl: Color, br: Color) {
    let m = tl.blend_halfway(br);
    draw_rect_gradient(top_left, size, tl, m, m, br);
}

#[inline]
pub fn draw_rect_gradient_tr_bl(top_left: Vec2, size: Vec2, tr: Color, bl: Color) {
    let m = tr.blend_halfway(bl);
    draw_rect_gradient(top_left, size, m, tr, bl, m);
}

#[inline]
pub fn draw_rect_gradient_vertical_world(top_left: Vec2, size: Vec2, top: Color, bottom: Color) {
    draw_rect_gradient_world(top_left, size, top, top, bottom, bottom);
}

#[inline]
pub fn draw_rect_gradient_horizontal_world(top_left: Vec2, size: Vec2, left: Color, right: Color) {
    draw_rect_gradient_world(top_left, size, left, right, left, right);
}

#[inline]
pub fn draw_rect_gradient_tl_br_world(top_left: Vec2, size: Vec2, tl: Color, br: Color) {
    let m = tl.blend_halfway(br);
    draw_rect_gradient_world(top_left, size, tl, m, m, br);
}

#[inline]
pub fn draw_rect_gradient_tr_bl_world(top_left: Vec2, size: Vec2, tr: Color, bl: Color) {
    let m = tr.blend_halfway(bl);
    draw_rect_gradient_world(top_left, size, m, tr, bl, m);
}

#[inline]
pub fn draw_square_gradient_vertical(top_left: Vec2, size: f32, top: Color, bottom: Color) {
    draw_rect_gradient_vertical(top_left, Vec2::splat(size), top, bottom);
}

#[inline]
pub fn draw_square_gradient_horizontal(top_left: Vec2, size: f32, left: Color, right: Color) {
    draw_rect_gradient_horizontal(top_left, Vec2::splat(size), left, right);
}

#[inline]
pub fn draw_square_gradient_tl_br(top_left: Vec2, size: f32, tl: Color, br: Color) {
    draw_rect_gradient_tl_br(top_left, Vec2::splat(size), tl, br);
}

#[inline]
pub fn draw_square_gradient_tr_bl(top_left: Vec2, size: f32, tr: Color, bl: Color) {
    draw_rect_gradient_tr_bl(top_left, Vec2::splat(size), tr, bl);
}

#[inline]
pub fn draw_square_gradient_vertical_world(top_left: Vec2, size: f32, top: Color, bottom: Color) {
    draw_rect_gradient_vertical_world(top_left, Vec2::splat(size), top, bottom);
}

#[inline]
pub fn draw_square_gradient_horizontal_world(top_left: Vec2, size: f32, left: Color, right: Color) {
    draw_rect_gradient_horizontal_world(top_left, Vec2::splat(size), left, right);
}

#[inline]
pub fn draw_square_gradient_tl_br_world(top_left: Vec2, size: f32, tl: Color, br: Color) {
    draw_rect_gradient_tl_br_world(top_left, Vec2::splat(size), tl, br);
}

#[inline]
pub fn draw_square_gradient_tr_bl_world(top_left: Vec2, size: f32, tr: Color, bl: Color) {
    draw_rect_gradient_tr_bl_world(top_left, Vec2::splat(size), tr, bl);
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

pub fn draw_gradient_path(points: &[Vec2], thickness: f32, start: Color, end: Color) {
    draw_gradient_path_internal(points, thickness, start, end, draw_line_gradient);
}

pub fn draw_gradient_path_world(points: &[Vec2], thickness: f32, start: Color, end: Color) {
    draw_gradient_path_internal(points, thickness, start, end, draw_line_gradient_world);
}

pub fn draw_rounded_rect(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) {
    let rounded_rect = RoundedRectangle::new(top_left, size, color, corner_radius);
    rounded_rect.draw();
}

pub fn draw_rounded_rect_world(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) {
    let rounded_rect = RoundedRectangle::new(top_left, size, color, corner_radius);
    rounded_rect.draw_world();
}

pub fn draw_rounded_square(top_left: Vec2, size: f32, color: Color, corner_radius: f32) {
    let rounded_rect = RoundedRectangle::new(top_left, Vec2::splat(size), color, corner_radius);
    rounded_rect.draw();
}

pub fn draw_rounded_square_world(top_left: Vec2, size: f32, color: Color, corner_radius: f32) {
    let rounded_rect = RoundedRectangle::new(top_left, Vec2::splat(size), color, corner_radius);
    rounded_rect.draw_world();
}

pub fn draw_rounded_rect_with_outline(
    top_left: Vec2,
    size: Vec2,
    color: Color,
    corner_radius: f32,
    outline_thickness: f32,
    outline_color: Color,
) {
    let rounded_rect = RoundedRectangle {
        top_left,
        size,
        fill_color: color,
        corner_radius,
        outline_thickness,
        outline_color,
    };
    rounded_rect.draw();
}

pub fn draw_rounded_rect_with_outline_world(
    top_left: Vec2,
    size: Vec2,
    color: Color,
    corner_radius: f32,
    outline_thickness: f32,
    outline_color: Color,
) {
    let rounded_rect = RoundedRectangle {
        top_left,
        size,
        fill_color: color,
        corner_radius,
        outline_thickness,
        outline_color,
    };
    rounded_rect.draw_world();
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

pub fn draw_tri_outline(a: Vec2, b: Vec2, c: Vec2, thickness: f32, color: Color) {
    draw_line(a, b, thickness, color);
    draw_line(b, c, thickness, color);
    draw_line(c, a, thickness, color);

    let radius = thickness / 2.0;
    draw_circle(a, radius, color);
    draw_circle(b, radius, color);
    draw_circle(c, radius, color);
}

pub fn draw_tri_outline_world(a: Vec2, b: Vec2, c: Vec2, thickness: f32, color: Color) {
    draw_line_world(a, b, thickness, color);
    draw_line_world(b, c, thickness, color);
    draw_line_world(c, a, thickness, color);

    let radius = thickness / 2.0;
    draw_circle_world(a, radius, color);
    draw_circle_world(b, radius, color);
    draw_circle_world(c, radius, color);
}

pub fn draw_rect_outline(top_left: Vec2, size: Vec2, thickness: f32, color: Color) {
    let half_thick = thickness / 2.0;
    let top_right = top_left + Vec2::new(size.x, 0.0);
    let bottom_left = top_left + Vec2::new(0.0, size.y);
    let bottom_right = top_left + size;

    draw_line(
        top_left - Vec2::new(half_thick, 0.0),
        top_right + Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line(
        top_right + Vec2::new(0.0, -half_thick),
        bottom_right + Vec2::new(0.0, half_thick),
        thickness,
        color,
    );
    draw_line(
        bottom_right + Vec2::new(half_thick, 0.0),
        bottom_left - Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line(
        bottom_left + Vec2::new(0.0, half_thick),
        top_left - Vec2::new(0.0, -half_thick),
        thickness,
        color,
    );
}

pub fn draw_rect_outline_world(top_left: Vec2, size: Vec2, thickness: f32, color: Color) {
    let half_thick = thickness / 2.0;
    let top_right = top_left + Vec2::new(size.x, 0.0);
    let bottom_left = top_left + Vec2::new(0.0, size.y);
    let bottom_right = top_left + size;

    draw_line_world(
        top_left - Vec2::new(half_thick, 0.0),
        top_right + Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line_world(
        top_right + Vec2::new(0.0, -half_thick),
        bottom_right + Vec2::new(0.0, half_thick),
        thickness,
        color,
    );
    draw_line_world(
        bottom_right + Vec2::new(half_thick, 0.0),
        bottom_left - Vec2::new(half_thick, 0.0),
        thickness,
        color,
    );
    draw_line_world(
        bottom_left + Vec2::new(0.0, half_thick),
        top_left - Vec2::new(0.0, -half_thick),
        thickness,
        color,
    );
}

pub fn draw_square_outline(top_left: Vec2, size: f32, thickness: f32, color: Color) {
    draw_rect_outline(top_left, Vec2::splat(size), thickness, color);
}

pub fn draw_square_outline_world(top_left: Vec2, size: f32, thickness: f32, color: Color) {
    draw_rect_outline_world(top_left, Vec2::splat(size), thickness, color);
}

pub fn draw_poly_outline(
    center: Vec2,
    sides: usize,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
) {
    let poly = Poly {
        sides,
        radius,
        center,
        rotation,
        color,
    };
    let points = poly.gen_points();
    let half_thick = thickness / 2.0;

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        let dir = (end - start).normalize();

        draw_line(
            start - dir * half_thick,
            end + dir * half_thick,
            thickness,
            color,
        );
    }
}

pub fn draw_poly_outline_world(
    center: Vec2,
    sides: usize,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
) {
    let poly = Poly {
        sides,
        radius,
        center,
        rotation,
        color,
    };
    let points = poly.gen_points();
    let half_thick = thickness / 2.0;

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        let dir = (end - start).normalize();

        draw_line_world(
            start - dir * half_thick,
            end + dir * half_thick,
            thickness,
            color,
        );
    }
}

pub fn draw_arrow(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    draw_line(start, end, thickness, color);

    let dir = (end - start).normalize();
    let perp = Vec2::new(-dir.y, dir.x);
    let head_size = thickness * 4.0;

    draw_line(
        end,
        end - dir * head_size + perp * head_size / 2.0,
        thickness,
        color,
    );
    draw_line(
        end,
        end - dir * head_size - perp * head_size / 2.0,
        thickness,
        color,
    );
}

pub fn draw_arrow_world(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    draw_line_world(start, end, thickness, color);

    let dir = (end - start).normalize();
    let perp = Vec2::new(-dir.y, dir.x);
    let head_size = thickness * 4.0;

    draw_line_world(
        end,
        end - dir * head_size + perp * head_size / 2.0,
        thickness,
        color,
    );
    draw_line_world(
        end,
        end - dir * head_size - perp * head_size / 2.0,
        thickness,
        color,
    );
}
