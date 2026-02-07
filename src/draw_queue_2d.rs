use crate::api::{
    debugger_add_draw_calls, debugger_add_drawn_objects, debugger_add_indices,
    debugger_add_vertices,
};
use crate::prelude::{Transform2D, current_scissor, draw_shape};
use crate::programs::{CIRCLE_PROGRAM, FLAT_PROGRAM, ROUNDED_PROGRAM, TEXTURED_PROGRAM};
use crate::shapes_2d::{QUAD_INDICES, Shape2D, UNIT_QUAD};
use crate::textures::TextureRef;
use crate::{Color, get_state};
use bevy_math::{Mat4, Rect, Vec2, Vec3};
use glium::vertex::Vertex as GliumVertex;
use glium::{Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer, uniform};
use glium::{Depth, DepthTest, implement_vertex};
use log::info;

pub struct DrawQueue2D {
    shape_batches: Vec<ShapeBatch>,
    current_shape_batch: ShapeBatch,

    circle_batches: Vec<CircleBatch>,
    current_circle_batch: CircleBatch,

    rounded_batches: Vec<RoundedBatch>,
    current_rounded_batch: RoundedBatch,

    sprite_draws: Vec<SpriteBatch>,

    current_z: f32,
    start_z: f32,
    z_increment: f32,
}

struct RoundedBatch {
    instances: Vec<RoundedInstance>,
    scissor: Option<glium::Rect>,
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
struct RoundedInstance {
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

struct ShapeBatch {
    vertices: Vec<Vertex3D>,
    indices: Vec<u32>,
    max_index: u32,
    scissor: Option<glium::Rect>,
}

struct CircleBatch {
    instances: Vec<CircleInstance>,
    scissor: Option<glium::Rect>,
}

struct SpriteBatch {
    texture: TextureRef,
    vertices: Vec<SpriteVertex>,
    indices: Vec<u32>,
    scissor: Option<glium::Rect>,
}

implement_vertex!(SpriteVertex, position, tex_coords, color);
#[derive(Copy, Clone, Debug)]
struct SpriteVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
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

implement_vertex!(Vertex3D, position, color);
#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex3D {}

implement_vertex!(MaterialVertex3D, position, normal, tex_coords);
#[derive(Copy, Clone, Debug)]
pub struct MaterialVertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl MaterialVertex3D {
    pub fn new(pos: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position: pos.into(),
            normal: normal.into(),
            tex_coords: tex_coords.into(),
        }
    }

    pub fn from_pos(pos: Vec3) -> Self {
        Self::new(pos, Vec3::ZERO, Vec2::ZERO)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

implement_vertex!(Vertex2D, position, color);

impl Vertex2D {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            position: [x, y],
            color: color.for_gpu(),
        }
    }

    pub fn to_3d(self, z: f32) -> Vertex3D {
        Vertex3D {
            position: [self.position[0], self.position[1], z],
            color: self.color,
        }
    }
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

impl ShapeBatch {
    fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}

impl CircleBatch {
    fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

impl RoundedBatch {
    fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

impl DrawQueue2D {
    pub fn empty() -> Self {
        Self {
            shape_batches: Vec::new(),
            current_shape_batch: ShapeBatch::new(None),
            circle_batches: Vec::new(),
            current_circle_batch: CircleBatch::new(None),
            rounded_batches: Vec::new(),
            current_rounded_batch: RoundedBatch::new(None),
            sprite_draws: Vec::new(),
            current_z: 0.0,
            start_z: 0.0,
            z_increment: 0.001,
        }
    }

    pub fn with_z_config(start_z: f32, z_increment: f32) -> Self {
        Self {
            shape_batches: Vec::new(),
            current_shape_batch: ShapeBatch::new(None),
            circle_batches: Vec::new(),
            current_circle_batch: CircleBatch::new(None),
            rounded_batches: Vec::new(),
            current_rounded_batch: RoundedBatch::new(None),
            sprite_draws: Vec::new(),
            current_z: start_z,
            start_z,
            z_increment,
        }
    }

    pub fn step_z(&mut self) {
        self.current_z += self.z_increment;

        if self.current_z >= 0.998 {
            // get_state().new_draw_queues();
            // info!("max z-index exceeded. creating new draw queue");
        }
    }

    fn ensure_shape_batch(&mut self) {
        let current_scissor = current_scissor();

        if self.current_shape_batch.scissor != current_scissor {
            if !self.current_shape_batch.vertices.is_empty() {
                let old_batch = std::mem::replace(
                    &mut self.current_shape_batch,
                    ShapeBatch::new(current_scissor),
                );
                self.shape_batches.push(old_batch);
            } else {
                self.current_shape_batch.scissor = current_scissor;
            }
        }
    }

    fn ensure_circle_batch(&mut self) {
        let current_scissor = current_scissor();

        if self.current_circle_batch.scissor != current_scissor {
            if !self.current_circle_batch.instances.is_empty() {
                let old_batch = std::mem::replace(
                    &mut self.current_circle_batch,
                    CircleBatch::new(current_scissor),
                );
                self.circle_batches.push(old_batch);
            } else {
                self.current_circle_batch.scissor = current_scissor;
            }
        }
    }

    fn ensure_rounded_batch(&mut self) {
        let current_scissor = current_scissor();

        if self.current_rounded_batch.scissor != current_scissor {
            if !self.current_rounded_batch.instances.is_empty() {
                let old_batch = std::mem::replace(
                    &mut self.current_rounded_batch,
                    RoundedBatch::new(current_scissor),
                );
                self.rounded_batches.push(old_batch);
            } else {
                self.current_rounded_batch.scissor = current_scissor;
            }
        }
    }

    pub fn add_shape(&mut self, shape: &impl Shape2D) {
        self.add_shape_at_z(shape, self.current_z);
        self.step_z();
    }

    pub fn add_shape_at_z(&mut self, shape: &impl Shape2D, z: f32) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.ensure_shape_batch();

        let (mut indices, vertices) = shape.points(self.current_shape_batch.max_index);

        for vertex in &vertices {
            self.current_shape_batch.vertices.push(vertex.to_3d(z));
        }

        self.current_shape_batch.max_index += vertices.len() as u32;
        self.current_shape_batch.indices.append(&mut indices);
    }

    pub fn add_circle(&mut self, center: Vec2, radius: Vec2, color: Color) {
        self.add_circle_at_z(center, radius, color, self.current_z);
        self.step_z();
    }

    pub fn add_circle_with_outline(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) {
        self.add_circle_with_outline_at_z(
            center,
            radius,
            fill_color,
            outline_thickness,
            outline_color,
            self.current_z,
        );
        self.step_z();
    }

    pub fn add_circle_at_z(&mut self, center: Vec2, radius: Vec2, color: Color, z: f32) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.ensure_circle_batch();

        self.current_circle_batch
            .instances
            .push(CircleInstance::new(center, z, radius, color));
    }

    pub fn add_circle_with_outline_at_z(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        z: f32,
    ) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.ensure_circle_batch();

        self.current_circle_batch
            .instances
            .push(CircleInstance::new_with_outline(
                center,
                z,
                radius,
                fill_color,
                outline_thickness,
                outline_color,
            ));
    }

    pub fn add_rounded_rectangle(
        &mut self,
        center: Vec2,
        dimensions: Vec2,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) {
        self.add_rounded_rectangle_at_z(
            center,
            dimensions,
            corner_radius,
            fill_color,
            outline_thickness,
            outline_color,
            self.current_z,
        );
        self.step_z();
    }

    pub fn add_rounded_rectangle_at_z(
        &mut self,
        center: Vec2,
        dimensions: Vec2,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        z: f32,
    ) {
        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame = debug.current_frame_mut();
            frame.drawn_objects += 1;
        }

        self.ensure_rounded_batch();

        let shortest_side = dimensions.x.min(dimensions.y);

        self.current_rounded_batch
            .instances
            .push(RoundedInstance::new(
                dimensions,
                center,
                z,
                corner_radius.clamp(0.0, 0.5 * shortest_side),
                fill_color,
                outline_thickness,
                outline_color,
            ));
    }

    pub fn add_sprite(
        &mut self,
        texture: TextureRef,
        transform: Transform2D,
        color: Color,
        region: Option<Rect>,
    ) {
        self.add_sprite_at_z(texture, transform, color, region, self.current_z);
        self.step_z();
    }

    pub fn add_mesh(&mut self, vertices: &[Vertex2D], indices: &[u32]) {
        self.add_mesh_at_z(vertices, indices, self.current_z);
        self.step_z();
    }

    pub fn add_mesh_at_z(&mut self, vertices: &[Vertex2D], indices: &[u32], z: f32) {
        self.ensure_shape_batch();

        let base_index = self.current_shape_batch.max_index;

        for v in vertices {
            self.current_shape_batch.vertices.push(v.to_3d(z));
        }

        self.current_shape_batch
            .indices
            .extend(indices.iter().map(|i| i + base_index));

        self.current_shape_batch.max_index += vertices.len() as u32;
    }

    fn get_or_create_sprite_batch(&mut self, texture: TextureRef) -> &mut SpriteBatch {
        let current_scissor = current_scissor();

        let batch_index = self
            .sprite_draws
            .iter()
            .position(|b| b.texture == texture && b.scissor == current_scissor);

        if let Some(idx) = batch_index {
            &mut self.sprite_draws[idx]
        } else {
            self.sprite_draws.push(SpriteBatch {
                texture,
                vertices: Vec::new(),
                indices: Vec::new(),
                scissor: current_scissor,
            });
            self.sprite_draws.last_mut().unwrap()
        }
    }

    pub fn add_sprite_at_z(
        &mut self,
        texture: TextureRef,
        mut transform: Transform2D,
        color: Color,
        region: Option<Rect>,
        z: f32,
    ) {
        debugger_add_drawn_objects(1);

        let batch = self.get_or_create_sprite_batch(texture);
        let base_index = batch.vertices.len() as u32;

        let (tex_min_x, tex_min_y, tex_max_x, tex_max_y) = if let Some(region) = region {
            let tex = texture.get();
            let tex_width = tex.gl_texture.width() as f32;
            let tex_height = tex.gl_texture.height() as f32;

            (
                region.min.x / tex_width,
                region.min.y / tex_height,
                region.max.x / tex_width,
                region.max.y / tex_height,
            )
        } else {
            (0.0, 0.0, 1.0, 1.0)
        };

        let color_gpu = color.for_gpu();
        let mat = transform.matrix();

        let corners = [
            bevy_math::Vec3::new(0.0, 0.0, 0.0),
            bevy_math::Vec3::new(1.0, 0.0, 0.0),
            bevy_math::Vec3::new(1.0, 1.0, 0.0),
            bevy_math::Vec3::new(0.0, 1.0, 0.0),
        ];

        let tex_coords = [
            [tex_min_x, tex_min_y],
            [tex_max_x, tex_min_y],
            [tex_max_x, tex_max_y],
            [tex_min_x, tex_max_y],
        ];

        for i in 0..4 {
            let v = mat.transform_point3(corners[i]);
            batch.vertices.push(SpriteVertex {
                position: [v.x, v.y, z],
                tex_coords: tex_coords[i],
                color: color_gpu,
            });
        }

        batch.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    pub fn draw<T: Surface>(&mut self, frame: &mut T, projection: &Mat4) {
        if !self.current_shape_batch.vertices.is_empty() {
            let batch = std::mem::replace(&mut self.current_shape_batch, ShapeBatch::new(None));
            self.shape_batches.push(batch);
        }

        if !self.current_circle_batch.instances.is_empty() {
            let batch = std::mem::replace(&mut self.current_circle_batch, CircleBatch::new(None));
            self.circle_batches.push(batch);
        }

        if !self.current_rounded_batch.instances.is_empty() {
            let batch = std::mem::replace(&mut self.current_rounded_batch, RoundedBatch::new(None));
            self.rounded_batches.push(batch);
        }

        for batch in &self.shape_batches {
            if batch.vertices.is_empty() {
                continue;
            }
            self.draw_mesh_batch(frame, projection, batch, FLAT_PROGRAM.get());
        }

        for batch in &self.circle_batches {
            if batch.instances.is_empty() {
                continue;
            }
            self.draw_quad_instanced(
                frame,
                projection,
                &batch.instances,
                CIRCLE_PROGRAM.get(),
                batch.scissor,
            );
        }

        for batch in &self.rounded_batches {
            if batch.instances.is_empty() {
                continue;
            }
            self.draw_quad_instanced(
                frame,
                projection,
                &batch.instances,
                ROUNDED_PROGRAM.get(),
                batch.scissor,
            );
        }

        for batch in &self.sprite_draws {
            if !batch.vertices.is_empty() {
                self.draw_sprite_batch(frame, projection, batch);
            }
        }
    }

    fn common_draw_params(scissor: Option<glium::Rect>) -> DrawParameters<'static> {
        DrawParameters {
            blend: Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            },
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            scissor,
            ..Default::default()
        }
    }

    fn draw_mesh_batch<T: Surface>(
        &self,
        frame: &mut T,
        projection: &Mat4,
        batch: &ShapeBatch,
        program: &glium::Program,
    ) {
        let state = get_state();
        let display = &state.display;

        let params = Self::common_draw_params(batch.scissor);

        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            transform: projection.to_cols_array_2d(),
        };

        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;

            let debug = get_debug_info_mut();
            let frame_dbg = debug.current_frame_mut();
            frame_dbg.draw_calls += 1;
            frame_dbg.vertex_count += vertex_buffer.len();
            frame_dbg.index_count += index_buffer.len();
        }

        frame
            .draw(&vertex_buffer, &index_buffer, program, &uniforms, &params)
            .unwrap();
    }

    fn draw_quad_instanced<T, S>(
        &self,
        frame: &mut S,
        projection: &Mat4,
        instances: &[T],
        program: &glium::Program,
        scissor: Option<glium::Rect>,
    ) where
        T: Copy + GliumVertex,
        S: Surface,
    {
        let state = get_state();
        let display = &state.display;

        let params = Self::common_draw_params(scissor);

        let quad_buffer = VertexBuffer::new(display, &UNIT_QUAD).unwrap();
        let instance_buffer = VertexBuffer::dynamic(display, instances).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &QUAD_INDICES,
        )
        .unwrap();

        let uniforms = uniform! {
            transform: projection.to_cols_array_2d(),
        };

        #[cfg(feature = "debugging")]
        {
            use crate::debugging::get_debug_info_mut;
            let debug = get_debug_info_mut();
            let frame_dbg = debug.current_frame_mut();
            frame_dbg.draw_calls += 1;

            frame_dbg.vertex_count += quad_buffer.len() * instances.len();
            frame_dbg.index_count += index_buffer.len() * instances.len();
        }

        frame
            .draw(
                (&quad_buffer, instance_buffer.per_instance().unwrap()),
                &index_buffer,
                program,
                &uniforms,
                &params,
            )
            .unwrap();
    }

    fn draw_sprite_batch<T: Surface>(&self, frame: &mut T, projection: &Mat4, batch: &SpriteBatch) {
        let state = get_state();
        let display = &state.display;

        let texture = batch.texture.get();
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            tex: texture.gl_texture.sampled().minify_filter(texture.minify_filter).magnify_filter(texture.magnify_filter),
            projection: projection.to_cols_array_2d()
        };

        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            scissor: batch.scissor,
            ..Default::default()
        };

        debugger_add_draw_calls(1);
        debugger_add_indices(index_buffer.len());
        debugger_add_vertices(vertex_buffer.len());

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                TEXTURED_PROGRAM.get(),
                &uniforms,
                &params,
            )
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.shape_batches.clear();
        self.current_shape_batch = ShapeBatch::new(None);
        self.circle_batches.clear();
        self.current_circle_batch = CircleBatch::new(None);
        self.rounded_batches.clear();
        self.current_rounded_batch = RoundedBatch::new(None);
        self.sprite_draws.clear();
        self.current_z = self.start_z;
    }
}

pub trait Drawable {
    fn draw(&self);
}

impl<T: Shape2D> Drawable for T {
    fn draw(&self) {
        draw_shape(self);
    }
}

pub fn draw<T: Drawable>(o: T) {
    o.draw();
}
