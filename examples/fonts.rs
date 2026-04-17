use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Title for the window")?;

    // this font is included in the SANS constant, if you use the extra-fonts feauture
    let inter = load_font(include_bytes!("../crates/sge_text/assets/inter.ttf"))?;

    loop {
        inter.draw_text("Hello world, from Inter", Vec2::splat(100.0), 100);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
