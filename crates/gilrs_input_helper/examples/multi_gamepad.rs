use gilrs_input_helper::GilrsInputHelper;
use gilrs::Button;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Gilrs Input Helper - Multi-Gamepad Example");
    println!("==========================================");
    println!("Connect multiple gamepads to see per-player input.");
    println!("Press the South button (A/Cross) on any gamepad.");
    println!("Press Ctrl+C to exit.\n");

    let mut input = GilrsInputHelper::new()
        .expect("Failed to initialize gamepad input");

    let mut player_scores: Vec<(usize, u32)> = Vec::new();

    loop {
        input.update();

        // Display connected gamepads
        let connected_count = input.connected_count();
        if connected_count == 0 {
            println!("No gamepads connected. Waiting...");
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        // Check each connected gamepad individually
        for gamepad_id in input.connected_gamepads() {
            let gamepad = input.gilrs().gamepad(gamepad_id);
            let player_num: usize = gamepad_id.into();

            // Handle new connections
            if input.was_connected(gamepad_id) {
                println!(
                    "\n🎮 Player {} joined: {} (Power: {:?})",
                    player_num,
                    gamepad.name(),
                    gamepad.power_info()
                );
                player_scores.push((player_num, 0));
            }

            // Handle disconnections
            if input.was_disconnected(gamepad_id) {
                println!("\n❌ Player {} disconnected", player_num);
                player_scores.retain(|(id, _)| *id != player_num);
                continue;
            }

            // Check for button press on this specific gamepad
            if input.gamepad_button_pressed(gamepad_id, Button::South) {
                // Find or create player score
                if let Some((_, score)) = player_scores.iter_mut().find(|(id, _)| *id == player_num)
                {
                    *score += 1;
                    println!("Player {} scored! Total: {}", player_num, score);
                }
            }

            // Check other buttons for this gamepad
            if input.gamepad_button_pressed(gamepad_id, Button::North) {
                println!("Player {} pressed North button", player_num);
            }

            if input.gamepad_button_pressed(gamepad_id, Button::Start) {
                println!("\n=== Current Scores ===");
                for (player_id, score) in &player_scores {
                    println!("Player {}: {} points", player_id, score);
                }
                println!("=====================\n");
            }

            // Show stick movement for this gamepad
            if let Some(x) = input.gamepad_axis_value(gamepad_id, gilrs::Axis::LeftStickX) {
                if x.abs() > 0.5 {
                    let direction = if x > 0.0 { "right" } else { "left" };
                    println!("Player {} moved stick {}", player_num, direction);
                }
            }
        }

        // Show active player info
        if let Some(active_id) = input.active_gamepad() {
            let player_num: usize = active_id.into();
            if input.button_pressed(Button::Mode) {
                println!("Player {} pressed the Guide/Home button", player_num);
            }
        }

        // Small sleep to avoid busy-waiting
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
