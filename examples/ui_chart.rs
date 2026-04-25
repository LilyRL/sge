use sge::prelude::*;
use ui::*;

#[main("Chart")]
fn main() -> anyhow::Result<()> {
    let mut data = vec![1, 3, 5, 8, 2, 3, 1, 4, 5, 7, 7, 9, 0];

    loop {
        clear_screen(Color::hex(0xEEEEEC));

        if key_pressed(KeyCode::Space) {
            data.push(rand_range(0..10));
        }

        let ui = LineChart::new(&data, Color::hex(0x999999))
            .padding(4.0)
            .fill(Color::hex(0xD3D7CF))
            .border(Color::hex(0xBABDB6), 4.0)
            .sized_wh((data.len() / 20 + 1) as f32 * 200.0, 200.0);

        draw_ui(ui, Vec2::splat(20.0));

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
