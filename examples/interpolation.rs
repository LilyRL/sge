use std::f32::consts::TAU;

use sge::prelude::*;

fn random_rect() -> Rect {
    let sf = window_height().min(window_width());
    let center: Vec2 = rand::<Vec2>() * sf;
    let size: Vec2 = rand::<Vec2>() * sf;
    let color = rand_choice(&Palette::PALETTES).v500;

    Rect::from_center(center, size, color).with_rotation(rand::<f32>() * TAU)
}

fn main() -> anyhow::Result<()> {
    init("Animation")?;

    let mut animation_controller =
        AnimationController::new(random_rect(), random_rect(), 1.0, EaseOutElastic);

    loop {
        clear_screen(Color::hex(0x101013));

        draw(&animation_controller.value());

        if animation_controller.is_complete() {
            animation_controller.now_animate_towards(random_rect());
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
