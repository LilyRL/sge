use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("3D Shapes")?;

    // for lines and wireframes you can also use line_3d_flat and cube_wireframe_flat
    let mesh = cube_wireframe(Vec3::ZERO, 3.0, 0.05);
    let material = create_flat_material(Color::RED_300);
    let outline = Object3D::from_mesh_and_material(mesh, material);

    let mesh = cube(Vec3::ZERO, 2.0);
    let material = create_gouraud_material(Color::SKY_400, Color::SKY_800, vec3(3.0, 10.0, 4.0));
    let cube = Object3D::from_mesh_and_material(mesh, material);

    let mut controller = OrbitCameraController::new(Vec3::ZERO);

    loop {
        clear_screen(Color::PINK_100);

        controller.update();

        cube.draw();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
