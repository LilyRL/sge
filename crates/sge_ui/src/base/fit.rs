use super::*;

#[derive(Debug)]
pub struct Fit {
    child: Child,
    horizontal: bool,
    vertical: bool,
}

impl Fit {
    pub fn new(child: Child) -> UiRef {
        Self {
            child,
            horizontal: true,
            vertical: true,
        }
        .to_ref()
    }

    pub fn horizontal(child: Child) -> UiRef {
        Self {
            child,
            horizontal: true,
            vertical: false,
        }
        .to_ref()
    }

    pub fn vertical(child: Child) -> UiRef {
        Self {
            child,
            horizontal: false,
            vertical: true,
        }
        .to_ref()
    }
}

impl UiRef {
    pub fn fit(self) -> Self {
        Fit::new(self)
    }

    pub fn fit_horizontal(self) -> Self {
        Fit::horizontal(self)
    }

    pub fn fit_vertical(self) -> Self {
        Fit::vertical(self)
    }
}

impl UiNode for Fit {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        let preferred = self.preferred_dimensions();

        if self.horizontal {
            area.size.x = area.size.x.min(preferred.x);
        }

        if self.vertical {
            area.size.y = area.size.y.min(preferred.y);
        }

        self.child.node.draw(area, state)
    }
}

#[derive(Debug)]
pub struct FitSize {
    child: Child,
    horizontal: bool,
    vertical: bool,
}

impl FitSize {
    pub fn new(child: Child) -> UiRef {
        Self {
            child,
            horizontal: true,
            vertical: true,
        }
        .to_ref()
    }

    pub fn horizontal(child: Child) -> UiRef {
        Self {
            child,
            horizontal: true,
            vertical: false,
        }
        .to_ref()
    }

    pub fn vertical(child: Child) -> UiRef {
        Self {
            child,
            horizontal: false,
            vertical: true,
        }
        .to_ref()
    }
}

impl UiRef {
    pub fn fit_size(self) -> Self {
        FitSize::new(self)
    }

    pub fn fit_size_horizontal(self) -> Self {
        FitSize::horizontal(self)
    }

    pub fn fit_size_vertical(self) -> Self {
        FitSize::vertical(self)
    }
}

impl UiNode for FitSize {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        let preferred = self.child.size(area);

        Vec2 {
            x: if self.horizontal {
                area.size.x.min(preferred.x)
            } else {
                area.size.x
            },
            y: if self.vertical {
                area.size.y.min(preferred.y)
            } else {
                area.size.y
            },
        }
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        let preferred = self.child.size(area);

        if self.horizontal {
            area.size.x = area.size.x.min(preferred.x);
        }

        if self.vertical {
            area.size.y = area.size.y.min(preferred.y);
        }

        self.child.node.draw(area, state)
    }
}
