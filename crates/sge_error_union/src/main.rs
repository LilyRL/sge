use sge_error_union::ErrorUnion;

#[derive(ErrorUnion, Debug)]
pub enum EvilError {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
    Lock(std::fs::TryLockError),
}

fn main() {}
