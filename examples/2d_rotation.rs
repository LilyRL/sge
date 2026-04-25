use sge::prelude::*;

#[main("2D rotation")]
async fn main() -> anyhow::Result<()> {
    loop {
        draw_square_rotation(Vec2::splat(100.0), 100.0, Color::RED_500, time());

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
