use bevy_math::VectorSpace;
use engine_4::prelude::*;
use engine_color::schemes::ColorScheme;

const COLORS: ColorScheme = ColorScheme::GRUVBOX_DARK;

fn main() -> anyhow::Result<()> {
    init("Color schemes")?;

    let text = "Hello color schemes!";
    let mut text_params = TextDrawParams {
        color: COLORS.fg0,
        font_size: 50,
        ..Default::default()
    };

    loop {
        draw_rect(Vec2::ZERO, Vec2::splat(1000.0), COLORS.bg0);
        clear_screen(COLORS.bg0);

        text_params.position = draw_palette() + Vec2::splat(20.0);

        let dimensions = measure_text_ex(text, text_params);
        draw_rect(text_params.position, dimensions.size, COLORS.bg1);
        draw_text_ex(text, text_params);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

fn draw_palette() -> Vec2 {
    let palette = COLORS.palette();
    let mut cursor = Vec2::ZERO;
    let size = Vec2::splat(50.0);

    for color in palette {
        draw_rect(cursor, size, color);
        cursor.x += size.x;

        if cursor.x >= 500.0 {
            cursor.x = 0.0;
            cursor.y += size.y;
        }
    }

    cursor.x = 0.0;
    cursor.y += size.y;

    cursor
}
