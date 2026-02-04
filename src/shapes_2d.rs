use crate::{
    collisions::{AABB2D, HasBounds2D},
    color::Color,
    draw_queue_2d::{DrawQueue2D, Vertex2D},
    get_state,
};
use bevy_math::Vec2;
use std::f32::consts::TAU;

pub trait Shape2D: HasBounds2D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>);
    fn is_visible_in_world(&self) -> bool {
        self.bounds().is_visible_in_world()
    }
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D);
    fn draw(&self) {
        self.add_to_draw_queue(get_state().draw_queue_2d())
    }
    fn draw_world(&self) {
        if self.is_visible_in_world() {
            self.add_to_draw_queue(get_state().world_draw_queue_2d())
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
}

impl Shape2D for Circle {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_circle(self.center, self.radius, self.color);
    }

    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

impl HasBounds2D for Circle {
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.center, self.radius * 2.0)
    }
}

impl Circle {
    pub fn encompassing_radius(&self) -> f32 {
        self.radius.x.max(self.radius.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CircleOutline {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
    pub thickness: f32,
}

impl HasBounds2D for CircleOutline {
    fn bounds(&self) -> AABB2D {
        let total_radius = self.radius + Vec2::splat(self.thickness);
        AABB2D::from_center_size(self.center, total_radius * 2.0)
    }
}

impl Shape2D for CircleOutline {
    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_circle_with_outline(
            self.center,
            self.radius,
            Color::TRANSPARENT,
            self.thickness,
            self.color,
        );
    }

    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

pub struct RoundedRectangle {
    pub top_left: Vec2,
    pub size: Vec2,
    pub fill_color: Color,
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub outline_color: Color,
}

impl HasBounds2D for RoundedRectangle {
    fn bounds(&self) -> AABB2D {
        AABB2D::new(self.top_left, self.top_left + self.size)
    }
}

impl Shape2D for RoundedRectangle {
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

    fn points(&self, _: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

impl RoundedRectangle {
    pub fn new(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) -> Self {
        Self {
            top_left,
            size,
            fill_color: color,
            corner_radius,
            outline_color: color,
            outline_thickness: 0.0,
        }
    }

    pub fn square(top_left: Vec2, size: f32, color: Color, corner_radius: f32) -> Self {
        Self::new(top_left, Vec2::splat(size), color, corner_radius)
    }

    pub fn square_with_outline(
        top_left: Vec2,
        size: f32,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self::with_outline(
            top_left,
            Vec2::splat(size),
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        )
    }

    pub fn with_outline(
        top_left: Vec2,
        size: Vec2,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            top_left,
            size,
            fill_color: color,
            corner_radius,
            outline_thickness,
            outline_color,
        }
    }

    pub fn from_center(center: Vec2, size: Vec2, color: Color, corner_radius: f32) -> Self {
        Self::new(center - size / 2.0, size, color, corner_radius)
    }

    pub fn from_center_with_outline(
        center: Vec2,
        size: Vec2,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self::with_outline(
            center - size / 2.0,
            size,
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        )
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl HasBounds2D for Rect {
    fn bounds(&self) -> AABB2D {
        AABB2D::new(self.top_left, self.top_left + self.size)
    }
}

impl Rect {
    pub fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            top_left,
            size,
            color,
        }
    }

    pub fn from_center(center: Vec2, size: Vec2, color: Color) -> Self {
        Self::new(center - size / 2.0, size, color)
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }

    pub fn new_square(top_left: Vec2, size: f32, color: Color) -> Self {
        Self::new(top_left, Vec2::splat(size), color)
    }

    pub fn from_square_center(center: Vec2, size: f32, color: Color) -> Self {
        Self::from_center(center, Vec2::splat(size), color)
    }

    fn gen_quad(&self) -> Vec<Vertex2D> {
        let tl = self.top_left;
        let br = self.top_left + self.size;

        vec![
            Vertex2D::new(tl.x, tl.y, self.color),
            Vertex2D::new(br.x, tl.y, self.color),
            Vertex2D::new(tl.x, br.y, self.color),
            Vertex2D::new(br.x, br.y, self.color),
        ]
    }
}

impl Shape2D for Rect {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let quad = self.gen_quad();
        let indices = QUAD_INDICES.map(|n| n + starting_index).to_vec();
        (indices, quad)
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub color: Color,
}

impl HasBounds2D for Triangle {
    fn bounds(&self) -> AABB2D {
        let min = self.points[0].min(self.points[1]).min(self.points[2]);
        let max = self.points[0].max(self.points[1]).max(self.points[2]);
        AABB2D::new(min, max)
    }
}

impl Shape2D for Triangle {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let tri = self.points.map(|p| Vertex2D::new(p.x, p.y, self.color));
        let indices = starting_index..starting_index + 3;
        (indices.collect(), tri.to_vec())
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line2D {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl HasBounds2D for Line2D {
    fn bounds(&self) -> AABB2D {
        let half_thick = self.thickness * 0.5;
        AABB2D::new(
            self.start.min(self.end) - Vec2::splat(half_thick),
            self.start.max(self.end) + Vec2::splat(half_thick),
        )
    }
}

impl Line2D {
    fn gen_mesh(&self) -> Option<Vec<Vertex2D>> {
        let direction = self.end - self.start;
        let length = direction.length();

        if length == 0.0 {
            return None;
        }

        let normalized = direction / length;
        let perpendicular = Vec2::new(-normalized.y, normalized.x) * self.thickness / 2.0;

        Some(vec![
            Vertex2D::new(
                self.start.x - perpendicular.x,
                self.start.y - perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.end.x - perpendicular.x,
                self.end.y - perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.start.x + perpendicular.x,
                self.start.y + perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.end.x + perpendicular.x,
                self.end.y + perpendicular.y,
                self.color,
            ),
        ])
    }
}

impl Shape2D for Line2D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        if let Some(mesh) = self.gen_mesh() {
            (QUAD_INDICES.map(|n| n + starting_index).to_vec(), mesh)
        } else {
            (vec![], vec![])
        }
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Poly {
    pub sides: usize,
    pub radius: f32,
    pub center: Vec2,
    pub rotation: f32,
    pub color: Color,
}

impl HasBounds2D for Poly {
    fn bounds(&self) -> AABB2D {
        AABB2D::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl Poly {
    pub fn gen_points(&self) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(self.sides);
        let angle_step = TAU / self.sides as f32;

        for i in 0..self.sides {
            let angle = angle_step * i as f32 + self.rotation;
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            points.push(Vec2::new(x, y));
        }

        points
    }

    pub fn gen_mesh(&self) -> (Vec<Vertex2D>, Vec<u32>) {
        let points = self.gen_points();
        gen_mesh_from_points(&points, self.color)
    }
}

impl Shape2D for Poly {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let (vertices, indices) = self.gen_mesh();
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

#[derive(Clone, Debug)]
pub struct CustomShape {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl HasBounds2D for CustomShape {
    fn bounds(&self) -> AABB2D {
        if self.points.is_empty() {
            return AABB2D::new(Vec2::ZERO, Vec2::ZERO);
        }

        let mut min = self.points[0];
        let mut max = self.points[0];

        for point in &self.points[1..] {
            min = min.min(*point);
            max = max.max(*point);
        }

        AABB2D::new(min, max)
    }
}

impl Shape2D for CustomShape {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let (vertices, indices) = gen_mesh_from_points(&self.points, self.color);
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }

    fn add_to_draw_queue(&self, draw_queue: &mut DrawQueue2D) {
        draw_queue.add_shape(self);
    }
}

pub(crate) const QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];

pub(crate) const UNIT_QUAD: [Vertex2D; 4] = [
    Vertex2D {
        position: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [-1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

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
    draw_rect: top_left: Vec2, size: Vec2, color: Color => Rect { top_left, size, color },
    draw_square: top_left: Vec2, size: f32, color: Color => Rect { top_left, size: Vec2::splat(size), color },
    draw_tri: a: Vec2, b: Vec2, c: Vec2, color: Color => Triangle { points: [a, b, c], color },
    draw_line: start: Vec2, end: Vec2, thickness: f32, color: Color => Line2D { start, end, thickness, color },
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
    get_state()
        .draw_queue_2d()
        .add_circle(center, Vec2::splat(radius), color);
}

pub fn draw_circle_world(center: Vec2, radius: f32, color: Color) {
    let circle = Circle {
        center,
        radius: Vec2::splat(radius),
        color,
    };
    if circle.bounds().is_visible_in_world() {
        get_state()
            .world_draw_queue_2d()
            .add_circle(center, Vec2::splat(radius), color);
    }
}

pub fn draw_ellipse(center: Vec2, radius: Vec2, color: Color) {
    get_state()
        .draw_queue_2d()
        .add_circle(center, radius, color);
}

pub fn draw_ellipse_world(center: Vec2, radius: Vec2, color: Color) {
    let circle = Circle {
        center,
        radius,
        color,
    };
    if circle.bounds().is_visible_in_world() {
        get_state()
            .world_draw_queue_2d()
            .add_circle(center, radius, color);
    }
}

pub fn draw_circle_outline(center: Vec2, radius: f32, outline_color: Color, thickness: f32) {
    get_state().draw_queue_2d().add_circle_with_outline(
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
        get_state().world_draw_queue_2d().add_circle_with_outline(
            center,
            Vec2::splat(radius),
            Color::new(0.0, 0.0, 0.0).with_alpha(0.0),
            thickness,
            outline_color,
        );
    }
}

pub fn draw_ellipse_outline(center: Vec2, radius: Vec2, outline_color: Color, thickness: f32) {
    get_state().draw_queue_2d().add_circle_with_outline(
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
        get_state().world_draw_queue_2d().add_circle_with_outline(
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
    get_state().draw_queue_2d().add_circle_with_outline(
        center,
        Vec2::splat(radius),
        fill,
        thickness,
        outline,
    );
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
        get_state().world_draw_queue_2d().add_circle_with_outline(
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
    get_state()
        .draw_queue_2d()
        .add_circle_with_outline(center, radius, fill, thickness, outline);
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
        get_state()
            .world_draw_queue_2d()
            .add_circle_with_outline(center, radius, fill, thickness, outline);
    }
}

pub fn draw_shape(shape: &impl Shape2D) {
    shape.draw();
}

pub fn draw_shape_world(shape: &impl Shape2D) {
    shape.draw_world();
}

fn gen_mesh_from_points(points: &[Vec2], color: Color) -> (Vec<Vertex2D>, Vec<u32>) {
    if points.len() < 3 {
        return (vec![], vec![]);
    }

    let mut polygon_builder = lyon::tessellation::path::Path::builder();
    polygon_builder.begin(lyon::math::point(points[0].x, points[0].y));
    for point in &points[1..] {
        polygon_builder.line_to(lyon::math::point(point.x, point.y));
    }
    polygon_builder.end(false);
    let polygon = polygon_builder.build();

    struct VertexConstructor {
        color: Color,
    }

    impl lyon::tessellation::FillVertexConstructor<Vertex2D> for VertexConstructor {
        fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> Vertex2D {
            let pos = vertex.position();
            Vertex2D::new(pos.x, pos.y, self.color)
        }
    }

    let mut tessellator = lyon::tessellation::FillTessellator::new();
    let mut buffers = lyon::tessellation::VertexBuffers::<Vertex2D, u32>::new();

    tessellator
        .tessellate_path(
            &polygon,
            &lyon::tessellation::FillOptions::non_zero(),
            &mut lyon::tessellation::BuffersBuilder::new(&mut buffers, VertexConstructor { color }),
        )
        .unwrap();

    let vertices = buffers.vertices;
    let indices = buffers.indices;

    (vertices, indices)
}

pub fn draw_triangle_gradient(a: Vec2, b: Vec2, c: Vec2, ca: Color, cb: Color, cc: Color) {
    let vertices = [
        Vertex2D::new(a.x, a.y, ca),
        Vertex2D::new(b.x, b.y, cb),
        Vertex2D::new(c.x, c.y, cc),
    ];

    let indices = [0, 1, 2];

    get_state().draw_queue_2d().add_mesh(&vertices, &indices);
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

    get_state()
        .draw_queue_2d()
        .add_mesh(&vertices, &QUAD_INDICES);
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

    get_state()
        .draw_queue_2d()
        .add_mesh(&vertices, &QUAD_INDICES);
}

pub fn draw_triangle_gradient_world(a: Vec2, b: Vec2, c: Vec2, ca: Color, cb: Color, cc: Color) {
    let tri = Triangle {
        points: [a, b, c],
        color: Color::TRANSPARENT,
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

    get_state()
        .world_draw_queue_2d()
        .add_mesh(&vertices, &indices);
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

    get_state()
        .world_draw_queue_2d()
        .add_mesh(&vertices, &QUAD_INDICES);
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

    get_state()
        .world_draw_queue_2d()
        .add_mesh(&vertices, &QUAD_INDICES);
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
