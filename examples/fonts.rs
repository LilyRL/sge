use sge::prelude::*;

#[main("Fonts")]
async fn main() -> anyhow::Result<()> {
    // this font is included in the SANS constant, if you use the extra-fonts feauture
    let inter = load_font_sync(include_bytes!("../crates/sge_text/assets/inter.ttf"))?;

    loop {
        inter.draw_text("Hello world, from Inter", Vec2::splat(100.0), 100);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
