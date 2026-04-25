use std::cmp::Ordering;

use super::*;

#[derive(Debug)]
pub enum FlexBox {
    Fixed(Child),
    Flex(Child),
}

impl FlexBox {
    fn child(&self) -> &Child {
        match self {
            Self::Fixed(c) | Self::Flex(c) => c,
        }
    }
}

#[derive(Debug)]
pub struct FlexRow {
    children: Vec<FlexBox>,
    gap: f32,
}

impl FlexRow {
    pub fn new(children: impl Into<Vec<FlexBox>>) -> UiRef {
        Self {
            children: children.into(),
            gap: 0.0,
        }
        .to_ref()
    }

    pub fn with_gap(gap: f32, children: impl Into<Vec<FlexBox>>) -> UiRef {
        Self {
            gap,
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for FlexRow {
    fn preferred_dimensions(&self) -> Vec2 {
        let dimensions = self
            .children
            .iter()
            .map(|child| child.child().node.preferred_dimensions());

        let x = dimensions.clone().map(|d| d.x).sum::<f32>()
            + self.gap * (self.children.len() - 1) as f32;

        let y = dimensions
            .map(|d| d.y)
            .filter(|y| y.is_finite())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0);

        Vec2::new(x, y)
    }

    fn size(&self, area: Area) -> Vec2 {
        Vec2::new(area.size.x, area.size.y)
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let total_flex = self
            .children
            .iter()
            .filter(|child| matches!(child, FlexBox::Flex(_)))
            .count();

        let total_fixed_width: f32 = self
            .children
            .iter()
            .filter_map(|child| {
                if let FlexBox::Fixed(c) = child {
                    Some(c.node.preferred_dimensions().x.min(area.width()))
                } else {
                    None
                }
            })
            .sum();

        let remaining_width =
            area.width() - total_fixed_width - self.gap * (self.children.len() - 1) as f32;
        let flex_unit_width = if total_flex > 0 {
            remaining_width / total_flex as f32
        } else {
            0.0
        };

        let mut x_offset = 0.0;
        let mut max_height: f32 = 0.0;

        for child in self.children.iter() {
            let child_width = match child {
                FlexBox::Fixed(c) => c.node.preferred_dimensions().x.min(area.width()),
                FlexBox::Flex(_) => flex_unit_width,
            };

            let new_area = {
                let mut a = area;
                a.top_left.x += x_offset;
                a.size.x = child_width;
                a
            };

            let child_dimensions = child.child().node.draw(new_area, ui);
            max_height = max_height.max(child_dimensions.y);
            x_offset += child_width + self.gap;
        }

        Vec2::new(area.width(), max_height.min(area.height()))
    }
}

#[derive(Debug)]
pub struct FlexCol {
    children: Vec<FlexBox>,
    gap: f32,
}

impl FlexCol {
    pub fn new(children: impl Into<Vec<FlexBox>>) -> UiRef {
        Self {
            children: children.into(),
            gap: 0.0,
        }
        .to_ref()
    }

    pub fn with_gap(gap: f32, children: impl Into<Vec<FlexBox>>) -> UiRef {
        Self {
            gap,
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for FlexCol {
    fn preferred_dimensions(&self) -> Vec2 {
        let dimensions = self
            .children
            .iter()
            .map(|child| child.child().node.preferred_dimensions());

        let x = dimensions
            .clone()
            .map(|d| d.x)
            .filter(|x| x.is_finite())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0);

        let y = dimensions.map(|d| d.y).sum::<f32>() + self.gap * (self.children.len() - 1) as f32;

        Vec2::new(x, y)
    }

    fn size(&self, area: Area) -> Vec2 {
        Vec2::new(area.size.x, area.size.y)
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let total_flex = self
            .children
            .iter()
            .filter(|child| matches!(child, FlexBox::Flex(_)))
            .count();

        let total_fixed_height: f32 = self
            .children
            .iter()
            .filter_map(|child| {
                if let FlexBox::Fixed(c) = child {
                    Some(c.node.preferred_dimensions().y.min(area.height()))
                } else {
                    None
                }
            })
            .sum();

        let remaining_height =
            area.height() - total_fixed_height - self.gap * (self.children.len() - 1) as f32;
        let flex_unit_height = if total_flex > 0 {
            remaining_height / total_flex as f32
        } else {
            0.0
        };

        let mut y_offset = 0.0;
        let mut max_width: f32 = 0.0;

        for child in self.children.iter() {
            let child_height = match child {
                FlexBox::Fixed(c) => c.node.preferred_dimensions().y.min(area.height()),
                FlexBox::Flex(_) => flex_unit_height,
            };

            let new_area = {
                let mut a = area;
                a.top_left.y += y_offset;
                a.size.x = area.size.x;
                a.size.y = child_height;
                a
            };

            let child_dimensions = child.child().node.draw(new_area, ui);
            max_width = max_width.max(child_dimensions.x);
            y_offset += child_height + self.gap;
        }

        Vec2::new(max_width.min(area.width()), area.height())
    }
}
