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
        let mut max_width: f32 = 0.0;

        for (i, child) in self.choices.iter().enumerate() {
            let preferred_dimensions = child.node.preferred_dimensions();

            let new_area = {
                let mut a = area;
                a.top_left.y += y_offset;
                a.size = preferred_dimensions;
                a.size = a.size.min(area.size);
                a
            };

            let dimensions = child.node.draw(new_area, ui);

            let child_area = Area {
                top_left: new_area.top_left,
                size: dimensions,
            };

            if ui.is_clicked(child_area) {
                let state = self.state.get_or_default();
                state.selected = Some(i);
            }

            y_offset += dimensions.y;
            max_width = max_width.max(dimensions.x);
        }

        Vec2::new(max_width, y_offset)
    }
}

pub fn select_box_value(id: usize) -> Option<usize> {
    let state = State::<SelectBoxState>::from_id(id).get_or_default();
    state.selected
}

pub fn select_box_state(id: usize) -> &'static mut SelectBoxState {
    State::<SelectBoxState>::from_id(id).get_or_default()
}
