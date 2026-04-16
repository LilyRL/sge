use crate::{NumberValue, UiRef};

pub struct LineChart;

impl LineChart {
    pub fn new<T: NumberValue>(data: &[T], width: f32, height: f32) -> UiRef {
        crate::base::LineChart::new(&data, super::FG3)
            .padding(4.0)
            .fill(super::BG1)
            .sized_wh(width, height)
    }
}
