use crate::prelude::*;

pub struct Card;

impl Card {
    pub fn fit(bg: Color, child: Child) -> UiRef {
        Fit::new(BoxFill::new(bg, Padding::all(20.0, child)))
    }

    pub fn sized(size: Vec2, bg: Color, child: Child) -> UiRef {
        SizedBox::new(size, BoxFill::new(bg, Padding::all(20.0, child)))
    }

    pub fn expand(bg: Color, child: Child) -> UiRef {
        BoxFill::new(bg, Padding::all(20.0, child))
    }

    pub fn bg0_expand(child: Child) -> UiRef {
        Self::expand(super::BG0, child)
    }

    pub fn bg0_fit(child: Child) -> UiRef {
        Self::fit(super::BG0, child)
    }

    pub fn bg0_sized(size: Vec2, child: Child) -> UiRef {
        Self::sized(size, super::BG0, child)
    }

    pub fn bg1_expand(child: Child) -> UiRef {
        Self::expand(super::BG1, child)
    }

    pub fn bg1_fit(child: Child) -> UiRef {
        Self::fit(super::BG1, child)
    }

    pub fn bg1_sized(size: Vec2, child: Child) -> UiRef {
        Self::sized(size, super::BG1, child)
    }

    pub fn bg2_expand(child: Child) -> UiRef {
        Self::expand(super::BG2, child)
    }

    pub fn bg2_fit(child: Child) -> UiRef {
        Self::fit(super::BG2, child)
    }

    pub fn bg2_sized(size: Vec2, child: Child) -> UiRef {
        Self::sized(size, super::BG2, child)
    }

    pub fn bg3_expand(child: Child) -> UiRef {
        Self::expand(super::BG3, child)
    }

    pub fn bg3_fit(child: Child) -> UiRef {
        Self::fit(super::BG3, child)
    }

    pub fn bg3_sized(size: Vec2, child: Child) -> UiRef {
        Self::sized(size, super::BG3, child)
    }

    pub fn bg4_expand(child: Child) -> UiRef {
        Self::expand(super::BG4, child)
    }

    pub fn bg4_fit(child: Child) -> UiRef {
        Self::fit(super::BG4, child)
    }

    pub fn bg4_sized(size: Vec2, child: Child) -> UiRef {
        Self::sized(size, super::BG4, child)
    }

    pub fn text(child: Child) -> UiRef {
        Padding::tblr(10.0, 15.0, 40.0, 40.0, child).fill(super::BG0)
    }
}
