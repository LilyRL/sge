use std::cmp::Ordering;

use super::*;

#[derive(Default)]
pub struct Row {
    pub gap: f32,
    pub children: Vec<Child>,
}

impl Row {
    pub fn new(children: impl Into<Vec<Child>>) -> UiRef {
        Self {
            children: children.into(),
            gap: 0.0,
        }
        .to_ref()
    }

    pub fn with_gap(gap: f32, children: impl Into<Vec<Child>>) -> UiRef {
        Self {
            gap,
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for Row {
    fn preferred_dimensions(&self) -> Vec2 {
        let dimensions = self
            .children
            .iter()
            .map(|child| child.node.preferred_dimensions());

        let x = dimensions.clone().map(|d| d.x).sum::<f32>()
            + self.gap * (self.children.len() - 1) as f32;

        let y = dimensions
            .map(|d| d.y)
            .max_by(|a, b| {
                if a > b {
                    Ordering::Greater
                } else if a < b {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap_or(0.0);

        Vec2::new(x, y)
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        let mut x_offset = 0.0;
        let mut max_height: f32 = 0.0;

        for child in self.children.iter() {
            let new_area = {
                let mut a = area;
                a.top_left.x += x_offset;
                a
            };

            let dimensions = child.node.draw(new_area, state);

            x_offset += dimensions.x + self.gap;
            max_height = max_height.max(dimensions.y);
        }

        x_offset -= self.gap;

        Vec2::new(x_offset, max_height)
    }
}
