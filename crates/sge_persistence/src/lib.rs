use sge_error_union::ErrorUnion;

pub use sge_persistence_macros::persistent;
pub use rkyv;

#[derive(ErrorUnion, Debug)]
pub enum Error {
    Rkyv(rkyv::rancor::Error),
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
