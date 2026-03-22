pub use area::*;
pub mod area;

use error_union::ErrorUnion;
pub use rendering::*;
pub mod rendering;

pub use vertex::*;
pub mod vertex;

pub enum Verbosity {
    Low,
    Medium,
    High,
}

#[derive(ErrorUnion, Debug)]
pub enum BufferError {
    Vertex(glium::vertex::BufferCreationError),
    Index(glium::index::BufferCreationError),
}
