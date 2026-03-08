use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Post processing")?;

    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);
    let material = create_gouraud_material(Color::SLATE_200, Color::SLATE_600, Vec3::Y);
    let obj = Object3D::from_obj_bytes_with_material(
        include_bytes!("../assets/models/suzanne.obj"),
        material,
    )?;

    loop {
        clear_screen(Color::NEUTRAL_950);
        orbit_controller.update();

        obj.draw();

        sharpen_screen(100.0);

        run_egui(|ui| {
            draw_debug_info(ui);
        });

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
