use std::{num::ParseIntError, str::FromStr};

use sge_color::Color;
use sge_error_union::ErrorUnion;
use sge_vectors::Vec2;
use strum::Display;

use crate::{
    State, UiRef,
    base::{AspectRatio, BoxFill, Col, EMPTY, Row, data_input_state, slider_state},
    id,
    library::flat,
};

pub struct ColorSelector;

#[derive(Debug)]
struct ColorSelectorState {
    color: HexColor,
}

#[derive(Debug, Clone, Copy)]
struct HexColor {
    r: u8,
    g: u8,
    b: u8,
}

impl HexColor {
    fn to_color(&self) -> Color {
        Color::from_rgba_u8(self.r, self.g, self.b, 255)
    }

    fn from_color(color: Color) -> Self {
        let (r, g, b) = color.to_u8();
        Self { r, g, b }
    }
}

impl ToString for HexColor {
    fn to_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Display)]
pub enum InvalidHex {
    WrongLength,
}

#[derive(ErrorUnion, Debug)]
pub enum HexParseError {
    InvalidHex(InvalidHex),
    ParseIntError(ParseIntError),
}

impl FromStr for HexColor {
    type Err = HexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // #fff & #ffffff
        let s = s.trim_start_matches('#');
        if s.len() == 3 {
            let r = u8::from_str_radix(&s[0..1].repeat(2), 16)?;
            let g = u8::from_str_radix(&s[1..2].repeat(2), 16)?;
            let b = u8::from_str_radix(&s[2..3].repeat(2), 16)?;
            Ok(Self { r, g, b })
        } else if s.len() == 6 {
            let r = u8::from_str_radix(&s[0..2], 16)?;
            let g = u8::from_str_radix(&s[2..4], 16)?;
            let b = u8::from_str_radix(&s[4..6], 16)?;
            Ok(Self { r, g, b })
        } else {
            Err(InvalidHex::WrongLength.into())
        }
    }
}

impl ColorSelector {
    pub fn new(id: usize, default_color: Color) -> UiRef {
        let state =
            State::<ColorSelectorState>::from_id(id).get_or_create_mut(|| ColorSelectorState {
                color: HexColor::from_color(default_color),
            });

        let data_input_id = id ^ id!();
        let data = data_input_state::<HexColor>(data_input_id);

        if data.changed
            && data.err.is_none()
            && let Some(value) = &data.value
        {
            state.color = *value;
        }

        if data.value.is_none() && data.text.is_empty() {
            data.text = state.color.to_string();
        }

        for i in 1..=3 {
            let slider_id = id + i;
            let slider_state = slider_state::<u8>(slider_id);
            if slider_state.captured {
                data_input_state::<HexColor>(data_input_id).text = state.color.to_string();
            }
        }

        super::Card::sized(
            Vec2::new(500.0, 200.0),
            super::BG0.blend_halfway(super::BG1),
            Row::with_gap(
                20.0,
                [
                    AspectRatio::new(1.0, BoxFill::new(state.color.to_color(), EMPTY)),
                    Col::with_gap(
                        10.0,
                        [
                            flat::Slider::alternate(&mut state.color.r, 0, 255, id + 1),
                            flat::Slider::alternate(&mut state.color.g, 0, 255, id + 2),
                            flat::Slider::alternate(&mut state.color.b, 0, 255, id + 3),
                            flat::DataInput::<HexColor>::with_prompt(
                                super::BG1,
                                "Hex",
                                data_input_id,
                            ),
                        ],
                    ),
                ],
            ),
        )
    }
}
