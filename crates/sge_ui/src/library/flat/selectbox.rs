use sge_color::Color;

use crate::{
    UiRef,
    base::{BoxFill, Center, Padding, Text, select_box_value},
};

pub struct SelectBox;

impl SelectBox {
    pub fn new(
        base_color: Color,
        selected_color: Color,
        id: usize,
        choices: impl IntoIterator<Item = UiRef>,
    ) -> UiRef {
        let n = select_box_value(id).unwrap_or(0);
        crate::base::SelectBox::new(
            id,
            choices
                .into_iter()
                .enumerate()
                .map(|(i, node)| {
                    BoxFill::new(if i == n { selected_color } else { base_color }, node)
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn new_text<T: ToString>(
        base_color: Color,
        selected_color: Color,
        id: usize,
        choices: impl IntoIterator<Item = T>,
    ) -> UiRef {
        Self::new(
            base_color,
            selected_color,
            id,
            choices.into_iter().map(|s| {
                Padding::tblr(
                    10.0,
                    15.0,
                    0.0,
                    0.0,
                    Center::horizontal(Text::new(s.to_string())),
                )
            }),
        )
    }
}
