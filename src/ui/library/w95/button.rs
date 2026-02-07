use engine_color::Color;

use crate::ui::{
    Child, UiRef,
    base::{BoxFill, Fit, Padding, Text},
};

use super::{BLACK, Card, PRIMARY};

pub struct Button;

impl Button {
    pub fn new(id: usize, child: Child) -> UiRef {
        Fit::new(Card::border(
            2.0,
            BoxFill::new(
                PRIMARY,
                crate::ui::base::Button::new(id, Padding::xy(40.0, 10.0, child)),
            ),
        ))
    }

    pub fn text(id: usize, text: impl ToString) -> UiRef {
        Self::new(id, Text::new_with_color(text, BLACK))
    }
}
