use super::*;

#[derive(Debug)]
pub struct SelectBox {
    choices: Vec<Child>,
    state: State<SelectBoxState>,
}

#[derive(Debug, Default)]
pub struct SelectBoxState {
    selected: Option<usize>,
}

impl SelectBox {
    pub fn new(id: usize, choices: impl Into<Vec<Child>>) -> UiRef {
        Self {
            choices: choices.into(),
            state: State::from_id(id),
        }
        .to_ref()
    }
}

impl UiNode for SelectBox {
    fn preferred_dimensions(&self) -> Vec2 {
        col_calc_preferred_dimensions(&self.choices, 0.0)
    }

    fn size(&self, area: Area) -> Vec2 {
        col_calc_size(&self.choices, 0.0, area)
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let mut y_offset = 0.0;
        for (i, child) in self.choices.iter().enumerate() {
            let child_height = child.node.preferred_dimensions().y;
            let new_area = Area {
                top_left: Vec2::new(area.top_left.x, area.top_left.y + y_offset),
                size: Vec2::new(area.size.x, child_height),
            };
            let dimensions = child.node.draw(new_area, ui);
            let child_area = Area {
                top_left: new_area.top_left,
                size: Vec2::new(area.size.x, dimensions.y),
            };
            if ui.is_clicked(child_area) {
                self.state.get_or_default().selected = Some(i);
            }
            y_offset += dimensions.y;
        }
        Vec2::new(area.size.x, y_offset)
    }
}

pub fn select_box_value(id: usize) -> Option<usize> {
    let state = State::<SelectBoxState>::from_id(id).get_or_default();
    state.selected
}

pub fn select_box_state(id: usize) -> &'static mut SelectBoxState {
    State::<SelectBoxState>::from_id(id).get_or_default()
}
