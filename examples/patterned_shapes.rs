use sge::*;
use sge_api::shapes_2d::Shape2DExt;

const PATTERNS: &[Pattern] = &[
    Pattern::Fill,
    Pattern::Checker,
    Pattern::ConcentricRings,
    Pattern::HorizontalLines,
    Pattern::VerticalLines,
    Pattern::NwseLines,
    Pattern::NeswLines,
    Pattern::Dots,
    Pattern::ConcentricRings,
    Pattern::Grid,
    Pattern::CrossHatch,
    Pattern::ConcentricRings,
    Pattern::SparseDots,
    Pattern::ConcentricRings,
    Pattern::Bricks,
    Pattern::HerringBone,
    Pattern::ConcentricSquares,
    Pattern::Triangles,
    Pattern::Waves,
    Pattern::Textured,
    Pattern::ConcentricRings,
    Pattern::Truchet,
    Pattern::RandomTiles,
    Pattern::ConcentricRings,
    Pattern::DiagonalWaves,
];

#[main("Patterned shapes")]
async fn main() {
    let mut controller = PanningCameraController::new();
    let mut i = 0;
    let n = PATTERNS.len();
    let w = (n as f32).sqrt().ceil() as usize;
    let colors = Palette::PALETTES.map(|p| p.v500);

    loop {
        controller.update();

        if key_pressed(KeyCode::KeyN) {
            i = (i + 1) % n;
        }

        clear_screen(Color::NEUTRAL_100);

        for i in 0..n {
            let x = (i % w) as f32 * 400.0 - (w as f32 * 200.0);
            let y = (i / w) as f32 * 400.0 - (w as f32 * 200.0);
            let color = colors[i % colors.len()];
            let square = Rect::new_square(Vec2::new(x, y), 300.0, color);
            square.draw_outline_world(20.0, color);
            draw_shape_with_pattern_world(square, Color::TRANSPARENT, PATTERNS[i], 10.0);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
