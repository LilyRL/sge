use super::*;
use crate::NumberValue;

pub use line::LineChart;
mod line;

#[derive(Debug)]
struct Data<T: NumberValue> {
    data: *const [T],
}

impl<T: NumberValue> Data<T> {
    fn new(ref_: &[T]) -> Self {
        Self {
            data: ref_ as *const [T],
        }
    }

    fn as_ref(&self) -> &'static [T] {
        unsafe { &*self.data }
    }
}
