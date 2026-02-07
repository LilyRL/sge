use bevy_math::{usizevec2, vec2};

use super::*;

/// Requires sized container outside
#[derive(Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,
    children: Vec<Child>,
}

impl Grid {
    /// Requires sized container outside
    fn new(rows: usize, cols: usize, children: impl Into<Vec<Child>>) -> UiRef {
        Self {
            rows,
            cols,
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for Grid {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let mut cursor = usizevec2(0, 0);
        let size_of_one = vec2(
            area.size.x / self.cols as f32,
            area.size.y / self.rows as f32,
        );

        for child in &self.children {
            let pos = cursor.as_vec2() * size_of_one;
            let area = Area::new(pos, size_of_one);

            child.node.draw(area, ui);

            cursor.x += 1;
            if cursor.x >= self.rows {
                cursor.x = 0;
                cursor.y += 1;

                if cursor.y >= self.rows {
                    break;
                }
            }
        }

        area.size
    }
}
