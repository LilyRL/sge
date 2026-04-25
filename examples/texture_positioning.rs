// i dont even remember what this example was for

use sge::prelude::*;

#[main("Texture positioning")]
fn main() -> anyhow::Result<()> {
    let sprite = load_texture_from_bytes_sync(
        include_bytes!("../assets/textures/guy.jpg"),
        ImageFormat::Jpeg,
    )?;

    loop {
        draw_texture(sprite, Vec2::ZERO, 500.0);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
