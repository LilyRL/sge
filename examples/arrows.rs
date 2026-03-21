use glium::PolygonMode;
use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Arrows")?;

    let mut controller = PanningCameraController::new();

    loop {
        controller.update();

        if key_pressed(KeyCode::KeyD) {
            set_wireframe_line_width(10.0);
            toggle_wireframe();
        }

        let thickness = 5.0;
        draw_circle_world(vec2(-50.0, -60.0), 10.0, Color::RED_400);
        draw_circle_world(vec2(50.0, -60.0), 10.0, Color::RED_400);
        draw_zig_zag_world_ex(
            vec2(-50.0, -60.0),
            vec2(50.0, -60.0),
            thickness,
            Color::WHITE,
            5.0,
            10,
        );
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
