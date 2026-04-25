use sge::prelude::*;
use sge_input::gilrs::{Gamepad, GamepadId};

const PRIMARY: Color = Color::WHITE;
const SECONDARY: Color = Color::WHITE;

#[main("Gamepad")]
async fn main() -> anyhow::Result<()> {
    let mut gamepad = None;

    loop {
        if let Some(pad) = gamepad {
            visualize_gamepad(pad);
        } else {
            gamepad = choose_gamepad();
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn choose_gamepad() -> Option<GamepadId> {
    let input = gamepad::input();
    let gamepads: Vec<_> = input.gamepads().collect();

    use ui::prelude::*;
    let ui = Col::with_gap(
        10.0,
        gamepads
            .iter()
            .enumerate()
            .map(|(i, (_, pad))| flat::Button::primary_text(i, pad.name()))
            .collect::<Vec<_>>(),
    );

    draw_ui(ui, Vec2::splat(10.0));

    Some(
        gamepads
            .get(*all_elements_interacted_this_frame().first()?)?
            .0,
    )
}

fn visualize_gamepad(pad: GamepadId) {
    let input = gamepad::input();
    let pad = input.gamepad(pad);

    draw_stick(pad.left_stick(), vec2(150.0, 150.0));
    draw_stick(pad.right_stick(), vec2(450.0, 450.0));

    draw_dpad(pad, vec2(150.0, 450.0));
    draw_buttons(pad, vec2(450.0, 150.0));
}

fn draw_stick(val: Vec2, center: Vec2) {
    const SIZE: f32 = 100.0;
    let val = val.invert_y(); // because we draw with positive y-down

    draw_circle_outline(center, SIZE, SECONDARY, 5.0);

    let offset = val * SIZE;
    draw_circle(center + offset, SIZE / 2.0, PRIMARY);

    draw_text(
        format!("{}, {}", val.x, val.y),
        vec2(center.x - SIZE, center.y + SIZE + 20.0),
    );
}

// this is extremely ugly im afraid
fn draw_dpad(pad: Gamepad, center: Vec2) {
    const SIZE: f32 = 100.0;
    const THICKNESS: f32 = 65.0;
    const OUTLINE: f32 = 5.0;

    let half = THICKNESS / 2.0;
    let half_outline = OUTLINE / 2.0;
    let arm = SIZE - half;

    let buttons = [
        (
            gamepad::Button::DPadUp,
            vec2(center.x - half, center.y - SIZE),
            vec2(THICKNESS, arm + half_outline),
        ),
        (
            gamepad::Button::DPadDown,
            vec2(center.x - half, center.y + half - half_outline),
            vec2(THICKNESS, arm),
        ),
        (
            gamepad::Button::DPadLeft,
            vec2(center.x - SIZE, center.y - half),
            vec2(arm + half_outline, THICKNESS),
        ),
        (
            gamepad::Button::DPadRight,
            vec2(center.x + half - half_outline, center.y - half),
            vec2(arm, THICKNESS),
        ),
    ];

    let points = [
        vec2(center.x - half, center.y - SIZE),
        vec2(center.x + half, center.y - SIZE),
        vec2(center.x + half, center.y - half),
        vec2(center.x + SIZE, center.y - half),
        vec2(center.x + SIZE, center.y + half),
        vec2(center.x + half, center.y + half),
        vec2(center.x + half, center.y + SIZE),
        vec2(center.x - half, center.y + SIZE),
        vec2(center.x - half, center.y + half),
        vec2(center.x - SIZE, center.y + half),
        vec2(center.x - SIZE, center.y - half),
        vec2(center.x - half, center.y - half),
        vec2(center.x - half, center.y - SIZE),
    ];

    draw_connected_path(&points, OUTLINE, SECONDARY);

    for (button, pos, size) in buttons {
        if pad.is_pressed(button) {
            draw_rect(pos, size, PRIMARY);
        }
    }
}

fn draw_buttons(pad: Gamepad, center: Vec2) {
    const RADIUS: f32 = 30.0;
    const DIST: f32 = 70.0;
    const OUTLINE: f32 = 5.0;

    let buttons = [
        (gamepad::Button::North, vec2(center.x, center.y - DIST)),
        (gamepad::Button::South, vec2(center.x, center.y + DIST)),
        (gamepad::Button::West, vec2(center.x - DIST, center.y)),
        (gamepad::Button::East, vec2(center.x + DIST, center.y)),
    ];

    for (button, pos) in buttons {
        if pad.is_pressed(button) {
            draw_circle(pos, RADIUS + OUTLINE, PRIMARY);
        } else {
            draw_circle_outline(pos, RADIUS, SECONDARY, OUTLINE);
        }
    }
}
