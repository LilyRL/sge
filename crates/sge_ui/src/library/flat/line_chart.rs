use crate::{NumberValue, UiRef, base::Data};

pub struct LineChart;

impl LineChart {
    pub fn new<T: NumberValue>(data: impl Into<Data<T>>, width: f32, height: f32) -> UiRef {
        crate::base::LineChart::new(data, super::FG3)
            .padding(4.0)
            .fill(super::BG1)
            .sized_wh(width, height)
    }

    pub fn with_y<T: NumberValue>(
        data: impl Into<Data<T>>,
        width: f32,
        height: f32,
        include_y: T,
    ) -> UiRef {
        crate::base::LineChart::custom(data, super::FG3, 2.0, include_y)
            .padding(4.0)
            .fill(super::BG1)
            .sized_wh(width, height)
    }
}
