use super::*;

#[derive(Debug)]
pub struct Expandable {
    title: Child,
    contents: Child,
    default_open: bool,
    state: State<ExpandableState>,
}

impl Expandable {
    pub fn new(title: Child, contents: Child, id: usize) -> UiRef {
        Self {
            title,
            contents,
            default_open: false,
            state: State::from_id(id),
        }
        .to_ref()
    }

    pub fn new_with_default_open(title: Child, contents: Child, id: usize) -> UiRef {
        Self {
            title,
            contents,
            default_open: true,
            state: State::from_id(id),
        }
        .to_ref()
    }

    pub fn new_full(title: Child, contents: Child, default_open: bool, id: usize) -> UiRef {
        Self {
            title,
            contents,
            default_open,
            state: State::from_id(id),
        }
        .to_ref()
    }

    fn state(&self) -> &'static mut ExpandableState {
        self.state.get_or_create_mut(|| ExpandableState {
            open: self.default_open,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct ExpandableState {
    pub(crate) open: bool,
}

impl UiNode for Expandable {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state();

        let title_size = self.title.draw(area, ui);
        let title_area = Area::new(area.top_left, title_size);

        if ui.is_clicked(title_area) {
            state.open = !state.open;
        }

        if state.open {
            let contents_origin = area.top_left + Vec2::new(0.0, title_size.y);
            let contents_area =
                Area::new(contents_origin, area.size - Vec2::new(0.0, title_size.y));
            self.contents.draw(contents_area, ui);
            title_size + Vec2::new(0.0, self.contents.size(contents_area).y)
        } else {
            title_size
        }
    }

    fn preferred_dimensions(&self) -> Vec2 {
        let state = self.state();

        if state.open {
            Col::new([self.title, self.contents]).preferred_dimensions()
        } else {
            self.title.preferred_dimensions()
        }
    }

    fn size(&self, area: Area) -> Vec2 {
        let state = self.state();

        if state.open {
            Col::new([self.title, self.contents]).size(area)
        } else {
            self.title.size(area)
        }
    }
}
