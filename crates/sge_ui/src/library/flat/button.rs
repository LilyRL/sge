use crate::prelude::*;

pub struct Button;

impl Button {
    pub fn new(bg: Color, hover: Color, id: usize, child: Child) -> UiRef {
        Fit::new(HoverBoxFill::new(
            bg,
            hover,
            base::Button::new(id, Padding::xy(40.0, 10.0, child)),
        ))
    }

    pub fn primary(id: usize, child: Child) -> UiRef {
        Self::new(super::BG1, super::BG2, id, child)
    }

    pub fn text(bg: Color, hover: Color, id: usize, text: impl ToString) -> UiRef {
        Fit::new(ActiveFill::new(
            bg,
            hover,
            bg,
            0.0,
            base::Button::new(id, Padding::xy(40.0, 10.0, Text::nowrap(text))),
        ))
    }

    pub fn primary_text(id: usize, text: impl ToString) -> UiRef {
        Self::text(super::BG1, super::BG2, id, text)
    }
}
