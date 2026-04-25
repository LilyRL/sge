use sge::prelude::*;

#[main("Default material")]
async fn main() -> anyhow::Result<()> {
    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);

    let object = Object3D::from_obj(include_str!("../assets/models/suzanne.obj"))?;
    // object.compute_smooth_normals();

    loop {
        clear_screen(Color::hex(0x3F3F3F));
        orbit_controller.update();

        object.draw();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
