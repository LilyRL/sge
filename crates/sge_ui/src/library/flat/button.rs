use crate::prelude::*;

pub struct Button;

impl Button {
    pub fn new(bg: Color, hover: Color, id: usize, child: Child) -> UiRef {
        Fit::new(ActiveFill::new(
            bg,
            hover,
            bg,
            0.0,
            base::Button::new(id, Padding::tblr(10.0, 15.0, 40.0, 40.0, child)),
        ))
    }

    pub fn primary(id: usize, child: Child) -> UiRef {
        Self::new(super::BG1, super::BG2, id, child)
    }

    pub fn text(bg: Color, hover: Color, id: usize, text: impl ToString) -> UiRef {
        Self::new(bg, hover, id, Text::nowrap(text))
    }

    pub fn primary_text(id: usize, text: impl ToString) -> UiRef {
        Self::text(super::BG1, super::BG2, id, text)
    }

    pub fn danger(id: usize, child: Child) -> UiRef {
        Self::new(super::BG1, super::ERROR, id, child)
    }
}
