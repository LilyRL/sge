use bevy_math::{UVec2, Vec2, vec2};
use error_union::Union;
#[cfg(feature = "gamepad")]
pub use gilrs;
#[cfg(feature = "gamepad")]
use gilrs::Gilrs;
use glium::winit;
use global::global;
use log::info;
use sge_types::Area;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
pub use winit::event::MouseButton;
pub use winit::keyboard::{Key, KeyCode};
use winit_input_helper::WinitInputHelper;

#[cfg(feature = "gamepad")]
pub mod gamepad;
pub mod keys;

pub struct Input {
    helper: WinitInputHelper,
    action_map: HashMap<Action, Button>,
    #[cfg(feature = "gamepad")]
    pub gamepad: Gilrs,
    last_cursor_position: Vec2,
}

global!(Input, input);

#[cfg(feature = "gamepad")]
pub fn init() -> Result<(), GilrsError> {
    set_input(Input::new()?);
    info!("Initialized input");
    Ok(())
}

#[cfg(not(feature = "gamepad"))]
pub fn init() {
    set_input(Input::new());
    info!("Initialized input");
}

pub fn update() {
    if let Some(cursor) = cursor() {
        get_input().last_cursor_position = cursor;
    }

    #[cfg(feature = "gamepad")]
    {
        let input = get_input();
        while let Some(event) = input.gamepad.next_event() {
            input.gamepad.update(&event);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Union)]
pub enum Button {
    Mouse(MouseButton),
    Keyboard(KeyCode),
}

impl Button {
    /// Returns `true` if the button is [`Mouse`].
    ///
    /// [`Mouse`]: Button::Mouse
    #[must_use]
    pub fn is_mouse(&self) -> bool {
        matches!(self, Self::Mouse(..))
    }

    /// Returns `true` if the button is [`Keyboard`].
    ///
    /// [`Keyboard`]: Button::Keyboard
    #[must_use]
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Self::Keyboard(..))
    }

    pub fn as_mouse(&self) -> Option<&MouseButton> {
        if let Self::Mouse(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_keyboard(&self) -> Option<&KeyCode> {
        if let Self::Keyboard(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Action(u32);

impl Action {
    pub const fn new(n: u32) -> Self {
        Self(n)
    }
}

impl Input {
    #[cfg(feature = "gamepad")]
    pub fn new() -> Result<Self, GilrsError> {
        Ok(Self {
            helper: WinitInputHelper::new(),
            action_map: HashMap::new(),
            gamepad: Gilrs::new()?,
            last_cursor_position: Vec2::ZERO,
        })
    }

    #[cfg(not(feature = "gamepad"))]
    pub fn new() -> Self {
        Self {
            helper: WinitInputHelper::new(),
            action_map: HashMap::new(),
            last_cursor_position: Vec2::ZERO,
        }
    }

    pub fn is_cursor_within_area(&self, area: Area) -> bool {
        let cursor = self.cursor();
        if let Some(cursor) = cursor {
            cursor.0 >= area.top_left().x
                && cursor.0 <= area.bottom_right().x
                && cursor.1 >= area.top_left().y
                && cursor.1 <= area.bottom_right().y
        } else {
            false
        }
    }

    pub fn bind_key(&mut self, action: Action, key: KeyCode) {
        self.action_map.insert(action, key.into());
    }

    pub fn bind_mouse(&mut self, action: Action, mouse_button: MouseButton) {
        self.action_map.insert(action, mouse_button.into());
    }

    pub fn bind_button(&mut self, action: Action, button: Button) {
        self.action_map.insert(action, button);
    }

    pub fn bind(&mut self, action: Action, button: impl Into<Button>) {
        self.action_map.insert(action, button.into());
    }

    pub fn get_key(&self, action: Action) -> Option<&KeyCode> {
        self.action_map.get(&action).and_then(|n| n.as_keyboard())
    }

    pub fn get_mouse(&self, action: Action) -> Option<&MouseButton> {
        self.action_map.get(&action).and_then(|n| n.as_mouse())
    }

    pub fn get_button(&self, action: Action) -> Option<&Button> {
        self.action_map.get(&action)
    }

    pub fn action_pressed(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_pressed(*key),
                Button::Mouse(button) => self.mouse_pressed(*button),
            }
        } else {
            false
        }
    }

    pub fn action_pressed_os(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_pressed_os(*key),
                Button::Mouse(button) => self.mouse_pressed(*button),
            }
        } else {
            false
        }
    }

    pub fn action_released(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_released(*key),
                Button::Mouse(button) => self.mouse_released(*button),
            }
        } else {
            false
        }
    }

    pub fn action_held(&self, action: Action) -> bool {
        if let Some(button) = self.get_button(action) {
            match button {
                Button::Keyboard(key) => self.key_held(*key),
                Button::Mouse(button) => self.mouse_held(*button),
            }
        } else {
            false
        }
    }

    pub fn get_all_binds(&self) -> &HashMap<Action, Button> {
        &self.action_map
    }

    pub fn last_cursor_pos(&self) -> Vec2 {
        self.last_cursor_position
    }
}

impl Deref for Input {
    type Target = WinitInputHelper;

    fn deref(&self) -> &Self::Target {
        &self.helper
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.helper
    }
}

/// Returns true when the key with the specified keycode goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout, so for example the `W` key will be in the same physical key on both US and french keyboards.
///
/// This is suitable for game controls.
pub fn key_pressed(keycode: KeyCode) -> bool {
    get_input().key_pressed(keycode)
}

pub fn button_pressed(button: Button) -> bool {
    match button {
        Button::Keyboard(key) => key_pressed(key),
        Button::Mouse(button) => mouse_pressed(button),
    }
}

/// Returns true when the key with the specified keycode goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout, so for example the `W` key will be in the same physical key on both US and french keyboards.
///
/// Will repeat key presses while held down according to the OS's key repeat configuration
/// This is suitable for UI.
pub fn key_pressed_os(keycode: KeyCode) -> bool {
    get_input().key_pressed_os(keycode)
}

/// Returns true when the key with the specified KeyCode goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout.
pub fn key_released(keycode: KeyCode) -> bool {
    get_input().key_released(keycode)
}

pub fn button_released(button: Button) -> bool {
    match button {
        Button::Keyboard(key) => key_released(key),
        Button::Mouse(button) => mouse_released(button),
    }
}

/// Returns true when the key with the specified keycode remains "pressed".
/// Otherwise returns false.
///
/// Uses physical keys in the US layout.
pub fn key_held(keycode: KeyCode) -> bool {
    get_input().key_held(keycode)
}

pub fn button_held(button: Button) -> bool {
    match button {
        Button::Keyboard(key) => key_held(key),
        Button::Mouse(button) => mouse_held(button),
    }
}

/// Returns true while any shift key is held on the keyboard.
/// Otherwise returns false.
pub fn held_shift() -> bool {
    get_input().held_shift()
}

/// Returns true while any control key is held on the keyboard.
/// Otherwise returns false.
pub fn held_control() -> bool {
    get_input().held_control()
}

/// Returns true while any alt key is held on the keyboard.
/// Otherwise returns false.
pub fn held_alt() -> bool {
    get_input().held_alt()
}

/// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
/// Will never repeat keypresses while held.
pub fn key_pressed_logical(check_key: Key<&str>) -> bool {
    get_input().key_pressed_logical(check_key)
}

/// Returns true when the specified keyboard key goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
/// Will repeat key presses while held down according to the OS's key repeat configuration.
/// This is suitable for UI.
pub fn key_pressed_os_logical(check_key: Key<&str>) -> bool {
    get_input().key_pressed_os_logical(check_key)
}

/// Returns true when the specified keyboard key goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
pub fn key_released_logical(check_key: Key<&str>) -> bool {
    get_input().key_released_logical(check_key)
}

/// Returns true while the specified keyboard key remains "pressed".
/// Otherwise returns false.
///
/// Uses logical keypresses, so for example W is changed between a US and french keyboard.
pub fn key_held_logical(check_key: Key<&str>) -> bool {
    get_input().key_held_logical(check_key)
}

/// Returns true when the specified mouse button goes from "not pressed" to "pressed".
/// Otherwise returns false.
pub fn mouse_pressed(mouse_button: MouseButton) -> bool {
    get_input().mouse_pressed(mouse_button)
}

/// Returns true when the specified mouse button goes from "pressed" to "not pressed".
/// Otherwise returns false.
pub fn mouse_released(mouse_button: MouseButton) -> bool {
    get_input().mouse_released(mouse_button)
}

/// Returns true while the specified mouse button remains "pressed".
/// Otherwise returns false.
pub fn mouse_held(mouse_button: MouseButton) -> bool {
    get_input().mouse_held(mouse_button)
}

/// Returns the amount scrolled by the mouse during the last step.
/// Returns (horizontally, vertically).
///
/// Returns (0.0, 0.0) when the window is not focused.
pub fn scroll_diff() -> Vec2 {
    get_input().scroll_diff().into()
}

/// Returns the cursor coordinates in pixels, when window is focused AND
/// (cursor is on window OR any mouse button remains held while cursor moved off window).
/// Otherwise returns None.
pub fn cursor() -> Option<Vec2> {
    get_input().cursor().map(|c| vec2(c.0, c.1))
}

pub fn cursor_prev() -> Option<Vec2> {
    get_input().cursor_prev().map(|c| vec2(c.0, c.1))
}

/// Returns the change in cursor coordinates that occurred during the last step,
/// when window is focused AND (cursor is on window OR any mouse button remains held
/// while cursor moved off window). Otherwise returns (0.0, 0.0).
pub fn cursor_diff() -> Vec2 {
    get_input().cursor_diff().into()
}

/// Returns the change in mouse coordinates that occurred during the last step.
///
/// This is useful when implementing first person controls with a captured mouse.
pub fn mouse_diff() -> Vec2 {
    get_input().mouse_diff().into()
}

/// Returns the characters pressed during the last step.
/// The characters are in the order they were pressed.
pub fn input_text() -> &'static [Key] {
    get_input().text()
}

/// Returns the path to a file that has been drag-and-dropped onto the window.
pub fn dropped_file() -> Option<PathBuf> {
    get_input().dropped_file()
}

/// Returns the current window size if it was resized during the last step.
/// Otherwise returns None.
pub fn window_resized() -> Option<UVec2> {
    get_input()
        .window_resized()
        .map(|size| UVec2::new(size.width, size.height))
}

/// Returns the current resolution of the window.
///
/// Returns None when no WindowEvent::Resized have been received yet.
pub fn resolution() -> Option<(u32, u32)> {
    get_input().resolution()
}

/// Returns the current scale factor if it was changed during the last step.
/// Otherwise returns None.
pub fn scale_factor_changed() -> Option<f64> {
    get_input().scale_factor_changed()
}

/// Returns the current scale_factor of the window.
///
/// Returns None when no WindowEvent::ScaleFactorChanged have been received yet.
pub fn scale_factor() -> Option<f64> {
    get_input().scale_factor()
}

/// Returns true if the window has been destroyed. Otherwise returns false.
///
/// Once this method has returned true once, all following calls to this method will also return true.
pub fn destroyed() -> bool {
    get_input().destroyed()
}

/// Returns true if the OS has requested the application to close during this step.
/// Otherwise returns false.
pub fn close_requested() -> bool {
    get_input().close_requested()
}

/// Returns true when the action's bound button goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_pressed(action: Action) -> bool {
    get_input().action_pressed(action)
}

/// Returns true when the action's bound button goes from "not pressed" to "pressed".
/// Otherwise returns false.
///
/// For keyboard keys, will repeat key presses while held down according to the OS's key repeat configuration.
/// This is suitable for UI.
///
/// Returns false if the action is not bound to any button.
pub fn action_pressed_os(action: Action) -> bool {
    get_input().action_pressed_os(action)
}

/// Returns true when the action's bound button goes from "pressed" to "not pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_released(action: Action) -> bool {
    get_input().action_released(action)
}

/// Returns true while the action's bound button remains "pressed".
/// Otherwise returns false.
///
/// Returns false if the action is not bound to any button.
pub fn action_held(action: Action) -> bool {
    get_input().action_held(action)
}

/// Binds a keyboard key to an action.
///
/// When the key is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind_key(action: Action, key: KeyCode) {
    get_input().bind_key(action, key)
}

/// Binds a mouse button to an action.
///
/// When the mouse button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind_mouse(action: Action, mouse_button: MouseButton) {
    get_input().bind_mouse(action, mouse_button)
}

/// Binds a button (either keyboard or mouse) to an action.
///
/// When the button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind_button(action: Action, button: Button) {
    get_input().bind_button(action, button)
}

/// Binds a button (either keyboard or mouse) to an action.
///
/// When the button is pressed, `action_pressed()` and related functions will return true for this action.
pub fn bind(action: Action, button: impl Into<Button>) {
    get_input().bind(action, button)
}

/// Returns the keyboard key bound to the specified action, if any.
///
/// Returns None if the action is not bound or is bound to a mouse button instead.
pub fn get_key_binding(action: Action) -> Option<&'static KeyCode> {
    get_input().get_key(action)
}

/// Returns the mouse button bound to the specified action, if any.
///
/// Returns None if the action is not bound or is bound to a keyboard key instead.
pub fn get_mouse_binding(action: Action) -> Option<&'static MouseButton> {
    get_input().get_mouse(action)
}

/// Returns the button (keyboard or mouse) bound to the specified action, if any.
///
/// Returns None if the action is not bound to any button.
pub fn get_binding(action: Action) -> Option<&'static Button> {
    get_input().get_button(action)
}

/// Get a map of all the bindings that have been registered with the engine.
pub fn get_all_binds() -> &'static HashMap<Action, Button> {
    get_input().get_all_binds()
}

#[cfg(feature = "precise_cursor_movement")]
pub fn cursor_movements() -> Vec<Vec2> {
    get_input()
        .helper
        .cursor_movements()
        .iter()
        .map(|&v| v.into())
        .collect()
}

pub fn last_cursor_pos() -> Vec2 {
    get_input().last_cursor_pos()
}

pub fn should_quit() -> bool {
    get_input().close_requested()
}

#[cfg(feature = "gamepad")]
pub fn gamepad_input() -> &'static Gilrs {
    &get_input().gamepad
}

#[non_exhaustive]
#[derive(Debug)]
#[cfg(feature = "gamepad")]
pub enum GilrsError {
    NotImplemented,
    InvalidAxisToBtn,
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[cfg(feature = "gamepad")]
impl From<gilrs::Error> for GilrsError {
    fn from(value: gilrs::Error) -> Self {
        match value {
            gilrs::Error::InvalidAxisToBtn => Self::InvalidAxisToBtn,
            gilrs::Error::NotImplemented(_) => Self::NotImplemented,
            gilrs::Error::Other(e) => Self::Other(e),
            _ => unimplemented!(),
        }
    }
}

#[cfg(feature = "gamepad")]
impl std::error::Error for GilrsError {}

#[cfg(feature = "gamepad")]
impl std::fmt::Display for GilrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented => f.write_str("Gilrs does not support current platform."),
            Self::InvalidAxisToBtn => f.write_str(
                "Either `pressed ≤ released` or one of values is outside [0.0, 1.0] range.",
            ),
            Self::Other(e) => e.fmt(f),
        }
    }
}
