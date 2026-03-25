use bevy_math::Vec2;
use log::warn;
use sge_color::Color;

use super::*;
use sge_text::{
    FontRef, MONO, SANS, SANS_BOLD, SANS_BOLD_ITALIC, SANS_DISPLAY, SANS_ITALIC, TextDrawParams,
    draw_text_custom, measure_text_ex, measure_wrapped_text,
    wrapped_text::draw_wrapped_text_in_area,
};

#[derive(Debug)]
pub struct Text {
    text: String,
    font: FontRef,
    font_size: usize,
    color: Color,
    /// scale the font size by the DPI scaling of your monitor
    do_dpi_scaling: bool,
    line_spacing: f32,
    wrap: bool,
}

impl Default for Text {
    fn default() -> Self {
        let params = TextDrawParams::default();
        Self {
            text: String::new(),
            font_size: params.font_size,
            color: params.color,
            do_dpi_scaling: params.do_dpi_scaling,
            font: SANS,
            line_spacing: 1.0,
            wrap: true,
        }
    }
}

impl From<Text> for TextDrawParams {
    fn from(value: Text) -> Self {
        TextDrawParams {
            font: Some(value.font),
            font_size: value.font_size,
            color: value.color,
            position: Vec2::ZERO,
            do_dpi_scaling: value.do_dpi_scaling,
        }
    }
}

impl From<&Text> for TextDrawParams {
    fn from(value: &Text) -> Self {
        TextDrawParams {
            font: Some(value.font),
            font_size: value.font_size,
            color: value.color,
            position: Vec2::ZERO,
            do_dpi_scaling: value.do_dpi_scaling,
        }
    }
}

impl Text {
    pub fn new(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
        .to_ref()
    }

    pub fn no_wrap(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            wrap: false,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn no_wrap_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            color,
            wrap: false,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn body(text: impl ToString) -> UiRef {
        Self::new(text)
    }

    pub fn mono(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn mono_sized(text: impl ToString, font_size: usize) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            font_size,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn mono_nowrap(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            wrap: false,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn mono_colored(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font: MONO,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn title(text: impl ToString) -> UiRef {
        Padding::tblr(
            2.0,
            10.0,
            0.0,
            0.0,
            Center::horizontal(
                Self {
                    text: text.to_string(),
                    font_size: 48,
                    font: SANS_DISPLAY,
                    ..Default::default()
                }
                .to_ref(),
            ),
        )
    }

    pub fn title_nowrap(text: impl ToString) -> UiRef {
        Padding::tblr(
            2.0,
            10.0,
            0.0,
            0.0,
            Center::horizontal(
                Self {
                    text: text.to_string(),
                    font_size: 48,
                    font: SANS_DISPLAY,
                    wrap: false,
                    ..Default::default()
                }
                .to_ref(),
            ),
        )
    }

    pub fn h1_no_padding(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 40,
            font: SANS_DISPLAY,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn h1(text: impl ToString) -> UiRef {
        Padding::tblr(20.0, 10.0, 0.0, 0.0, Self::h1_no_padding(text))
    }

    pub fn h2_no_padding(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 32,
            font: SANS_DISPLAY,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn h2(text: impl ToString) -> UiRef {
        Padding::tblr(20.0, 10.0, 0.0, 0.0, Self::h2_no_padding(text))
    }

    pub fn h3_no_padding(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 26,
            font: SANS_DISPLAY,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn h3(text: impl ToString) -> UiRef {
        Padding::tblr(20.0, 10.0, 0.0, 0.0, Self::h3_no_padding(text))
    }

    pub fn small(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 12,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn italic(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_ITALIC,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn bold(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_BOLD,
            font_size: 18,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn bold_italic(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font: SANS_BOLD_ITALIC,
            font_size: 18,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_color(text: impl ToString, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_size_color(text: impl ToString, font_size: usize, color: Color) -> UiRef {
        Self {
            text: text.to_string(),
            font_size,
            color,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_with_size(text: impl ToString, font_size: usize) -> UiRef {
        Self {
            text: text.to_string(),
            font_size,
            ..Default::default()
        }
        .to_ref()
    }

    pub fn new_full(
        text: impl ToString,
        font: FontRef,
        font_size: usize,
        color: Color,
        do_dpi_scaling: bool,
        line_spacing: f32,
        wrap: bool,
    ) -> UiRef {
        Self {
            text: text.to_string(),
            font,
            font_size,
            color,
            do_dpi_scaling,
            line_spacing,
            wrap,
        }
        .to_ref()
    }

    pub fn from_params(text: impl ToString, params: TextDrawParams, wrap: bool) -> UiRef {
        Self {
            text: text.to_string(),
            font: params.font.unwrap_or(SANS),
            font_size: params.font_size,
            color: params.color,
            do_dpi_scaling: params.do_dpi_scaling,
            line_spacing: 1.0,
            wrap,
        }
        .to_ref()
    }
}

impl UiNode for Text {
    fn preferred_dimensions(&self) -> bevy_math::Vec2 {
        let params: TextDrawParams = self.into();
        measure_text_ex(&self.text, params).size
    }

    fn size(&self, area: Area) -> Vec2 {
        if self.wrap {
            measure_wrapped_text(
                &self.text,
                area.width(),
                Some(self.font),
                self.font_size,
                self.do_dpi_scaling,
                self.line_spacing,
            )
        } else {
            self.preferred_dimensions()
        }
    }

    fn draw(&self, area: super::Area, _: &UiState) -> Vec2 {
        let size = area.size();
        let dim = self.preferred_dimensions();

        let is_overflow = size.x < dim.x || size.y < dim.y;
        if is_overflow {
            warn!("Text overflows container: '{}'.", self.text);
        }

        if self.wrap {
            draw_wrapped_text_in_area(
                &self.text,
                area,
                Some(self.font),
                self.font_size,
                self.color,
                self.do_dpi_scaling,
                self.line_spacing,
            )
        } else {
            let mut params: TextDrawParams = self.into();
            params.position = area.top_left;
            draw_text_custom(&self.text, params).size
        }
    }
}
