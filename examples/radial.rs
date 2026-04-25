use sge::prelude::*;

#[main("Radial")]
fn main() {
    loop {
        clear_screen(Color::NEUTRAL_100);

        draw_radial_gradient_circle(
            Vec2::splat(200.0),
            200.0,
            Color::RED_500,
            Color::NEUTRAL_100,
        );

        draw_radial_gradient_circle_offset(
            Vec2::new(600.0, 200.0),
            200.0,
            Color::SKY_500,
            Color::NEUTRAL_100,
            vec2(-100.0, 0.0),
        );

        if should_quit() {
            break;
        }

        next_frame().await;
    }
}
