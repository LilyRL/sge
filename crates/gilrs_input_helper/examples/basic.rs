use gilrs_input_helper::GilrsInputHelper;
use gilrs::{Axis, Button};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Gilrs Input Helper - Basic Example");
    println!("==================================");
    println!("Press buttons on your gamepad to see events.");
    println!("Press Ctrl+C to exit.\n");

    let mut input = GilrsInputHelper::new()
        .expect("Failed to initialize gamepad input");

    loop {
        // Update gamepad state
        input.update();

        // Check connection status
        let connected_count = input.connected_count();
        if connected_count > 0 {
            // Button events (any gamepad)
            if input.button_pressed(Button::South) {
                println!("South button (A/Cross) pressed!");
            }
            if input.button_pressed(Button::East) {
                println!("East button (B/Circle) pressed!");
            }
            if input.button_pressed(Button::West) {
                println!("West button (X/Square) pressed!");
            }
            if input.button_pressed(Button::North) {
                println!("North button (Y/Triangle) pressed!");
            }

            // D-pad
            if input.button_pressed(Button::DPadUp) {
                println!("D-Pad Up pressed!");
            }
            if input.button_pressed(Button::DPadDown) {
                println!("D-Pad Down pressed!");
            }
            if input.button_pressed(Button::DPadLeft) {
                println!("D-Pad Left pressed!");
            }
            if input.button_pressed(Button::DPadRight) {
                println!("D-Pad Right pressed!");
            }

            // Shoulder buttons
            if input.button_pressed(Button::LeftTrigger) {
                println!("Left shoulder button pressed!");
            }
            if input.button_pressed(Button::RightTrigger) {
                println!("Right shoulder button pressed!");
            }

            // Triggers (analog)
            let left_trigger = input.button_value(Button::LeftTrigger2);
            let right_trigger = input.button_value(Button::RightTrigger2);
            if left_trigger > 0.5 {
                println!("Left trigger: {:.2}", left_trigger);
            }
            if right_trigger > 0.5 {
                println!("Right trigger: {:.2}", right_trigger);
            }

            // Menu buttons
            if input.button_pressed(Button::Start) {
                println!("Start button pressed!");
            }
            if input.button_pressed(Button::Select) {
                println!("Select button pressed!");
            }

            // Analog sticks
            let left_x = input.axis_value(Axis::LeftStickX);
            let left_y = input.axis_value(Axis::LeftStickY);
            let right_x = input.axis_value(Axis::RightStickX);
            let right_y = input.axis_value(Axis::RightStickY);

            if left_x.abs() > 0.1 || left_y.abs() > 0.1 {
                println!("Left stick: ({:.2}, {:.2})", left_x, left_y);
            }
            if right_x.abs() > 0.1 || right_y.abs() > 0.1 {
                println!("Right stick: ({:.2}, {:.2})", right_x, right_y);
            }

            // Check if button is held
            if input.button_held(Button::South) {
                println!("  (South button is being held)");
            }

            // Active gamepad info
            if let Some(active_id) = input.active_gamepad() {
                let gamepad = input.gilrs().gamepad(active_id);
                if input.button_pressed(Button::Mode) {
                    println!("Mode/Guide button pressed on: {}", gamepad.name());
                }
            }
        } else {
            println!("No gamepads connected. Waiting...");
        }

        // Check for new connections
        for gamepad_id in input.connected_gamepads() {
            if input.was_connected(gamepad_id) {
                let gamepad = input.gilrs().gamepad(gamepad_id);
                println!("🎮 Gamepad connected: {} (ID: {:?})", gamepad.name(), gamepad_id);
            }
            if input.was_disconnected(gamepad_id) {
                println!("❌ Gamepad disconnected (ID: {:?})", gamepad_id);
            }
        }

        // Small sleep to avoid busy-waiting
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
