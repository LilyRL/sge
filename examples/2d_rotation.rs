use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("2d rotation")?;

    loop {
        draw_square_rotation(Vec2::splat(100.0), 100.0, Color::RED_500, time());

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
