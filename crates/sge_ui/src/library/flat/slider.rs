use crate::{
    NumberValue, UiRef,
    base::{BoxFill, CircleFill, EMPTY, HoverBoxFill, RoundedFill, Text},
};

use super::{BG1, BG3, BG4, FG3};

pub struct Slider;

impl Slider {
    pub fn new<T: NumberValue>(value: &mut T, min: T, max: T, id: usize) -> UiRef {
        let bar = BoxFill::new(BG1, EMPTY)
            .min_height(10.0)
            .padding_vertical(10.0);
        let handle = HoverBoxFill::new(BG3, BG4, EMPTY).sized_wh(20.0, 30.0);
        crate::base::Slider::new(value, min, max, handle, bar, id)
    }

    pub fn alternate<T: NumberValue + ToString>(value: &mut T, min: T, max: T, id: usize) -> UiRef {
        let string: String = value.to_string().chars().take(4).collect();
        let bar = BoxFill::new(BG1, EMPTY)
            .min_height(10.0)
            .padding_vertical(10.0);
        let handle = HoverBoxFill::new(
            BG3,
            BG4,
            Text::mono_colored(string, FG3)
                .padding_top(4.0)
                .padding_left(5.0),
        )
        .sized_wh(50.0, 30.0);
        crate::base::Slider::new(value, min, max, handle, bar, id)
    }

    pub fn rounded<T: NumberValue>(value: &mut T, min: T, max: T, id: usize) -> UiRef {
        let handle = CircleFill::new(BG4).sized_wh(30.0, 30.0);
        let bar = RoundedFill::new(BG1, 5.0, EMPTY)
            .min_height(10.0)
            .padding_vertical(10.0);
        crate::base::Slider::new(value, min, max, handle, bar, id)
    }
}
