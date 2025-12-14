# gilrs_input_helper

A simple, [winit_input_helper](https://github.com/rukai/winit_input_helper)-style wrapper for [gilrs](https://gitlab.com/gilrs-project/gilrs) gamepad input.

This crate provides an easy-to-use API for handling gamepad input in Rust games and applications, inspired by the excellent `winit_input_helper` crate.

## Features

- 🎮 **Simple API**: Check button presses, releases, and holds with simple method calls
- 🔄 **Automatic state management**: Just call `update()` once per frame
- 👥 **Multi-gamepad support**: Query any gamepad or specific gamepads
- 🔌 **Connection tracking**: Easy detection of gamepad connections and disconnections
- 📊 **Analog input**: Full support for analog sticks and triggers
- 🚀 **Zero-copy**: Efficient state tracking without unnecessary allocations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gilrs_input_helper = "0.1"
```

## Quick Start

```rust
use gilrs_input_helper::GilrsInputHelper;
use gilrs::Button;

fn main() {
    let mut input = GilrsInputHelper::new().unwrap();

    loop {
        // Update input state (call once per frame)
        input.update();

        // Check if any button was pressed
        if input.button_pressed(Button::South) {
            println!("Jump!");
        }

        // Check if button is being held
        if input.button_held(Button::North) {
            println!("Charging...");
        }

        // Read analog stick values
        let stick_x = input.axis_value(gilrs::Axis::LeftStickX);
        let stick_y = input.axis_value(gilrs::Axis::LeftStickY);

        // Your game loop here...
        # break;
    }
}
```

## API Overview

### Initialization and Updates

```rust
let mut input = GilrsInputHelper::new()?;

// In your game loop:
input.update(); // Call this once per frame
```

For blocking applications (e.g., GUI apps that only react to input):

```rust
use std::time::Duration;

// Wait up to 100ms for input, then process
input.update_blocking(Some(Duration::from_millis(100)));

// Or wait indefinitely
input.update_blocking(None);
```

### Button Input

```rust
// Check if button was pressed this frame (any gamepad)
if input.button_pressed(Button::South) {
    println!("South button pressed!");
}

// Check if button was released this frame
if input.button_released(Button::South) {
    println!("South button released!");
}

// Check if button is currently held
if input.button_held(Button::South) {
    println!("South button is held!");
}

// Check if button is repeating (OS-level key repeat)
if input.button_repeated(Button::South) {
    println!("South button repeated!");
}

// Get analog button value (0.0 to 1.0)
let trigger = input.button_value(Button::LeftTrigger2);
if trigger > 0.5 {
    println!("Trigger pressed: {:.2}", trigger);
}
```

### Axis Input

```rust
use gilrs::Axis;

// Get analog stick values (-1.0 to 1.0)
let left_x = input.axis_value(Axis::LeftStickX);
let left_y = input.axis_value(Axis::LeftStickY);

// Check if axis changed this frame
if input.axis_changed(Axis::LeftStickX) {
    println!("Left stick X changed!");
}

// Use stick values for movement
if left_x.abs() > 0.1 || left_y.abs() > 0.1 {
    move_character(left_x, left_y);
}
```

### Multi-Gamepad Support

```rust
// Get number of connected gamepads
let count = input.connected_count();

// Iterate over all connected gamepads
for gamepad_id in input.connected_gamepads() {
    println!("Gamepad {:?} is connected", gamepad_id);
}

// Get active gamepad (most recently used)
if let Some(gamepad_id) = input.active_gamepad() {
    println!("Active gamepad: {:?}", gamepad_id);
}

// Check specific gamepad
if input.gamepad_button_pressed(gamepad_id, Button::South) {
    println!("Player {:?} pressed South!", gamepad_id);
}

// Get axis value from specific gamepad
if let Some(value) = input.gamepad_axis_value(gamepad_id, Axis::LeftStickX) {
    println!("Gamepad {:?} left stick X: {:.2}", gamepad_id, value);
}
```

### Connection Events

```rust
// Check if a gamepad connected this frame
if input.was_connected(gamepad_id) {
    println!("Gamepad connected!");
}

// Check if a gamepad disconnected this frame
if input.was_disconnected(gamepad_id) {
    println!("Gamepad disconnected!");
}

// Check if a gamepad is currently connected
if input.is_connected(gamepad_id) {
    println!("Gamepad is connected");
}
```

### Advanced: Direct Access to Gilrs

If you need functionality not provided by the helper (e.g., force feedback, gamepad info):

```rust
// Get gamepad information
let gamepad = input.gilrs().gamepad(gamepad_id);
println!("Gamepad name: {}", gamepad.name());
println!("Power info: {:?}", gamepad.power_info());
println!("Vendor ID: {:?}", gamepad.vendor_id());

// Access force feedback (if supported)
if gamepad.is_ff_supported() {
    // Set up force feedback effects...
}

// Mutable access if needed
let gilrs = input.gilrs_mut();
```

## Button Mapping

The library uses standard gamepad button names that map to common controllers:

| Button Name      | Xbox      | PlayStation | Switch    |
|------------------|-----------|-------------|-----------|
| `South`          | A         | Cross (✕)   | B         |
| `East`           | B         | Circle (○)  | A         |
| `West`           | X         | Square (□)  | Y         |
| `North`          | Y         | Triangle (△)| X         |
| `LeftTrigger`    | LB        | L1          | L         |
| `RightTrigger`   | RB        | R1          | R         |
| `LeftTrigger2`   | LT        | L2          | ZL        |
| `RightTrigger2`  | RT        | R2          | ZR        |
| `Select`         | Back/View | Share       | -         |
| `Start`          | Start/Menu| Options     | +         |
| `Mode`           | Xbox      | PS          | Home      |
| `LeftThumb`      | LS        | L3          | Left Stick|
| `RightThumb`     | RS        | R3          | Right Stick|
| `DPadUp/Down/Left/Right` | D-Pad | D-Pad   | D-Pad     |

## Comparison with Direct Gilrs Usage

**Without gilrs_input_helper:**

```rust
let mut gilrs = Gilrs::new().unwrap();

loop {
    while let Some(Event { id, event, .. }) = gilrs.next_event() {
        match event {
            EventType::ButtonPressed(button, _) => {
                // Handle button press
            }
            EventType::ButtonReleased(button, _) => {
                // Handle button release
            }
            EventType::AxisChanged(axis, value, _) => {
                // Handle axis change
            }
            _ => {}
        }
    }
    
    // Need to manually track state for "held" buttons
    // and combine state from multiple gamepads...
}
```

**With gilrs_input_helper:**

```rust
let mut input = GilrsInputHelper::new().unwrap();

loop {
    input.update();
    
    if input.button_pressed(Button::South) {
        // Handle button press
    }
    
    if input.button_held(Button::South) {
        // Handle held button
    }
    
    let stick_x = input.axis_value(Axis::LeftStickX);
    // Use stick value directly
}
```

## Examples

Run the examples with:

```bash
# Basic example - shows all button and axis input
cargo run --example basic

# Multi-gamepad example - tracks multiple players
cargo run --example multi_gamepad
```

## Platform Support

This crate supports all platforms supported by `gilrs`:

- ✅ Windows (XInput and Windows Gaming Input)
- ✅ Linux (via evdev)
- ✅ macOS
- ✅ FreeBSD/OpenBSD
- ✅ Web (Wasm)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

- Inspired by [winit_input_helper](https://github.com/rukai/winit_input_helper) by rukai
- Built on top of [gilrs](https://gitlab.com/gilrs-project/gilrs) by Arvamer

## Changelog

### 0.1.0

- Initial release
- Basic button and axis input tracking
- Multi-gamepad support
- Connection event tracking
- Active gamepad management
