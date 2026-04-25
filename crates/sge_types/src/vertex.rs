use glium::implement_vertex;
use obj::{FromRawVertex, raw::object::Polygon};
use sge_color::Color;
use sge_vectors::{Vec2, Vec3};

// ////////////////////////////////////////////////////////////////////////////
//                                     2D                                    //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(Vertex2D, position);
#[derive(Copy, Clone, Debug)]
pub struct Vertex2D {
    pub position: [f32; 2],
}

impl Vertex2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }

    pub fn round(self) -> Self {
        Self {
            position: [self.position[0].round(), self.position[1].round()],
        }
    }
}

implement_vertex!(ColorVertex2D, position, color);
#[derive(Copy, Clone, Debug)]
pub struct ColorVertex2D {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl ColorVertex2D {
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

    pub fn solid_pattern(self) -> PatternVertex2D {
        PatternVertex2D {
            pos: self.position,
            color: self.color,
            alt_color: self.color,
            pattern: Pattern::Fill as i32,
            scale: 1.0,
        }
    }

    pub fn to_pattern(self, alt_color: Color, pattern: Pattern, scale: f32) -> PatternVertex2D {
        PatternVertex2D {
            pos: self.position,
            color: self.color,
            alt_color: alt_color.for_gpu(),
            pattern: pattern as i32,
            scale,
        }
    }

    pub fn round(self) -> Self {
        Self {
            position: [self.position[0].round(), self.position[1].round()],
            color: self.color,
        }
    }
}

implement_vertex!(PatternVertex2D, pos, color, alt_color, pattern, scale);

#[derive(Clone, Copy, Debug)]
pub struct PatternVertex2D {
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub alt_color: [f32; 4],
    pub pattern: i32,
    pub scale: f32,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Pattern {
    Fill = 0,
    Checker,
    HorizontalLines,
    VerticalLines,
    NwseLines,
    NeswLines,
    Dots,
    Grid,
    CrossHatch,
    SparseDots,
    Bricks,
    HerringBone,
    Triangles,
    ConcentricSquares,
    Waves,
    Textured,
    ConcentricRings,
    Truchet,
    RandomTiles,
    DiagonalWaves,
}

// ////////////////////////////////////////////////////////////////////////////
//                                     3D                                    //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(Vertex3D, position, color);
#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex3D {}

// ////////////////////////////////////////////////////////////////////////////
//                                Material 3D                                //
///////////////////////////////////////////////////////////////////////////////

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

impl FromRawVertex<u32> for MaterialVertex3D {
    fn process(
        vertices: Vec<(f32, f32, f32, f32)>,
        normals: Vec<(f32, f32, f32)>,
        tex_coords: Vec<(f32, f32, f32)>,
        polygons: Vec<obj::raw::object::Polygon>,
    ) -> obj::ObjResult<(Vec<Self>, Vec<u32>)> {
        use std::collections::HashMap;

        let mut output_vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_map: HashMap<(usize, Option<usize>, Option<usize>), u32> = HashMap::new();

        let tex_coords: Vec<(f32, f32)> = tex_coords
            .into_iter()
            .map(|(u, v, _)| (u, 1.0 - v))
            .collect();

        for polygon in polygons {
            match polygon {
                Polygon::P(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &idx in &[v[0], v[i], v[i + 1]] {
                            let key = (idx, None, None);
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[idx as usize];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [0.0, 0.0, 0.0],
                                    tex_coords: [0.0, 0.0],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
                Polygon::PT(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &(pos_idx, tex_idx) in &[v[0], v[i], v[i + 1]] {
                            let key = (pos_idx, Some(tex_idx), None);
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[pos_idx];
                                let tex = tex_coords[tex_idx];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [0.0, 0.0, 0.0],
                                    tex_coords: [tex.0, tex.1],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
                Polygon::PTN(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &(pos_idx, tex_idx, norm_idx) in &[v[0], v[i], v[i + 1]] {
                            let key = (pos_idx, Some(tex_idx), Some(norm_idx));
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[pos_idx];
                                let tex = tex_coords[tex_idx];
                                let normal = normals[norm_idx];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [normal.0, normal.1, normal.2],
                                    tex_coords: [tex.0, tex.1],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
                Polygon::PN(v) => {
                    if v.len() < 3 {
                        continue;
                    }

                    for i in 1..v.len() - 1 {
                        for &(pos_idx, norm_idx) in &[v[0], v[i], v[i + 1]] {
                            let key = (pos_idx, None, Some(norm_idx));
                            let index = *vertex_map.entry(key).or_insert_with(|| {
                                let idx = output_vertices.len() as u32;
                                let pos = vertices[pos_idx];
                                let normal = normals[norm_idx];
                                output_vertices.push(MaterialVertex3D {
                                    position: [pos.0, pos.1, pos.2],
                                    normal: [normal.0, normal.1, normal.2],
                                    tex_coords: [0.0, 0.0],
                                });
                                idx
                            });
                            indices.push(index);
                        }
                    }
                }
            }
        }

        Ok((output_vertices, indices))
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                  Textured                                 //
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct TexturedVertex2D {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
implement_vertex!(TexturedVertex2D, position, tex_coords);

// ////////////////////////////////////////////////////////////////////////////
//                                   Sprite                                  //
///////////////////////////////////////////////////////////////////////////////

implement_vertex!(SpriteVertex, position, tex_coords, color);
#[derive(Copy, Clone, Debug)]
pub struct SpriteVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}
