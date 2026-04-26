use sge_window::use_pointer_cursor_icon;

use super::*;

#[derive(Debug)]
pub struct Hyperlink {
    href: String,
    child: Child,
}

impl Hyperlink {
    pub fn new(href: impl ToString, child: Child) -> UiRef {
        Self {
            href: href.to_string(),
            child: child.into(),
        }
        .to_ref()
    }
}

impl UiNode for Hyperlink {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let size = self.child.draw(area, ui);

        let new_area = Area::new(area.top_left, size);

        if ui.is_hovered(new_area) {
            use_pointer_cursor_icon();
        }

        if ui.is_clicked(new_area) {
            if let Err(err) = open::that(&self.href) {
                log::error!("Failed to open hyperlink: {}", err);
            }
        }

        size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.size(area)
    }
}
