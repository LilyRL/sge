use crate::prelude::*;

use super::{BG0, BG3, BG4, PRIMARY_TEXT_COLOR};

pub struct FloatingWindow;

impl FloatingWindow {
    pub fn new(title: impl ToString, id: usize, contents: Child) -> UiRef {
        crate::base::FloatingWindow::builder()
            .title(Text::no_wrap_with_color(title, PRIMARY_TEXT_COLOR))
            .contents(contents)
            .id(id)
            .bg(BG0)
            .padding(10.0)
            .border(BorderStyle::new(2.0, BG3))
            .button_color(BG4)
            .active_button_color(BG4.lighten(0.2))
            .default_size(Vec2::new(400.0, 600.0))
            .closable(true)
            .build()
    }
}
