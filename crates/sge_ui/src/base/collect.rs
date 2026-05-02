use super::*;
use sge_api::shapes_2d::Orientation;

#[derive(Debug)]
pub struct Collect {
    orientation: Orientation,
    children: Vec<Child>,
    gap: f32,
}

impl Collect {
    pub fn new(orientation: Orientation, children: Vec<Child>) -> UiRef {
        Self {
            orientation,
            children,
            gap: 0.0,
        }
        .to_ref()
    }

    pub fn with_gap(orientation: Orientation, gap: f32, children: Vec<Child>) -> UiRef {
        Self {
            orientation,
            children,
            gap,
        }
        .to_ref()
    }

    pub fn vertical(children: Vec<Child>) -> UiRef {
        Self::new(Orientation::Vertical, children)
    }

    pub fn vertical_with_gap(gap: f32, children: Vec<Child>) -> UiRef {
        Self::with_gap(Orientation::Vertical, gap, children)
    }

    pub fn horizontal(children: Vec<Child>) -> UiRef {
        Self::new(Orientation::Horizontal, children)
    }

    pub fn horizontal_with_gap(gap: f32, children: Vec<Child>) -> UiRef {
        Self::with_gap(Orientation::Horizontal, gap, children)
    }

    fn layout(&self, area: Area) -> Vec<Area> {
        let mut max_cross = 0.0;
        let mut main_offset = 0.0;
        let mut cross_offset = 0.0;
        let mut areas = Vec::new();

        for child in &self.children {
            let child_area = Area {
                top_left: area.top_left + self.orientation.create_vec2(main_offset, cross_offset),
                size: area.size - self.orientation.create_vec2(main_offset, cross_offset),
            };
            let size = dbg!(child.node.size(child_area));
            let main_axis_size = self.orientation.main(size);
            let main_axis_area = self.orientation.main(area.size);
            let remaining_space = main_axis_area - main_offset;

            if remaining_space < main_axis_size {
                cross_offset += max_cross + self.gap;
                main_offset = self.gap;
            }

            let child_area = Area {
                top_left: area.top_left + self.orientation.create_vec2(main_offset, cross_offset),
                size: self
                    .orientation
                    .create_vec2(main_axis_size, self.orientation.cross(size)),
            };

            main_offset += main_axis_size + self.gap;
            max_cross = max_cross.max(self.orientation.cross(size));
            areas.push(child_area);
        }

        areas
    }
}

impl UiNode for Collect {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let areas = self.layout(area);
        for (child, area) in self.children.iter().zip(&areas) {
            child.node.draw(*area, ui);
        }
        areas
            .into_iter()
            .reduce(|a, b| a.merge(b))
            .unwrap_or(Area::ZERO)
            .size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        let mut main_axis_size = 0.0;
        let mut max_cross = 0.0f32;

        for child in &self.children {
            let preferred_dimensions = child.node.preferred_dimensions();
            main_axis_size += self.orientation.main(preferred_dimensions) + self.gap;
            max_cross = max_cross.max(self.orientation.cross(preferred_dimensions));
        }

        if !self.children.is_empty() {
            main_axis_size -= self.gap;
        }

        self.orientation.create_vec2(main_axis_size, max_cross)
    }

    fn size(&self, area: Area) -> Vec2 {
        self.layout(area)
            .into_iter()
            .reduce(|a, b| a.merge(b))
            .unwrap_or(Area::ZERO)
            .size
    }
}
