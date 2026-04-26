use crate::{UiRef, base};

pub struct Tooltip;

impl Tooltip {
    pub fn new(pos: base::TooltipPosition, element: UiRef, tooltip_contents: UiRef) -> UiRef {
        let overlay = tooltip_contents
            .padding(10.0)
            .fill(super::BG0)
            .border(super::BG3, 2.0);

        base::Tooltip::new(element, overlay, 10.0, pos)
    }

    pub fn text(pos: base::TooltipPosition, element: UiRef, text: UiRef) -> UiRef {
        let overlay = super::Card::text(text).border(super::BG3, 2.0);

        base::Tooltip::new(element, overlay, 10.0, pos)
    }
}
