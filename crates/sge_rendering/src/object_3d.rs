use std::io::Cursor;

use error_union::ErrorUnion;
use glium::{IndexBuffer, VertexBuffer};
use obj::load_obj;
use sge_color::Color;
use sge_macros::gen_ref_type;
use sge_math::transform::Transform3D;
use sge_types::{BufferError, MaterialVertex3D};
use sge_window::get_display;

use crate::{
    api::create_flat_material,
    materials::{DEFAULT_MATERIAL, Material, MaterialRef},
    pipeline::draw_queue_3d,
};

pub struct Object3D {
    pub mesh: MeshRef,
    pub material: MaterialRef,
    pub transform: Transform3D,
}

pub enum ObjectToDraw {
    Single(Object3DRef),
    Many {
        object: Object3DRef,
        transforms: Vec<Transform3D>,
    },
    WithTransform(Object3DRef, Transform3D),
}

#[derive(ErrorUnion, Debug)]
pub enum ObjectLoadingError {
    VertexBuffer(glium::vertex::BufferCreationError),
    IndexBuffer(glium::index::BufferCreationError),
    Obj(obj::ObjError),
}

impl Object3D {
    pub fn from_obj(src: &str) -> Result<Object3DRef, ObjectLoadingError> {
        Self::from_obj_with_material(src, DEFAULT_MATERIAL)
    }

    pub fn from_obj_bytes(data: &[u8]) -> Result<Object3DRef, ObjectLoadingError> {
        Self::from_obj_bytes_with_material(data, DEFAULT_MATERIAL)
    }

    pub fn from_obj_with_material(
        src: &str,
        material: MaterialRef,
    ) -> Result<Object3DRef, ObjectLoadingError> {
        Self::from_obj_bytes_with_material(src.as_bytes(), material)
    }

    pub fn from_obj_bytes_with_material(
        data: &[u8],
        material: MaterialRef,
    ) -> Result<Object3DRef, ObjectLoadingError> {
        let display = get_display();
        let buf = Cursor::new(data);
        let obj = load_obj::<MaterialVertex3D, _, _>(buf)?;

        let vertices = obj.vertices;
        let indices = obj.indices;

        let vertices = VertexBuffer::new(display, &vertices)?;
        let indices = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )?;

        let object = Self {
            mesh: Mesh { vertices, indices }.create(),
            material,
            transform: Transform3D::IDENTITY,
        };

        Ok(object.create())
    }

    pub fn compute_smooth_normals(&mut self) {
        use bevy_math::Vec3;
        use std::collections::HashMap;

        let vertices: Vec<MaterialVertex3D> = self.mesh.vertices.read().unwrap();

        let mut position_to_vertices: HashMap<[i32; 3], Vec<usize>> = HashMap::new();

        for (i, vertex) in vertices.iter().enumerate() {
            let key = [
                (vertex.position[0] * 1000.0) as i32,
                (vertex.position[1] * 1000.0) as i32,
                (vertex.position[2] * 1000.0) as i32,
            ];
            position_to_vertices.entry(key).or_default().push(i);
        }

        let mut new_normals = vec![[0.0f32; 3]; vertices.len()];

        for vertex_indices in position_to_vertices.values() {
            let mut avg_normal = Vec3::ZERO;
            for &idx in vertex_indices {
                let n = vertices[idx].normal;
                avg_normal += Vec3::new(n[0], n[1], n[2]);
            }
            avg_normal = avg_normal.normalize_or_zero();

            for &idx in vertex_indices {
                new_normals[idx] = [avg_normal.x, avg_normal.y, avg_normal.z];
            }
        }

        let new_vertices: Vec<MaterialVertex3D> = vertices
            .iter()
            .enumerate()
            .map(|(i, v)| MaterialVertex3D {
                position: v.position,
                normal: new_normals[i],
                tex_coords: v.tex_coords,
            })
            .collect();

        let display = get_display();
        self.mesh.vertices = VertexBuffer::new(display, &new_vertices).unwrap();
    }

    pub fn from_mesh_and_material(mesh: MeshRef, material: MaterialRef) -> Object3DRef {
        Self {
            mesh,
            material,
            transform: Transform3D::IDENTITY,
        }
        .create()
    }
}

gen_ref_type!(Object3D, Object3DRef, objects);

impl Object3DRef {
    pub fn draw(&self) {
        draw_queue_3d().objects.push(ObjectToDraw::Single(*self));
    }

    pub fn draw_many(&self, transforms: Vec<Transform3D>) {
        draw_queue_3d().objects.push(ObjectToDraw::Many {
            object: *self,
            transforms,
        });
    }

    pub fn draw_with_transform(&self, transform: Transform3D) {
        draw_queue_3d()
            .objects
            .push(ObjectToDraw::WithTransform(*self, transform));
    }

    pub fn with_transform(self, transform: Transform3D) -> Object3DRef {
        let object = self.get_mut();
        object.transform = transform;
        self
    }

    pub fn transform(&self) -> &mut Transform3D {
        &mut self.get_mut().transform
    }

    // pub fn vertices(&self) -> &mut Vec<MaterialVertex3D> {
    //     &mut self.get_mut().vertices
    // }

    // pub fn indices(&self) -> &mut Vec<u32> {
    //     &mut self.get_mut().indices
    // }

    pub fn material(&self) -> &mut Material {
        self.get_mut().material.get_mut()
    }
}

pub fn test_triangle() -> Result<Object3DRef, BufferError> {
    let vertices = vec![
        MaterialVertex3D {
            position: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.5, 0.0],
        },
        MaterialVertex3D {
            position: [-1.0, -1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        MaterialVertex3D {
            position: [1.0, -1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2];

    let mesh = Mesh::from_points(vertices, indices)?;

    let triangle = Object3D {
        mesh,
        material: create_flat_material(Color::RED_500),
        transform: Transform3D::IDENTITY,
    };

    Ok(triangle.create())
}

pub struct Mesh {
    pub vertices: VertexBuffer<MaterialVertex3D>,
    pub indices: IndexBuffer<u32>,
}

impl Mesh {
    pub(crate) fn from_points_internal(
        vertices: Vec<MaterialVertex3D>,
        indices: Vec<u32>,
    ) -> Result<Self, BufferError> {
        let display = get_display();
        let vertices = VertexBuffer::new(display, &vertices)?;
        let indices = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )?;

        Ok(Self { vertices, indices })
    }

    pub fn from_points(
        vertices: Vec<MaterialVertex3D>,
        indices: Vec<u32>,
    ) -> Result<MeshRef, BufferError> {
        Self::from_points_internal(vertices, indices).map(|m| m.create())
    }
}

gen_ref_type!(Mesh, MeshRef, meshes);

pub fn init() {
    init_meshes_storage();
    log::info!("Initialized mesh storage");
    init_objects_storage();
    log::info!("Initialized object storage");
}
