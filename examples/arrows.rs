use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Arrows")?;

    let mut controller = PanningCameraController::new();

    loop {
        controller.update();

        let thickness = 5.0;
        draw_arrow_world(
            vec2(-50.0, -30.0),
            vec2(50.0, -30.0),
            thickness,
            Color::WHITE,
        );
        draw_solid_arrow_world(vec2(-50.0, 0.0), vec2(50.0, 0.0), thickness, Color::WHITE);
        draw_sharp_arrow_world(vec2(-50.0, 30.0), vec2(50.0, 30.0), thickness, Color::WHITE);

        draw_right_angled_arrow_world(
            vec2(50.0, 50.0),
            vec2(150.0, 300.0),
            thickness,
            Color::WHITE,
        );

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
