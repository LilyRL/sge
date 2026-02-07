use crate::prelude::*;
use crate::ui::prelude::*;

pub struct Card;

impl Card {
    pub fn fit(bg: Color, child: Child) -> UiRef {
        Fit::new(BoxFill::new(bg, Padding::all(20.0, child)))
    }

    pub fn sized(size: Vec2, bg: Color, child: Child) -> UiRef {
        SizedBox::new(size, BoxFill::new(bg, Padding::all(20.0, child)))
    }
}
