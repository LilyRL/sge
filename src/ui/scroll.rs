use bevy_math::{FloatExt, vec2};

use crate::{
    api::delta_time,
    prelude::{draw_rounded_rect, pop_scissor, push_scissor},
};

use super::*;

pub struct Scroll {
    child: Child,
    state: State<ScrollState>,
    scroll_speed: f32,
    interpolation_speed: f32,
    scrollbar: Option<ScrollbarStyle>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ScrollbarStyle {
    pub width: f32,
    pub handle_color: Color,
    pub padding: f32,
    pub corner_radius: f32,
}

impl Default for ScrollbarStyle {
    fn default() -> Self {
        Self {
            width: 10.0,
            padding: 2.0,
            handle_color: Color::NEUTRAL_400,
            corner_radius: 999.0,
        }
    }
}

const DEFAULT_SCROLL_SPEED: f32 = 48.0;
const DEFAULT_INTERPOLATION_SPEED: f32 = 20.0;

#[derive(Default)]
struct ScrollState {
    desired_offset: f32,
    offset: f32,
    scrollbar_opacity: f32,
    desired_scrollbar_opacity: f32,
}

impl Scroll {
    pub fn new(id: usize, child: Child) -> UiRef {
        let state = State::from_id(id);

        Self {
            child,
            state,
            scroll_speed: DEFAULT_SCROLL_SPEED,
            interpolation_speed: DEFAULT_INTERPOLATION_SPEED,
            scrollbar: Some(ScrollbarStyle::default()),
        }
        .to_ref()
    }

    pub fn new_no_scrollbar(id: usize, child: Child) -> UiRef {
        let state = State::from_id(id);

        Self {
            child,
            state,
            scroll_speed: DEFAULT_SCROLL_SPEED,
            interpolation_speed: DEFAULT_INTERPOLATION_SPEED,
            scrollbar: None,
        }
        .to_ref()
    }

    pub fn new_custom_scrollbar(id: usize, child: Child, scrollbar: ScrollbarStyle) -> UiRef {
        let state = State::from_id(id);

        Self {
            child,
            state,
            scroll_speed: DEFAULT_SCROLL_SPEED,
            interpolation_speed: DEFAULT_INTERPOLATION_SPEED,
            scrollbar: Some(scrollbar),
        }
        .to_ref()
    }

    pub fn new_with_scroll_speed(speed: f32, id: usize, child: Child) -> UiRef {
        let state = State::from_id(id);

        Self {
            child,
            state,
            scroll_speed: speed,
            interpolation_speed: DEFAULT_INTERPOLATION_SPEED,
            scrollbar: Some(ScrollbarStyle::default()),
        }
        .to_ref()
    }

    pub fn new_full(
        speed: f32,
        interpolation_speed: f32,
        id: usize,
        child: Child,
        scrollbar: Option<ScrollbarStyle>,
    ) -> UiRef {
        let state = State::from_id(id);

        Self {
            child,
            state,
            scroll_speed: speed,
            interpolation_speed,
            scrollbar,
        }
        .to_ref()
    }

    fn draw_scrollbar(&self, area: Area, state: &mut ScrollState, child_dimensions: Vec2) {
        if let Some(style) = self.scrollbar {
            let stopped_moving = (state.desired_offset - state.offset).abs() < 0.001;
            if stopped_moving {
                state.desired_scrollbar_opacity = 0.0;
            } else {
                state.desired_scrollbar_opacity = 20.0;
            }

            state.scrollbar_opacity = state
                .scrollbar_opacity
                .lerp(state.desired_scrollbar_opacity, 5.0 * delta_time());

            let scrollbar_area = Area {
                top_left: area.top_left + vec2(area.width() - style.width, 0.0),
                size: vec2(style.width, area.height()),
            };

            let content_height = child_dimensions.y;
            let viewport_height = area.height();

            if content_height > viewport_height {
                let max_offset = content_height - viewport_height;
                let scroll_ratio = if max_offset > 0.0 {
                    state.offset / max_offset
                } else {
                    0.0
                };

                let handle_height = (viewport_height / content_height * viewport_height)
                    .min(viewport_height - style.padding * 2.0);

                let handle_travel = viewport_height - handle_height - style.padding * 2.0;
                let handle_pos = area.top_left.y + style.padding + scroll_ratio * handle_travel;

                draw_rounded_rect(
                    vec2(scrollbar_area.top_left.x + style.padding, handle_pos),
                    vec2(style.width - style.padding * 2.0, handle_height),
                    style
                        .handle_color
                        .with_alpha(state.scrollbar_opacity.clamp(0.0, 0.8)),
                    style.corner_radius,
                );
            }
        }
    }
}

impl UiNode for Scroll {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state.get_or_default();
        let input = ui.input();

        state.offset = state.offset.lerp(
            state.desired_offset,
            ui.delta_time * self.interpolation_speed,
        );

        let child_dimensions = self.child.node.preferred_dimensions();
        let max_offset = (child_dimensions.y - area.height()).max(0.0);

        if input.is_cursor_within_area(area) {
            let diff = -input.scroll_diff().1 * self.scroll_speed;
            state.desired_offset = (state.desired_offset + diff).clamp(0.0, max_offset);
        }

        push_scissor(area.to_rect());

        let mut draw_area = area;
        draw_area.top_left.y -= state.offset;

        let dimensions = self.child.node.draw(draw_area, ui);

        pop_scissor();

        if dimensions.y > area.height() {
            self.draw_scrollbar(area, state, dimensions);
        }

        area.size()
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }
}
