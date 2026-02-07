use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("3D Shapes")?;

    // for lines and wireframes you can also use line_3d_flat and cube_wireframe_flat
    let mesh = cube_wireframe(Vec3::ZERO, 3.0, 0.05);
    let material = create_flat_material(Color::RED_300);
    let mut outline = Object3D::from_mesh_and_material(mesh, material);

    let mesh = cube(Vec3::ZERO, 2.0);
    let material = create_flat_material(Color::SKY_300);
    let mut cube = Object3D::from_mesh_and_material(mesh, material);

    let mut controller = OrbitCameraController::new(Vec3::ZERO);

    loop {
        let dt = delta_time();

        clear_screen(Color::PINK_100);

        cube.transform.rotate_by(Quat::from_rotation_x(1.0 * dt));
        cube.transform.rotate_by(Quat::from_rotation_y(0.5 * dt));
        cube.transform.rotate_by(Quat::from_rotation_z(0.25 * dt));

        outline.transform.rotate_by(Quat::from_rotation_z(1.0 * dt));
        outline.transform.rotate_by(Quat::from_rotation_x(0.5 * dt));
        outline
            .transform
            .rotate_by(Quat::from_rotation_y(0.25 * dt));

        controller.update();

        cube.draw();
        outline.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
