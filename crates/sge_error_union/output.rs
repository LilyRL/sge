#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub enum EvilError {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
    Lock(std::fs::TryLockError),
}
#[automatically_derived]
impl ::core::fmt::Debug for EvilError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EvilError::Fmt(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Fmt", &__self_0)
            }
            EvilError::Io(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Io", &__self_0)
            }
            EvilError::Lock(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Lock", &__self_0)
            }
        }
    }
}
impl From<std::fmt::Error> for EvilError {
    fn from(value: std::fmt::Error) -> Self {
        Self::Fmt(value)
    }
}
impl From<std::io::Error> for EvilError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<std::fs::TryLockError> for EvilError {
    fn from(value: std::fs::TryLockError) -> Self {
        Self::Lock(value)
    }
}
impl std::fmt::Display for EvilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Fmt(e) => f.write_fmt(format_args!("{0}: {1}", "Fmt", e)),
            Self::Io(e) => f.write_fmt(format_args!("{0}: {1}", "Io", e)),
            Self::Lock(e) => f.write_fmt(format_args!("{0}: {1}", "Lock", e)),
        }
    }
}
impl std::error::Error for EvilError {}
fn main() {}
