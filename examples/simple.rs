use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Simple example")?;

    loop {
        clear_screen(Color::NEUTRAL_900);

        draw_circle(vec2(10.0, 20.0), 30.0, Color::YELLOW_400);
        draw_square(vec2(100.0, 40.0), 20.0, Color::RED_300);

        draw_text(
            format!("{} x {}", window_width(), window_height()),
            vec2(40.0, 80.0),
        );

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
