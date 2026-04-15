use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Particle emitters")?;

    let emitter = ParticleEmitter::builder()
        .shape(Poly::new(Vec2::ZERO, 20.0, 3, Color::GRAY_500))
        .duration(10.0)
        .acceleration(vec2(0.0, 0.5))
        .end_color(Color::GRAY_900)
        .color_randomness(Color::GRAY_900)
        .size_randomness(1.0)
        .rotation_speed_randomness(1.0)
        .spawn_interval(0.01)
        .spawn_interval_randomness(0.01)
        .lifetime(3.0)
        .direction_randomness(0.5)
        .speed(25.0)
        .speed_randomness(10.0)
        .build();

    let mut system = ParticleSystem::new();

    loop {
        if is_first_frame() {
            system.spawn_emitter(emitter.clone(), Vec2::ZERO);
        }

        draw_text(10.0 - time(), vec2(10.0, 5.0));

        clear_screen(Color::GRAY_900);
        system.update();

        draw_circle_world(Vec2::ZERO, 10.0, Color::GRAY_800);
        system.draw_world();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
