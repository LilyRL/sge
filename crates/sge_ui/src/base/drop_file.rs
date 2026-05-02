use std::path::PathBuf;

use sge_input::{dropped_file, hovered_file};

use super::*;

#[derive(Debug)]
pub struct DropFile {
    size: Vec2,
    base: fn() -> Child,
    hovered: fn(PathBuf) -> Child,
    dropped: fn(PathBuf) -> Child,
    state: State<DropFileState>,
}

#[derive(Debug, Default)]
struct DropFileState {
    path: Option<PathBuf>,
}

impl DropFile {
    pub fn new(
        size: Vec2,
        id: usize,
        base: fn() -> Child,
        hovered: fn(PathBuf) -> Child,
        dropped: fn(PathBuf) -> Child,
    ) -> UiRef {
        Self {
            size,
            state: State::from_id(id),
            base,
            hovered,
            dropped,
        }
        .to_ref()
    }
}

impl UiNode for DropFile {
    fn preferred_dimensions(&self) -> Vec2 {
        self.size
    }

    fn size(&self, _: Area) -> Vec2 {
        self.size
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let area = area.resize(self.size);
        if ui.is_hovered(area)
            && let Some(path) = dropped_file()
        {
            self.state.get_or_default().path = Some(path.clone());
            (self.dropped)(path).draw(area, ui)
        } else if ui.is_hovered(area)
            && let Some(path) = hovered_file()
        {
            (self.hovered)(path).draw(area, ui)
        } else {
            self.state.get_or_default().path = None;
            (self.base)().draw(area.resize(self.size), ui)
        }
    }
}

pub fn drop_file_path(id: usize) -> Option<PathBuf> {
    State::<DropFileState>::from_id(id)
        .get_or_default()
        .path
        .clone()
}
