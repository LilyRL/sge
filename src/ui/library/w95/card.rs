use super::super::*;
use super::*;

pub struct Card;

impl Card {
    pub fn new(padding: f32, child: Child) -> UiRef {
        Self::card(padding, 5.0, child)
    }

    pub fn thin(padding: f32, child: Child) -> UiRef {
        Self::card(padding, 2.0, child)
    }

    pub fn thick(padding: f32, child: Child) -> UiRef {
        Self::card(padding, 10.0, child)
    }

    fn card(padding: f32, thickness: f32, child: Child) -> UiRef {
        Self::border(
            thickness,
            BoxFill::new(PRIMARY, Padding::all(padding, child)),
        )
    }

    pub(crate) fn border(thickness: f32, child: Child) -> UiRef {
        Border::tblr(
            BorderStyle::new(thickness, WHITE),
            BorderStyle::new(thickness, BLACK),
            BorderStyle::new(thickness, WHITE),
            BorderStyle::new(thickness, BLACK),
            child,
        )
    }
}
