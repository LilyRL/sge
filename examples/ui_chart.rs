use sge::prelude::*;
use ui::*;

fn main() -> anyhow::Result<()> {
    init("Chart")?;

    let data = vec![1, 3, 5, 8, 2, 3, 1, 4, 5, 7, 7, 9, 0];

    loop {
        clear_screen(Color::hex(0xEEEEEC));

        let ui = LineChart::new(&data, Color::hex(0x999999))
            .padding(4.0)
            .fill(Color::hex(0xD3D7CF))
            .border(Color::hex(0xBABDB6), 4.0)
            .sized_wh(200.0, 200.0);

        draw_ui(ui, Vec2::splat(20.0));

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
