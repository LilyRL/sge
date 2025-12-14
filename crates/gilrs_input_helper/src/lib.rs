use gilrs::{Axis, Button, Event, EventType, Gilrs};
use std::collections::BTreeMap;
use std::time::Duration;

mod gamepad_state;
use gamepad_state::GamepadState;

pub use gilrs::{Error, GamepadId};

/// The main struct providing a simple API for gamepad input.
///
/// Create with `GilrsInputHelper::new()`, then:
/// * Call `update()` at the start of each frame/update loop
/// * Use accessor methods to query gamepad state
/// * All state is cached and updated only when `update()` is called
///
/// # Example
///
/// ```no_run
/// use gilrs_input_helper::GilrsInputHelper;
/// use gilrs::Button;
///
/// let mut input = GilrsInputHelper::new().unwrap();
///
/// loop {
///     input.update();
///     
///     // Check if any gamepad has the South button pressed
///     if input.button_pressed(Button::South) {
///         println!("Jump!");
///     }
///     
///     // Check specific gamepad
///     if let Some(gamepad_id) = input.active_gamepad() {
///         if input.gamepad_button_pressed(gamepad_id, Button::South) {
///             println!("Player {:?} jumped!", gamepad_id);
///         }
///     }
///     
///     # break;
/// }
/// ```
#[derive(Debug)]
pub struct GilrsInputHelper {
    gilrs: Gilrs,
    gamepads: BTreeMap<usize, GamepadState>,
    active_gamepad: Option<GamepadId>,
}

impl GilrsInputHelper {
    /// Creates a new `GilrsInputHelper`.
    ///
    /// # Errors
    ///
    /// Returns an error if gilrs fails to initialize (e.g., platform not supported).
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            gilrs: Gilrs::new()?,
            gamepads: BTreeMap::new(),
            active_gamepad: None,
        })
    }

    /// Updates the internal state by processing all pending gamepad events.
    ///
    /// Call this once per frame/update loop before checking any input state.
    pub fn update(&mut self) {
        // Clear previous frame's state for all gamepads
        for state in self.gamepads.values_mut() {
            state.clear();
        }

        // Process all events
        while let Some(event) = self.gilrs.next_event() {
            self.process_event(event);
        }
    }

    /// Updates the internal state with blocking, waiting for events.
    ///
    /// Similar to `update()` but will block until an event is received or timeout expires.
    /// Useful for applications that only need to react to input (e.g., GUI apps).
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time to wait for an event. `None` means wait indefinitely.
    pub fn update_blocking(&mut self, timeout: Option<Duration>) {
        // Clear previous frame's state for all gamepads
        for state in self.gamepads.values_mut() {
            state.clear();
        }

        // Process first event (blocking)
        if let Some(event) = self.gilrs.next_event_blocking(timeout) {
            self.process_event(event);

            // Then process any remaining events without blocking
            while let Some(event) = self.gilrs.next_event() {
                self.process_event(event);
            }
        }
    }

    fn process_event(&mut self, event: Event) {
        let gamepad_id: usize = event.id.into();

        match event.event {
            EventType::Connected => {
                self.gamepads.insert(gamepad_id, GamepadState::new());
                if self.active_gamepad.is_none() {
                    self.active_gamepad = Some(event.id);
                }
            }
            EventType::Disconnected => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.mark_disconnected();
                }
                if self.active_gamepad == Some(event.id) {
                    self.active_gamepad = self.find_next_active_gamepad();
                }
            }
            EventType::ButtonPressed(button, _) => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.button_pressed(button);
                }
                if self.active_gamepad.is_none() {
                    self.active_gamepad = Some(event.id);
                }
            }
            EventType::ButtonReleased(button, _) => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.button_released(button);
                }
            }
            EventType::ButtonRepeated(button, _) => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.button_repeated(button);
                }
            }
            EventType::ButtonChanged(button, value, _) => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.button_changed(button, value);
                }
            }
            EventType::AxisChanged(axis, value, _) => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.axis_changed(axis, value);
                }
            }
            EventType::Dropped => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.mark_dropped();
                }
            }
            EventType::ForceFeedbackEffectCompleted => {
                if let Some(state) = self.gamepads.get_mut(&gamepad_id) {
                    state.mark_ff_completed();
                }
            }
            _ => (),
        }
    }

    fn find_next_active_gamepad(&self) -> Option<GamepadId> {
        self.gamepads
            .iter()
            .find(|(_, state)| state.is_connected())
            .map(|(id, _)| unsafe { std::mem::transmute(id) })
    }

    /// Returns the ID of the most recently used connected gamepad, or `None` if no gamepads are connected.
    ///
    /// The active gamepad is automatically updated when:
    /// - A gamepad connects (if no active gamepad exists)
    /// - A button is pressed on any gamepad (if no active gamepad exists)
    /// - The active gamepad disconnects (switches to another connected gamepad)
    pub fn active_gamepad(&self) -> Option<GamepadId> {
        self.active_gamepad
    }

    /// Sets the active gamepad to the specified ID.
    ///
    /// Returns `true` if the gamepad exists and is connected, `false` otherwise.
    pub fn set_active_gamepad(&mut self, id: GamepadId) -> bool {
        let id_usize: usize = id.into();
        if self
            .gamepads
            .get(&id_usize)
            .map_or(false, |s| s.is_connected())
        {
            self.active_gamepad = Some(id);
            true
        } else {
            false
        }
    }

    /// Returns an iterator over all connected gamepad IDs.
    pub fn connected_gamepads(&self) -> impl Iterator<Item = GamepadId> + '_ {
        self.gamepads
            .iter()
            .filter(|(_, state)| state.is_connected())
            .map(|(id, _)| unsafe { std::mem::transmute(id) })
    }

    /// Returns the number of connected gamepads.
    pub fn connected_count(&self) -> usize {
        self.gamepads
            .values()
            .filter(|state| state.is_connected())
            .count()
    }

    // ==================== Button queries (any gamepad) ====================

    /// Returns `true` if the button was pressed on any connected gamepad this frame.
    pub fn button_pressed(&self, button: Button) -> bool {
        self.gamepads
            .values()
            .any(|state| state.was_button_pressed(button))
    }

    /// Returns `true` if the button was released on any connected gamepad this frame.
    pub fn button_released(&self, button: Button) -> bool {
        self.gamepads
            .values()
            .any(|state| state.was_button_released(button))
    }

    /// Returns `true` if the button is currently held on any connected gamepad.
    pub fn button_held(&self, button: Button) -> bool {
        self.gamepads
            .values()
            .any(|state| state.is_button_held(button))
    }

    /// Returns `true` if the button repeated on any connected gamepad this frame.
    pub fn button_repeated(&self, button: Button) -> bool {
        self.gamepads
            .values()
            .any(|state| state.was_button_repeated(button))
    }

    /// Returns the value (0.0 to 1.0) of the button on any connected gamepad.
    ///
    /// Returns 0.0 if the button is not pressed on any gamepad.
    pub fn button_value(&self, button: Button) -> f32 {
        self.gamepads
            .values()
            .filter_map(|state| state.button_value(button))
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0)
    }

    // ==================== Axis queries (any gamepad) ====================

    /// Returns the current value (-1.0 to 1.0) of the axis on any connected gamepad.
    ///
    /// If multiple gamepads have the axis active, returns the value with the largest magnitude.
    pub fn axis_value(&self, axis: Axis) -> f32 {
        self.gamepads
            .values()
            .filter_map(|state| state.axis_value(axis))
            .max_by(|a, b| {
                a.abs()
                    .partial_cmp(&b.abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0)
    }

    /// Returns `true` if the axis changed on any connected gamepad this frame.
    pub fn axis_changed(&self, axis: Axis) -> bool {
        self.gamepads
            .values()
            .any(|state| state.did_axis_change(axis))
    }

    // ==================== Specific gamepad queries ====================

    /// Returns `true` if the button was pressed on the specified gamepad this frame.
    pub fn gamepad_button_pressed(&self, id: GamepadId, button: Button) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.was_button_pressed(button))
    }

    /// Returns `true` if the button was released on the specified gamepad this frame.
    pub fn gamepad_button_released(&self, id: GamepadId, button: Button) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.was_button_released(button))
    }

    /// Returns `true` if the button is currently held on the specified gamepad.
    pub fn gamepad_button_held(&self, id: GamepadId, button: Button) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.is_button_held(button))
    }

    /// Returns `true` if the button repeated on the specified gamepad this frame.
    pub fn gamepad_button_repeated(&self, id: GamepadId, button: Button) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.was_button_repeated(button))
    }

    /// Returns the value (0.0 to 1.0) of the button on the specified gamepad.
    pub fn gamepad_button_value(&self, id: GamepadId, button: Button) -> Option<f32> {
        self.get_gamepad_state(id)
            .and_then(|state| state.button_value(button))
    }

    /// Returns the current value (-1.0 to 1.0) of the axis on the specified gamepad.
    pub fn gamepad_axis_value(&self, id: GamepadId, axis: Axis) -> Option<f32> {
        self.get_gamepad_state(id)
            .and_then(|state| state.axis_value(axis))
    }

    /// Returns `true` if the axis changed on the specified gamepad this frame.
    pub fn gamepad_axis_changed(&self, id: GamepadId, axis: Axis) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.did_axis_change(axis))
    }

    /// Returns `true` if the specified gamepad is connected.
    pub fn is_connected(&self, id: GamepadId) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.is_connected())
    }

    /// Returns `true` if the specified gamepad was connected this frame.
    pub fn was_connected(&self, id: GamepadId) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.was_connected_this_frame())
    }

    /// Returns `true` if the specified gamepad was disconnected this frame.
    pub fn was_disconnected(&self, id: GamepadId) -> bool {
        self.get_gamepad_state(id)
            .map_or(false, |state| state.was_disconnected_this_frame())
    }

    fn get_gamepad_state(&self, id: GamepadId) -> Option<&GamepadState> {
        let id: usize = id.into();
        self.gamepads.get(&id)
    }

    /// Provides direct access to the underlying `Gilrs` instance.
    ///
    /// Use this if you need functionality not provided by `GilrsInputHelper`,
    /// such as force feedback, gamepad information, or custom event handling.
    pub fn gilrs(&self) -> &Gilrs {
        &self.gilrs
    }

    /// Provides mutable access to the underlying `Gilrs` instance.
    pub fn gilrs_mut(&mut self) -> &mut Gilrs {
        &mut self.gilrs
    }
}
