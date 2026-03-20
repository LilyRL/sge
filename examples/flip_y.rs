// if you'd prefer to use positive y meaning up, as is more standard in math

use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Flip y")?;
    use_positive_y_up();

    let mut position = window_center();

    loop {
        if window_resized().is_some() {
            position = window_center();
        }

        draw_circle(position, 100.0, Color::PINK_400);

        let speed = delta_time() * window_height() / 2.0;

        if key_held(KeyCode::ArrowUp) {
            position.y += speed;
        }
        if key_held(KeyCode::ArrowDown) {
            position.y -= speed;
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
