use std::ops::Index;
use std::{fmt::Debug, ops::IndexMut};

use glium::{
    Rect,
    buffer::BufferCreationError,
    implement_uniform_block, implement_vertex,
    texture::{ClientFormat, MipmapsOption, RawImage1d, Texture1d, UncompressedFloatFormat},
};
use sge_color::Color;
use sge_macros::gen_ref_type;
use sge_utils::ConstantArray;
use sge_vectors::Vec2;
use sge_window::get_display;

use crate::{Area, PatternVertex2D};

// ////////////////////////////////////////////////////////////////////////////
//                                  Rounded                                  //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct RoundedBatch {
    pub instances: Vec<RoundedInstance>,
    pub scissor: Option<glium::Rect>,
}

implement_vertex!(
    RoundedInstance,
    dimensions,
    center,
    corner_radius,
    outline_thickness,
    fill_color,
    outline_color
);
#[derive(Copy, Clone, Debug)]
pub struct RoundedInstance {
    pub dimensions: [f32; 2],
    pub center: [f32; 3],
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub fill_color: [f32; 4],
    pub outline_color: [f32; 4],
}

impl RoundedInstance {
    pub fn new(
        dimensions: Vec2,
        center: Vec2,
        z: f32,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            dimensions: dimensions.into(),
            center: [center.x, center.y, z],
            corner_radius,
            outline_thickness,
            fill_color: fill_color.for_gpu(),
            outline_color: outline_color.for_gpu(),
        }
    }
}

impl RoundedBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                   Circle                                  //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct CircleBatch {
    pub instances: Vec<CircleInstance>,
    pub scissor: Option<glium::Rect>,
}

impl CircleBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

implement_vertex!(
    CircleInstance,
    center,
    radius,
    fill_color,
    outline_thickness,
    outline_color
);
#[derive(Copy, Clone, Debug)]
pub struct CircleInstance {
    pub center: [f32; 3],
    pub radius: [f32; 2],
    pub fill_color: [f32; 4],
    pub outline_thickness: f32,
    pub outline_color: [f32; 4],
}

impl CircleInstance {
    pub fn new(center: Vec2, z: f32, radius: Vec2, fill_color: Color) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness: 0.0,
            outline_color: fill_color.for_gpu(),
        }
    }

    pub fn new_with_outline(
        center: Vec2,
        z: f32,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness,
            outline_color: outline_color.for_gpu(),
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                   Shape                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct ShapeBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub indices: Vec<u32>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl ShapeBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}

/// ///////////////////////////////////////////////////////////////////////////
//                              Radial gradient                              //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct RadialGradientInstance {
    pub center: [f32; 3],
    pub radius: [f32; 2],
    pub outline_thickness: f32,
    pub inner_color: [f32; 4],
    pub outer_color: [f32; 4],
    pub outline_color: [f32; 4],
    pub gradient_offset: [f32; 2],
}

implement_vertex!(
    RadialGradientInstance,
    center,
    radius,
    outline_thickness,
    inner_color,
    outer_color,
    outline_color,
    gradient_offset
);

impl RadialGradientInstance {
    pub fn new(center: Vec2, z: f32, radius: Vec2, inner: Color, outer: Color) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: [radius.x, radius.y],
            outline_thickness: 0.0,
            inner_color: inner.for_gpu(),
            outer_color: outer.for_gpu(),
            outline_color: [0.0; 4],
            gradient_offset: [0.0; 2],
        }
    }

    pub fn new_with_outline(
        center: Vec2,
        z: f32,
        radius: Vec2,
        inner: Color,
        outer: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: [radius.x, radius.y],
            outline_thickness,
            inner_color: inner.for_gpu(),
            outer_color: outer.for_gpu(),
            outline_color: outline_color.for_gpu(),
            gradient_offset: [0.0; 2],
        }
    }

    pub fn new_offset(
        center: Vec2,
        z: f32,
        radius: Vec2,
        inner: Color,
        outer: Color,
        gradient_offset: Vec2,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: [radius.x, radius.y],
            outline_thickness: 0.0,
            inner_color: inner.for_gpu(),
            outer_color: outer.for_gpu(),
            outline_color: [0.0; 4],
            gradient_offset: [gradient_offset.x, gradient_offset.y],
        }
    }
}

#[derive(Clone)]
pub struct RadialGradientBatch {
    pub instances: Vec<RadialGradientInstance>,
    pub scissor: Option<glium::Rect>,
}

impl RadialGradientBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                              Quadratic Bezier                             //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(QuadraticBezier, a, b, c, color, thickness);

#[derive(Clone, Copy)]
pub struct QuadraticBezier {
    pub a: [f32; 2],
    pub b: [f32; 2],
    pub c: [f32; 2],
    pub color: [f32; 4],
    pub thickness: f32,
}

impl QuadraticBezier {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, color: Color, thickness: f32) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            color: color.for_gpu(),
            thickness,
        }
    }
}

#[derive(Clone)]
pub struct QuadraticBezierBatch {
    pub instances: Vec<QuadraticBezier>,
    pub scissor: Option<glium::Rect>,
}

impl QuadraticBezierBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                Cubic Bezier                               //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(CubicBezier, a, b, c, d, color, thickness);

#[derive(Clone, Copy)]
pub struct CubicBezier {
    pub a: [f32; 2],
    pub b: [f32; 2],
    pub c: [f32; 2],
    pub d: [f32; 2],
    pub color: [f32; 4],
    pub thickness: f32,
}

impl CubicBezier {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, d: Vec2, color: Color, thickness: f32) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            d: d.into(),
            color: color.for_gpu(),
            thickness,
        }
    }
}

#[derive(Clone)]
pub struct CubicBezierBatch {
    pub instances: Vec<CubicBezier>,
    pub scissor: Option<glium::Rect>,
}

impl CubicBezierBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                 Metaballs                                 //
///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Metaball {
    pub center: [f32; 2],
    pub radius: f32,
    pub _pad: f32, // 16-byte alignment
}

impl Debug for Metaball {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Metaball(x: {}, y: {}, r: {})",
            self.center[0], self.center[1], self.radius
        ))
    }
}

impl Metaball {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
            _pad: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaballBlock {
    pub centers: [[f32; 2]; 32],
    pub radii: [f32; 32],
    pub pads: [f32; 32],
}

implement_uniform_block!(MetaballBlock, centers, radii, pads);

impl Metaballs {
    pub fn new() -> Result<Self, BufferCreationError> {
        MetaballBatch::new(None)
    }
}

gen_ref_type!(MetaballBatch, Metaballs, metaballs);

#[derive(Debug)]
pub struct MetaballBatch {
    color: Color,
    bounding_box: Area,
    data: ConstantArray<Metaball, 32>,
    texture_dirty: bool,
    bounding_box_dirty: bool,
    texture: Texture1d,
    pub scissor: Option<Rect>,
}

impl MetaballBatch {
    pub fn new(scissor: Option<Rect>) -> Result<Metaballs, BufferCreationError> {
        let texture = Texture1d::empty_with_format(
            get_display(),
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
            32,
        )
        .expect("failed to create metaball texture");

        Ok(Self {
            bounding_box: Area::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            data: ConstantArray::new(),
            texture_dirty: false,
            bounding_box_dirty: true,
            color: Color::WHITE,
            texture,
            scissor,
        }
        .create())
    }

    pub fn get_ball(&self, n: usize) -> Option<&Metaball> {
        self.data.as_slice().get(n)
    }

    pub fn get_ball_mut(&mut self, n: usize) -> Option<&mut Metaball> {
        if let Some(ball) = self.data.as_mut_slice().get_mut(n) {
            self.texture_dirty = true;
            self.bounding_box_dirty = true;
            Some(ball)
        } else {
            None
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn texture(&self) -> &Texture1d {
        &self.texture
    }

    fn update(&mut self) {
        if self.texture_dirty {
            let mut raw_data = vec![0.0f32; 32 * 4];
            for (i, ball) in self.data.as_slice().iter().enumerate() {
                raw_data[i * 4] = ball.center[0];
                raw_data[i * 4 + 1] = ball.center[1];
                raw_data[i * 4 + 2] = ball.radius;
                raw_data[i * 4 + 3] = 0.0;
            }
            let raw = RawImage1d {
                data: raw_data.into(),
                width: 32,
                format: ClientFormat::F32F32F32F32,
            };
            self.texture = Texture1d::with_format(
                get_display(),
                raw,
                UncompressedFloatFormat::F32F32F32F32,
                MipmapsOption::NoMipmap,
            )
            .expect("failed to update metaball texture");
            self.texture_dirty = false;
        }

        if self.bounding_box_dirty {
            self.recalculate_bounding_box();
            self.bounding_box_dirty = false;
        }
    }

    pub fn add_metaball(&mut self, ball: Metaball) -> Result<(), sge_utils::CapacityReached> {
        self.texture_dirty = true;
        self.grow_bounding_box(ball);
        self.data.push(ball)
    }

    pub fn remove_metaball(&mut self) -> Option<()> {
        if self.data.pop().is_some() {
            self.texture_dirty = true;
            self.bounding_box_dirty = true;
            Some(())
        } else {
            None
        }
    }

    /// this function is run by the engine internally
    pub unsafe fn init_storage() {
        init_metaballs_storage();
    }

    /// this function is run by the engine internally
    pub unsafe fn update_all() {
        for ball in get_metaballs_state() {
            ball.update();
        }
    }

    pub fn bounding_box(&self) -> Area {
        self.bounding_box
    }

    fn grow_bounding_box(&mut self, ball: Metaball) {
        let center = ball.center;
        let radius = ball.radius;

        let ball_min = Vec2::new(center[0] - radius, center[1] - radius);
        let ball_max = Vec2::new(center[0] + radius, center[1] + radius);

        let bb_min = self.bounding_box.top_left;
        let bb_max = self.bounding_box.bottom_right();

        let new_bb_min = Vec2::new(bb_min.x.min(ball_min.x), bb_min.y.min(ball_min.y));
        let new_bb_max = Vec2::new(bb_max.x.max(ball_max.x), bb_max.y.max(ball_max.y));

        self.bounding_box.top_left = new_bb_min;
        self.bounding_box.size = new_bb_max - new_bb_min;
    }

    fn recalculate_bounding_box(&mut self) {
        let slice = self.data.as_slice();
        if slice.is_empty() {
            self.bounding_box = Area::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
            return;
        }

        let mut min = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

        for ball in slice.iter() {
            let c = Vec2::new(ball.center[0], ball.center[1]);
            let r = ball.radius;
            min.x = min.x.min(c.x - r);
            min.y = min.y.min(c.y - r);
            max.x = max.x.max(c.x + r);
            max.y = max.y.max(c.y + r);
        }

        self.bounding_box = Area::new(min, max - min);
    }
}

impl Index<usize> for MetaballBatch {
    type Output = Metaball;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for MetaballBatch {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.texture_dirty = true;
        self.bounding_box_dirty = true;
        &mut self.data[index]
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                   Points                                  //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct PointBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl PointBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//                                   Lines                                   //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct LineBatch {
    pub vertices: Vec<PatternVertex2D>,
    pub indices: Vec<u32>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl LineBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}
