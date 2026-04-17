use super::*;
use crate::NumberValue;
use sge_error_union::Union;

pub use line::LineChart;
mod line;

#[derive(Union, Debug)]
pub enum Data<T> {
    Owned(Vec<T>),
    Borrowed(*const [T]),
}

impl<T> From<&[T]> for Data<T> {
    fn from(slice: &[T]) -> Self {
        Self::Borrowed(slice as *const [T])
    }
}

impl<T> From<&Vec<T>> for Data<T> {
    fn from(vec: &Vec<T>) -> Self {
        Self::Borrowed(vec.as_slice() as *const [T])
    }
}

impl<T: NumberValue> Data<T> {
    fn as_ref(&self) -> &[T] {
        match self {
            Self::Borrowed(p) => unsafe { &**p },
            Self::Owned(v) => &v,
        }
    }
}
