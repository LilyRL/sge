use bevy_math::Vec2;
use engine_color::Color;
use log::warn;

use super::*;
use crate::prelude::{
    FontRef, MONO, SANS, SANS_DISPLAY, TextDrawParams, draw_text_ex, measure_text_ex,
};

pub struct Text {
    text: String,
    font: FontRef,
    font_size: usize,
    color: Color,
    /// scale the font size by the DPI scaling of your monitor
    do_dpi_scaling: bool,
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

    pub fn h1(text: impl ToString) -> UiRef {
        Padding::tblr(
            20.0,
            10.0,
            0.0,
            0.0,
            Self {
                text: text.to_string(),
                font_size: 40,
                font: SANS_DISPLAY,
                ..Default::default()
            }
            .to_ref(),
        )
    }

    pub fn h2(text: impl ToString) -> UiRef {
        Padding::tblr(
            20.0,
            10.0,
            0.0,
            0.0,
            Self {
                text: text.to_string(),
                font_size: 32,
                font: SANS_DISPLAY,
                ..Default::default()
            }
            .to_ref(),
        )
    }

    pub fn h3(text: impl ToString) -> UiRef {
        Padding::tblr(
            20.0,
            10.0,
            0.0,
            0.0,
            Self {
                text: text.to_string(),
                font_size: 26,
                font: SANS_DISPLAY,
                ..Default::default()
            }
            .to_ref(),
        )
    }

    pub fn small(text: impl ToString) -> UiRef {
        Self {
            text: text.to_string(),
            font_size: 12,
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
    ) -> UiRef {
        Self {
            text: text.to_string(),
            font,
            font_size,
            color,
            do_dpi_scaling,
        }
        .to_ref()
    }
}

impl UiNode for Text {
    fn preferred_dimensions(&self) -> bevy_math::Vec2 {
        let params: TextDrawParams = self.into();
        measure_text_ex(&self.text, params).size
    }

    fn draw(&self, area: super::Area, _: &UiState) -> Vec2 {
        let immutable: &Self = &self;
        let mut params: TextDrawParams = immutable.into();
        let size = area.size();
        let dim = self.preferred_dimensions();

        let is_overflow = size.x < dim.x || size.y < dim.y;
        if is_overflow {
            warn!("Text overflows container: '{}'.", self.text);
        }

        params.position = area.top_left;

        draw_text_ex(&self.text, params).size
    }
}
