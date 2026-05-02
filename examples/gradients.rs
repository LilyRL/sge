use sge::prelude::*;

#[main("Gradients")]
async fn main() -> anyhow::Result<()> {
    let gen_point = || rand::<Vec2>() * 1000.0;
    let points: Vec<_> = (0..20).map(|_| gen_point()).collect();

    loop {
        draw_square_gradient_tl_br(Vec2::ZERO, 1000.0, Color::WHITE, Color::BLACK);

        draw_line_gradient(
            Vec2::ZERO,
            Vec2::splat(1000.0),
            100.0,
            Color::BLACK,
            Color::WHITE,
        );

        draw_square_outline(Vec2::ZERO - 50.0, 1100.0, 100.0, Color::BLACK);

        if (time() as usize / 2) % 2 == 1 {
            draw_gradient_path(&points, 20.0, Color::YELLOW_500, Color::BLUE_500);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
