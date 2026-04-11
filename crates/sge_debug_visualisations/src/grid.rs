use sge_vectors::Vec2;
use sge_error_union::ErrorUnion;
use glium::{IndexBuffer, ProgramCreationError, VertexBuffer, implement_vertex};
use sge_api::shapes_2d::draw_line_world;
use sge_camera::{get_camera_2d, get_camera_2d_mut};
use sge_color::Color;
use sge_math::transform::Transform3D;
use sge_programs::{ProgramRef, load_program};
use sge_rendering::{
    materials::Material,
    object_3d::{Mesh, Object3D, Object3DRef},
};
use sge_types::MaterialVertex3D;
use sge_window::get_display;

implement_vertex!(GridVertex, position);
#[derive(Copy, Clone, Debug)]
pub struct GridVertex {
    pub position: [f32; 3],
}

#[derive(ErrorUnion, Debug)]
pub enum GridCreationError {
    VertexBuffer(glium::vertex::BufferCreationError),
    IndexBuffer(glium::index::BufferCreationError),
    Program(glium::program::ProgramCreationError),
}

pub fn create_infinite_grid() -> Result<Object3DRef, GridCreationError> {
    let display = get_display();

    let size = 1000.0;
    let vertices = vec![
        MaterialVertex3D {
            position: [-size, 0.0, -size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        MaterialVertex3D {
            position: [size, 0.0, -size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        MaterialVertex3D {
            position: [size, 0.0, size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        MaterialVertex3D {
            position: [-size, 0.0, size],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    let vertex_buffer = VertexBuffer::new(display, &vertices)?;
    let index_buffer = IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )?;

    let program = load_grid_program()?;

    let draw_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: false,
            ..Default::default()
        },
        ..Default::default()
    };

    let material = Material::new(program)
        .with_float("grid_scale", 1.0)
        .with_float("grid_size", 10.0)
        .with_color("grid_color_thin", Color::NEUTRAL_500.with_alpha(0.4))
        .with_color("grid_color_thick", Color::NEUTRAL_500.with_alpha(0.8))
        .with_color("x_axis_color", Color::RED_200)
        .with_color("z_axis_color", Color::BLUE_200)
        .with_float("axis_width", 0.02)
        .with_draw_param_overrides(draw_params)
        .create();

    let object = Object3D {
        mesh: Mesh {
            vertices: vertex_buffer,
            indices: index_buffer,
        }
        .create(),
        material,
        transform: Transform3D::IDENTITY,
        flip_normals: false,
    };

    Ok(object.create())
}

fn load_grid_program() -> Result<ProgramRef, ProgramCreationError> {
    let vertex_shader = include_str!("../../sge_programs/shaders/grid/vertex.glsl");
    let fragment_shader = include_str!("../../sge_programs/shaders/grid/fragment.glsl");

    load_program(vertex_shader, fragment_shader)
}

pub fn draw_2d_grid_world(color: Color) {
    let (min, max) = get_camera_2d_mut().visible_bounds();
    let size = max - min;

    let rough_spacing = size.x.min(size.y) / 5.0;
    let log = rough_spacing.log10();
    let large_scale = 10_f32.powi(log.ceil() as i32);
    let small_scale = 10_f32.powi(log.floor() as i32);

    let t = ((1.0 - log.fract().rem_euclid(1.0)) * 2.0).clamp(0.0, 1.0);

    let small_alpha = t;
    let large_alpha = 1.0;

    draw_grid(min, max, large_scale, large_alpha, 2.0, color);
    draw_grid(min, max, small_scale, small_alpha, 1.0, color);
}

fn draw_grid(min: Vec2, max: Vec2, scale: f32, alpha: f32, thickness_mult: f32, color: Color) {
    let size = max - min;
    let width = (size.x / scale).ceil() as usize + 1;
    let height = (size.y / scale).ceil() as usize + 1;
    let tl = (min / scale).floor() * scale;

    let faded = Color::from_rgba(color.r, color.g, color.b, color.a * alpha);
    let thickness = thickness_mult / get_camera_2d().scale();

    for x in 0..width {
        let x = x as f32 * scale + tl.x;
        draw_line_world(Vec2::new(x, min.y), Vec2::new(x, max.y), thickness, faded);
    }
    for y in 0..height {
        let y = y as f32 * scale + tl.y;
        draw_line_world(Vec2::new(min.x, y), Vec2::new(max.x, y), thickness, faded);
    }
}
