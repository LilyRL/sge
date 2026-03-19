use bevy_math::{Mat4, Rect, Vec2, Vec3};
use glium::vertex::Vertex as GliumVertex;
use glium::{Blend, DrawParameters, IndexBuffer, Surface, VertexBuffer, uniform};
use glium::{Depth, DepthTest, implement_vertex};
use sge_color::Color;
use sge_config::get_dithering;
use sge_debugging::*;
use sge_math::transform::Transform2D;
use sge_programs::{
    CIRCLE_PROGRAM, FLAT_PROGRAM, RADIAL_PROGRAM, ROUNDED_PROGRAM, TEXTURED_PROGRAM,
};
use sge_shapes::d2::{QUAD_INDICES, Shape2D, UNIT_QUAD};
use sge_textures::TextureRef;
use sge_types::{
    CircleBatch, CircleInstance, RadialGradientBatch, RadialGradientInstance, RoundedBatch,
    RoundedInstance, ShapeBatch, Vertex2D,
};
use sge_window::get_display;

use crate::scissor::current_scissor;

enum DrawCommand {
    Shapes(ShapeBatch),
    Circles(CircleBatch),
    Rounded(RoundedBatch),
    Radial(RadialGradientBatch),
    Sprites(SpriteBatch),
}

pub struct DrawQueue2D {
    draws: Vec<DrawCommand>,
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

impl DrawQueue2D {
    pub fn empty() -> Self {
        Self { draws: Vec::new() }
    }

    fn current_shape_batch(&mut self) -> &mut ShapeBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws.last() {
            Some(DrawCommand::Shapes(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws
                .push(DrawCommand::Shapes(ShapeBatch::new(scissor)));
        }
        match self.draws.last_mut().unwrap() {
            DrawCommand::Shapes(b) => b,
            _ => unreachable!(),
        }
    }

    fn current_circle_batch(&mut self) -> &mut CircleBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws.last() {
            Some(DrawCommand::Circles(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws
                .push(DrawCommand::Circles(CircleBatch::new(scissor)));
        }
        match self.draws.last_mut().unwrap() {
            DrawCommand::Circles(b) => b,
            _ => unreachable!(),
        }
    }

    fn current_rounded_batch(&mut self) -> &mut RoundedBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws.last() {
            Some(DrawCommand::Rounded(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws
                .push(DrawCommand::Rounded(RoundedBatch::new(scissor)));
        }
        match self.draws.last_mut().unwrap() {
            DrawCommand::Rounded(b) => b,
            _ => unreachable!(),
        }
    }

    fn current_radial_batch(&mut self) -> &mut RadialGradientBatch {
        let scissor = current_scissor();
        let needs_new = match self.draws.last() {
            Some(DrawCommand::Radial(b)) => b.scissor != scissor,
            _ => true,
        };
        if needs_new {
            self.draws
                .push(DrawCommand::Radial(RadialGradientBatch::new(scissor)));
        }
        match self.draws.last_mut().unwrap() {
            DrawCommand::Radial(b) => b,
            _ => unreachable!(),
        }
    }

    fn current_sprite_batch(&mut self, texture: TextureRef) -> &mut SpriteBatch {
        let scissor = current_scissor();

        let can_merge = match self.draws.last() {
            Some(DrawCommand::Sprites(b)) => b.texture == texture && b.scissor == scissor,
            _ => false,
        };

        if !can_merge {
            self.draws.push(DrawCommand::Sprites(SpriteBatch {
                texture,
                vertices: Vec::new(),
                indices: Vec::new(),
                scissor,
            }));
        }

        match self.draws.last_mut().unwrap() {
            DrawCommand::Sprites(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn add_shape(&mut self, shape: &impl Shape2D) {
        debugger_add_drawn_objects(1);
        let batch = self.current_shape_batch();
        let (mut indices, vertices) = shape.gen_mesh(batch.max_index);
        for vertex in &vertices {
            batch.vertices.push(vertex.to_3d(0.0));
        }
        batch.max_index += vertices.len() as u32;
        batch.indices.append(&mut indices);
    }

    pub fn add_circle(&mut self, center: Vec2, radius: Vec2, color: Color) {
        debugger_add_drawn_objects(1);
        self.current_circle_batch()
            .instances
            .push(CircleInstance::new(center, 0.0, radius, color));
    }

    pub fn add_circle_at_z(&mut self, center: Vec2, radius: Vec2, color: Color, _z: f32) {
        self.add_circle(center, radius, color);
    }

    pub fn add_circle_with_outline(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) {
        debugger_add_drawn_objects(1);
        self.current_circle_batch()
            .instances
            .push(CircleInstance::new_with_outline(
                center,
                0.0,
                radius,
                fill_color,
                outline_thickness,
                outline_color,
            ));
    }

    pub fn add_circle_with_outline_at_z(
        &mut self,
        center: Vec2,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        _z: f32,
    ) {
        self.add_circle_with_outline(center, radius, fill_color, outline_thickness, outline_color);
    }

    pub fn add_radial_gradient(
        &mut self,
        center: Vec2,
        radius: Vec2,
        inner_color: Color,
        outer_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        gradient_offset: Vec2,
    ) {
        debugger_add_drawn_objects(1);
        self.current_radial_batch()
            .instances
            .push(RadialGradientInstance {
                center: [center.x, center.y, 0.0],
                radius: [radius.x, radius.y],
                outline_thickness,
                inner_color: inner_color.for_gpu(),
                outer_color: outer_color.for_gpu(),
                outline_color: outline_color.for_gpu(),
                gradient_offset: [gradient_offset.x, gradient_offset.y],
            });
    }

    pub fn add_radial_gradient_at_z(
        &mut self,
        center: Vec2,
        radius: Vec2,
        inner_color: Color,
        outer_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        gradient_offset: Vec2,
        _z: f32,
    ) {
        self.add_radial_gradient(
            center,
            radius,
            inner_color,
            outer_color,
            outline_thickness,
            outline_color,
            gradient_offset,
        );
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
        debugger_add_drawn_objects(1);
        let shortest_side = dimensions.x.min(dimensions.y);
        self.current_rounded_batch()
            .instances
            .push(RoundedInstance::new(
                dimensions,
                center,
                0.0,
                corner_radius.clamp(0.0, (0.5 * shortest_side).max(0.0)),
                fill_color,
                outline_thickness,
                outline_color,
            ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_rounded_rectangle_at_z(
        &mut self,
        center: Vec2,
        dimensions: Vec2,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
        _z: f32,
    ) {
        self.add_rounded_rectangle(
            center,
            dimensions,
            corner_radius,
            fill_color,
            outline_thickness,
            outline_color,
        );
    }

    pub fn add_sprite(
        &mut self,
        texture: TextureRef,
        transform: Transform2D,
        color: Color,
        region: Option<Rect>,
    ) {
        self.add_sprite_at_z(texture, transform, color, region, 0.0);
    }

    pub fn add_sprite_at_z(
        &mut self,
        texture: TextureRef,
        mut transform: Transform2D,
        color: Color,
        region: Option<Rect>,
        _z: f32,
    ) {
        debugger_add_drawn_objects(1);

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
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let tex_coords = [
            [tex_min_x, tex_min_y],
            [tex_max_x, tex_min_y],
            [tex_max_x, tex_max_y],
            [tex_min_x, tex_max_y],
        ];

        let batch = self.current_sprite_batch(texture);
        let base_index = batch.vertices.len() as u32;

        for i in 0..4 {
            let v = mat.transform_point3(corners[i]);
            batch.vertices.push(SpriteVertex {
                position: [v.x, v.y, 0.0],
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

    pub fn add_mesh(&mut self, vertices: &[Vertex2D], indices: &[u32]) {
        self.add_mesh_at_z(vertices, indices, 0.0);
    }

    pub fn add_mesh_at_z(&mut self, vertices: &[Vertex2D], indices: &[u32], _z: f32) {
        let batch = self.current_shape_batch();
        let base_index = batch.max_index;
        for v in vertices {
            batch.vertices.push(v.to_3d(0.0));
        }
        batch.indices.extend(indices.iter().map(|i| i + base_index));
        batch.max_index += vertices.len() as u32;
    }

    pub fn draw<T: Surface>(&mut self, frame: &mut T, projection: &Mat4) {
        for command in &self.draws {
            match command {
                DrawCommand::Shapes(batch) => {
                    if !batch.vertices.is_empty() {
                        self.draw_mesh_batch(frame, projection, batch, FLAT_PROGRAM.get());
                    }
                }
                DrawCommand::Circles(batch) => {
                    if !batch.instances.is_empty() {
                        self.draw_quad_instanced(
                            frame,
                            projection,
                            &batch.instances,
                            CIRCLE_PROGRAM.get(),
                            batch.scissor,
                        );
                    }
                }
                DrawCommand::Radial(batch) => {
                    if !batch.instances.is_empty() {
                        self.draw_quad_instanced(
                            frame,
                            projection,
                            &batch.instances,
                            RADIAL_PROGRAM.get(),
                            batch.scissor,
                        );
                    }
                }
                DrawCommand::Rounded(batch) => {
                    if !batch.instances.is_empty() {
                        self.draw_quad_instanced(
                            frame,
                            projection,
                            &batch.instances,
                            ROUNDED_PROGRAM.get(),
                            batch.scissor,
                        );
                    }
                }
                DrawCommand::Sprites(batch) => {
                    if !batch.vertices.is_empty() {
                        self.draw_sprite_batch(frame, projection, batch);
                    }
                }
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
                test: DepthTest::Overwrite,
                write: false,
                ..Default::default()
            },
            dithering: get_dithering(),
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
        let display = get_display();
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

        debugger_add_draw_calls(1);
        debugger_add_vertices(vertex_buffer.len());
        debugger_add_indices(index_buffer.len());

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
        let display = get_display();
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

        debugger_add_vertices(quad_buffer.len() * instances.len());
        debugger_add_indices(index_buffer.len() * instances.len());

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
        let display = get_display();
        let texture = batch.texture.get();
        let vertex_buffer = VertexBuffer::new(display, &batch.vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &batch.indices,
        )
        .unwrap();

        let uniforms = uniform! {
            tex: texture.gl_texture.sampled()
                .minify_filter(texture.minify_filter)
                .magnify_filter(texture.magnify_filter),
            projection: projection.to_cols_array_2d()
        };

        let params = Self::common_draw_params(batch.scissor);

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
        self.draws.clear();
    }
}
