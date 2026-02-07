use std::fmt::Debug;

use crate::{
    api::window_size,
    get_state,
    input::Input,
    prelude::{TextureRef, draw_rect},
    utils::EngineCreate,
};
use bevy_math::Vec2;
use engine_4_macros::gen_ref_type;
use engine_color::Color;
use glium::winit::event::MouseButton;

/// base building blocks
pub mod base;
/// more complex widgets made of compoenents
pub mod library;
pub mod prelude;

#[macro_export]
macro_rules! id {
    () => {
        $crate::prelude::const_random::const_random!(usize)
    };
}

pub struct UiState {
    frame: usize,
    delta_time: f32,
    time: f32,
}

impl UiState {
    pub fn input(&self) -> &'static Input {
        &get_state().input
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn is_hovered(&self, area: Area) -> bool {
        self.input().is_cursor_within_area(area)
    }

    pub fn is_active(&self, area: Area) -> bool {
        self.input().is_cursor_within_area(area) && self.input().mouse_held(MouseButton::Left)
    }
}

pub struct SomeNode {
    node: Box<dyn UiNode>,
}

gen_ref_type!(SomeNode, UiRef, ui_nodes);

impl Debug for UiRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(f)
    }
}

pub trait UiNode: Debug {
    fn preferred_dimensions(&self) -> Vec2;
    fn draw(&self, area: Area, ui: &UiState) -> Vec2;
}

pub trait ToGenericUiNode: UiNode + Sized {
    fn to_generic(self) -> SomeNode;
    fn to_ref(self) -> UiRef {
        self.to_generic().create()
    }
}

impl<T: UiNode + 'static> ToGenericUiNode for T {
    fn to_generic(self) -> SomeNode {
        SomeNode {
            node: Box::new(self),
        }
    }
}

pub(crate) type Child = UiRef;

impl Default for UiRef {
    fn default() -> Self {
        base::EMPTY
    }
}

/// run at start of frame
pub(crate) fn update_ui() {
    let state = get_state();
    state.storage.ui_nodes.clear();
    state.storage.button_clicked = None;
    base::Empty.to_ref(); // set default (id: 0) node to Empty
}

/// does not limit ui elements to the edge of the screen.
///
/// be very careful with this. make sure you only put sized nodes in here
pub fn draw_ui_unbounded(node: UiRef, position: Vec2) -> Vec2 {
    let area = Area {
        top_left: position,
        size: Vec2::INFINITY,
    };

    let state = get_state();
    let state = UiState {
        frame: state.frame_count,
        delta_time: state.delta_time,
        time: state.time,
    };

    node.node.draw(area, &state)
}

pub fn draw_ui(node: UiRef, position: Vec2) -> Vec2 {
    let area = Area {
        top_left: position,
        size: window_size() - position,
    };

    let state = get_state();
    let state = UiState {
        frame: state.frame_count,
        delta_time: state.delta_time,
        time: state.time,
    };

    node.node.draw(area, &state)
}

#[derive(Copy, Clone)]
pub struct Area {
    top_left: Vec2,
    size: Vec2,
}

impl Area {
    pub fn new(top_left: Vec2, size: Vec2) -> Self {
        Self { top_left, size }
    }

    pub fn to_rect(self) -> glium::Rect {
        let window_size = window_size();

        let bottom_y = window_size.y - (self.top_left.y + self.size.y);

        glium::Rect {
            left: self.top_left.x as u32,
            bottom: bottom_y as u32,
            width: self.size.x as u32,
            height: self.size.y as u32,
        }
    }

    pub fn top_left(&self) -> Vec2 {
        self.top_left
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.top_left + self.size
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.top_left.x, self.top_left.y + self.size.y)
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.top_left.x + self.size.x, self.top_left.y)
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn fill(&self, color: Color) {
        draw_rect(self.top_left, self.size, color);
    }

    pub fn draw_texture(&self, texture: TextureRef) {
        crate::prelude::draw_texture_scaled(texture, self.top_left, self.size);
    }
}

#[derive(Clone, Copy)]
pub struct State<T: Debug> {
    _ref: StateRef,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Debug + 'static> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRef(usize);

impl<T: Debug> State<T> {
    pub fn from_id(id: usize) -> Self {
        Self {
            _ref: StateRef(id),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get_or_create_mut<F>(&self, create: F) -> &'static mut T
    where
        F: FnOnce() -> T,
    {
        let storage = &mut get_state().storage.ui_states;
        let some_state = &mut storage.get_mut(&self._ref);

        if let Some(state) = some_state
            && state.state.downcast_ref::<T>().is_none()
        {
            let new_state = create();
            state.state = Box::new(new_state);
        } else if some_state.is_none() {
            storage.insert(
                self._ref,
                SomeState {
                    state: Box::new(create()),
                },
            );
        }

        storage
            .get_mut(&self._ref)
            .unwrap()
            .state
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn get(&self) -> Option<&'static T> {
        let storage = &get_state().storage.ui_states;
        let some_state = storage.get(&self._ref)?;

        some_state.state.downcast_ref::<T>()
    }
}

impl<T: Default + Debug> State<T> {
    pub fn get_or_default(&self) -> &'static mut T {
        self.get_or_create_mut(T::default)
    }
}

pub struct SomeState {
    state: Box<dyn std::any::Any>,
}
