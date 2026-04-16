use sge_api::shapes_2d::draw_line;
use sge_vectors::vec2;

use super::*;

use crate::{NumberValue, UiNode};

use super::Data;

#[derive(Debug)]
pub struct LineChart<T: NumberValue> {
    line_color: Color,
    line_thickness: f32,
    data: Data<T>,
}

impl<T: NumberValue> LineChart<T> {
    pub fn new(data: &[T], line_color: Color) -> UiRef {
        Self {
            line_color,
            line_thickness: 2.0,
            data: Data::new(data),
        }
        .to_ref()
    }

    pub fn new_with_line_thickness(data: &[T], line_color: Color, line_thickness: f32) -> UiRef {
        Self {
            line_color,
            line_thickness,
            data: Data::new(data),
        }
        .to_ref()
    }
}

impl<T: NumberValue> UiNode for LineChart<T> {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::INFINITY
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        let data = self.data.as_ref();
        let n = data.len();
        if n < 2 {
            return Vec2::ZERO;
        }

        let Some(max_value) = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) else {
            return Vec2::ZERO;
        };
        let max_value = max_value.to_f32();
        if max_value == 0.0 {
            return Vec2::ZERO;
        }

        let height_of_one = area.height() / max_value;
        let width_of_one = area.width() / (n - 1) as f32;
        let mut last_point: Option<Vec2> = None;

        for i in 0..n {
            let value = data[i];
            let x = i as f32 * width_of_one;
            let mut y = value.to_f32() * height_of_one;
            if !sge_config::get_config().use_positive_y_up {
                y = area.height() - y;
            }
            let point = vec2(x, y);
            if let Some(lp) = last_point {
                draw_line(
                    area.top_left + lp,
                    area.top_left + point,
                    self.line_thickness,
                    self.line_color,
                );
            }
            last_point = Some(point);
        }

        area.size
    }
}
