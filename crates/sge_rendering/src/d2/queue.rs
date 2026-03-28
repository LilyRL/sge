use super::*;

pub struct DrawQueue2D {
    world: bool,
    draws: Vec<DrawCommand>,
}

impl DrawQueue2D {
    pub fn screen() -> Self {
        Self {
            world: false,
            draws: vec![],
        }
    }

    pub fn world() -> Self {
        Self {
            world: true,
            draws: vec![],
        }
    }

    pub fn renderer(&mut self) -> Renderer2D {
        Renderer2D {
            draws: &mut self.draws as *mut Vec<DrawCommand>,
            ty: if self.world {
                RendererType::World
            } else {
                RendererType::Screen
            },
        }
    }
}

impl DrawQueue2D {
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
            polygon_mode: get_polygon_mode(),
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
