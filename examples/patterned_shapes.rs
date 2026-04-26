use sge::*;
use sge_api::shapes_2d::Shape2DExt;

const PATTERNS: &[Pattern] = &[
    Pattern::Fill,
    Pattern::Checker,
    Pattern::HorizontalLines,
    Pattern::VerticalLines,
    Pattern::NwseLines,
    Pattern::NeswLines,
    Pattern::Dots,
    Pattern::Grid,
    Pattern::CrossHatch,
    Pattern::SparseDots,
    Pattern::Bricks,
    Pattern::HerringBone,
    Pattern::Triangles,
    Pattern::ConcentricSquares,
    Pattern::Waves,
    Pattern::ConcentricRings,
    Pattern::Textured,
    Pattern::Truchet,
    Pattern::RandomTiles,
    Pattern::DiagonalWaves,
    Pattern::Topology,
    Pattern::Zebra,
    Pattern::FishScales,
    Pattern::Maze,
    Pattern::Moire,
    Pattern::LeopardSpots,
    Pattern::Rings,
];

#[main("Patterned shapes")]
async fn main() {
    let mut controller = PanningCameraController::new();
    let mut i = 0;
    let n = PATTERNS.len();
    let w = (n as f32).sqrt().ceil() as usize;
    let colors = Palette::PALETTES.map(|p| p.v500);
    let mut scale = 50.0;

    loop {
        controller.update();

        draw_fps_black();

        if key_pressed(KeyCode::KeyN) {
            i = (i + 1) % n;
        }

        if key_pressed(KeyCode::ArrowUp) {
            scale *= 2.0;
        }

        if key_pressed(KeyCode::ArrowDown) {
            scale *= 0.5;
        }

        clear_screen(Color::NEUTRAL_100);

        for i in 0..n {
            let x = (i % w) as f32 * 400.0 - (w as f32 * 200.0);
            let y = (i / w) as f32 * 400.0 - (w as f32 * 200.0);
            let color = colors[i % colors.len()];
            let square = Rect::new_square(Vec2::new(x, y), 300.0, color);
            square.draw_outline_world(20.0, color);
            draw_shape_with_pattern_world(square, Color::TRANSPARENT, PATTERNS[i], scale);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
