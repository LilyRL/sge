use super::*;
use bevy_math::{usizevec2, vec2};

/// Requires sized container outside
#[derive(Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,
    gap: f32,
    children: Vec<UiRef>,
}

impl Grid {
    /// Requires sized container outside
    pub fn new(rows: usize, cols: usize, children: impl Into<Vec<UiRef>>) -> UiRef {
        Self {
            rows,
            cols,
            gap: 0.0,
            children: children.into(),
        }
        .to_ref()
    }

    pub fn with_gap(rows: usize, cols: usize, gap: f32, children: impl Into<Vec<UiRef>>) -> UiRef {
        Self {
            rows,
            cols,
            gap,
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for Grid {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::INFINITY
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let mut cursor = usizevec2(0, 0);

        let total_gap_x = self.gap * (self.cols - 1) as f32;
        let total_gap_y = self.gap * (self.rows - 1) as f32;

        let size_of_one = vec2(
            (area.size.x - total_gap_x) / self.cols as f32,
            (area.size.y - total_gap_y) / self.rows as f32,
        );

        for child in &self.children {
            let pos = vec2(
                cursor.x as f32 * (size_of_one.x + self.gap),
                cursor.y as f32 * (size_of_one.y + self.gap),
            );

            let child_area = Area::new(area.top_left + pos, size_of_one);
            child.node.draw(child_area, ui);

            cursor.x += 1;
            if cursor.x >= self.cols {
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
