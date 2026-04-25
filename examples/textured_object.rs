use sge::prelude::*;

#[main("3D")]
fn main() -> anyhow::Result<()> {
    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);

    let texture = load_texture_from_bytes_sync(
        include_bytes!("../assets/models/beachball.png"),
        ImageFormat::Png,
    )?;
    let material = create_textured_material(texture);
    let data = include_bytes!("../assets/models/beachball.obj");
    let ball = Object3D::from_obj_bytes_with_material(data, material)?;

    loop {
        clear_screen(Color::PURPLE_300);

        orbit_controller.update();

        ball.draw();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
