use gilrs::{Axis, Button};
use std::collections::HashMap;

/// Tracks the state of a single gamepad across frames.
#[derive(Debug)]
pub(crate) struct GamepadState {
    // Button state
    buttons_pressed: Vec<Button>,
    buttons_released: Vec<Button>,
    buttons_repeated: Vec<Button>,
    buttons_held: Vec<Button>,
    button_values: HashMap<Button, f32>,

    // Axis state
    axis_values: HashMap<Axis, f32>,
    axis_changed: Vec<Axis>,

    // Connection state
    connected: bool,
    connected_this_frame: bool,
    disconnected_this_frame: bool,

    // Other event flags
    dropped: bool,
    ff_completed: bool,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            buttons_pressed: Vec::new(),
            buttons_released: Vec::new(),
            buttons_repeated: Vec::new(),
            buttons_held: Vec::new(),
            button_values: HashMap::new(),
            axis_values: HashMap::new(),
            axis_changed: Vec::new(),
            connected: true,
            connected_this_frame: true,
            disconnected_this_frame: false,
            dropped: false,
            ff_completed: false,
        }
    }

    /// Clear per-frame state, called at the start of each update.
    pub fn clear(&mut self) {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self.buttons_repeated.clear();
        self.axis_changed.clear();
        self.connected_this_frame = false;
        self.disconnected_this_frame = false;
        self.dropped = false;
        self.ff_completed = false;
    }

    // ==================== Button event handlers ====================

    pub fn button_pressed(&mut self, button: Button) {
        self.buttons_pressed.push(button);
        if !self.buttons_held.contains(&button) {
            self.buttons_held.push(button);
        }
    }

    pub fn button_released(&mut self, button: Button) {
        self.buttons_released.push(button);
        self.buttons_held.retain(|&b| b != button);
        self.button_values.remove(&button);
    }

    pub fn button_repeated(&mut self, button: Button) {
        self.buttons_repeated.push(button);
    }

    pub fn button_changed(&mut self, button: Button, value: f32) {
        self.button_values.insert(button, value);
    }

    // ==================== Axis event handlers ====================

    pub fn axis_changed(&mut self, axis: Axis, value: f32) {
        self.axis_values.insert(axis, value);
        if !self.axis_changed.contains(&axis) {
            self.axis_changed.push(axis);
        }
    }

    // ==================== Connection event handlers ====================

    pub fn mark_disconnected(&mut self) {
        self.connected = false;
        self.disconnected_this_frame = true;
        self.buttons_held.clear();
        self.button_values.clear();
        self.axis_values.clear();
    }

    pub fn mark_dropped(&mut self) {
        self.dropped = true;
    }

    pub fn mark_ff_completed(&mut self) {
        self.ff_completed = true;
    }

    // ==================== Query methods ====================

    pub fn was_button_pressed(&self, button: Button) -> bool {
        self.buttons_pressed.contains(&button)
    }

    pub fn was_button_released(&self, button: Button) -> bool {
        self.buttons_released.contains(&button)
    }

    pub fn is_button_held(&self, button: Button) -> bool {
        self.buttons_held.contains(&button)
    }

    pub fn was_button_repeated(&self, button: Button) -> bool {
        self.buttons_repeated.contains(&button)
    }

    pub fn button_value(&self, button: Button) -> Option<f32> {
        self.button_values.get(&button).copied()
    }

    pub fn axis_value(&self, axis: Axis) -> Option<f32> {
        self.axis_values.get(&axis).copied()
    }

    pub fn did_axis_change(&self, axis: Axis) -> bool {
        self.axis_changed.contains(&axis)
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn was_connected_this_frame(&self) -> bool {
        self.connected_this_frame
    }

    pub fn was_disconnected_this_frame(&self) -> bool {
        self.disconnected_this_frame
    }

    #[allow(dead_code)]
    pub fn was_dropped(&self) -> bool {
        self.dropped
    }

    #[allow(dead_code)]
    pub fn was_ff_completed(&self) -> bool {
        self.ff_completed
    }
}
