use sge::prelude::*;

const GRID_SIZE: usize = 50;

fn main() -> anyhow::Result<()> {
    init("3D?")?;
    wait_for_events();

    let mut clear_color = Color::PURPLE_300;

    mutate_camera_3d(|c| {
        c.fovy = 80.0;
    });

    let mut show_many = false;
    let mut orbit_controller = OrbitCameraController::new(Vec3::ZERO);
    let mut light_on_camera = false;
    let mut show_grid = false;

    let grid_width = (GRID_SIZE - 1) as f32 * 3.0;
    let grid_center = grid_width / 2.0;
    let grid = create_infinite_grid()?;

    let light_pos = Vec3::new(2.0, 5.0, 0.0);
    let material = create_gouraud_material(Color::SLATE_300, Color::SLATE_500, light_pos);
    let data = include_bytes!("../assets/models/suzanne.obj");
    let suzanne = Object3D::from_obj_bytes_with_material(data, material)?;

    let mut transforms = Vec::with_capacity(GRID_SIZE * GRID_SIZE);
    for x in 0..GRID_SIZE {
        for z in 0..GRID_SIZE {
            let vector = Vec3::new(x as f32 * 3.0, 0.0, z as f32 * 3.0);
            let transform = Transform3D::from_translation(vector);
            transforms.push(transform);
        }
    }

    loop {
        dont_clear_screen();
        let ui = {
            use ui::prelude::*;
            let len = min_window_dimension() / 6.0;
            let color = Color::NEUTRAL_950;

            FlexCol::new([
                FlexBox::Fixed(
                    Col::new([
                        Text::mono("M - show many suzannes"),
                        Text::mono("Y - change color to yellow"),
                        Text::mono("G - change color to grey"),
                        Text::mono("B - change color to blue"),
                        Text::mono("hold R - rainbow background"),
                        Text::mono("K - mirror on the y axis"),
                        Text::mono(", - mirror on the z axis"),
                        Text::mono("I - toggle isometric camera"),
                        Text::mono("L - toggle locking light position to camera position"),
                        Text::mono("X - toggle grid"),
                    ])
                    .padding_xy(len - 50.0, (len / 5.0).floor())
                    .fill(color)
                    .min_height(len),
                ),
                FlexBox::Flex(FlexRow::new([
                    FlexBox::Fixed(EMPTY.fill(color).width(len)),
                    FlexBox::Flex(EMPTY.fill(clear_color)),
                    FlexBox::Fixed(EMPTY.fill(color).width(len)),
                ])),
                FlexBox::Fixed(
                    Col::new([
                        Text::mono(format!(
                            "FPS: A{:.2} -  M{:.2} -  m{:.2}",
                            avg_fps(),
                            max_fps(),
                            min_fps()
                        )),
                        Text::mono(format!(
                            "Engine time: {:.2} - M{:.2}",
                            get_engine_time(),
                            get_max_engine_time()
                        )),
                        Text::mono(format!(
                            "Draw calls: {:.2} - M{:.2}",
                            get_draw_calls(),
                            get_max_draw_calls()
                        )),
                        Text::mono(format!(
                            "Vertices drawn: {:.2} - M{:.2}",
                            get_vertex_count(),
                            get_max_vertex_count()
                        )),
                        Text::mono(format!(
                            "Indices drawn: {:.2} - M{:.2}",
                            get_index_count(),
                            get_max_index_count()
                        )),
                        Text::mono(format!(
                            "Objects drawn: {:.2} - M{:.2}",
                            get_drawn_objects(),
                            get_max_drawn_objects()
                        )),
                        if is_about_to_wait_for_input() {
                            Text::mono("RENDERING PAUSED")
                        } else {
                            EMPTY
                        },
                    ])
                    .padding_xy(len - 50.0, (len / 5.0).floor())
                    .fill(color)
                    .min_height(len),
                ),
            ])
        };
        draw_ui(ui, Vec2::ZERO);

        new_draw_queues();

        if key_pressed(KeyCode::KeyM) {
            show_many = !show_many;
            orbit_controller.set_enabled(!show_many);
        }

        if key_pressed(KeyCode::KeyY) {
            let mat = suzanne.material();

            mat.set_color("regular_color", Color::YELLOW_300);
            mat.set_color("dark_color", Color::YELLOW_500);
            clear_color = Color::YELLOW_200;
        }

        if key_pressed(KeyCode::KeyG) {
            let mat = suzanne.material();

            mat.set_color("regular_color", Color::SLATE_300);
            mat.set_color("dark_color", Color::SLATE_500);
            clear_color = Color::NEUTRAL_100;
        }

        if key_pressed(KeyCode::KeyB) {
            let mat = suzanne.material();

            mat.set_color("regular_color", Color::BLUE_300.hue_rotate(-10.0));
            mat.set_color("dark_color", Color::BLUE_400.desaturate(0.5));
            clear_color = Color::EMERALD_300;
        }

        if key_held(KeyCode::KeyR) {
            clear_color = clear_color.hue_rotate_oklch(5.0);
        }

        if key_pressed(KeyCode::KeyK) {
            suzanne.transform().mirror_y();
        }

        if key_pressed(KeyCode::Comma) {
            suzanne.transform().mirror_z();
        }

        if key_pressed(KeyCode::KeyI) {
            mutate_camera_3d(|c| {
                c.isometric = !c.isometric;
            });
        }

        if key_pressed(KeyCode::KeyX) {
            show_grid = !show_grid;
        }

        orbit_controller.update();

        if key_pressed(KeyCode::KeyL) {
            light_on_camera = !light_on_camera;

            if !light_on_camera {
                suzanne.material().set_vec3("light_pos", light_pos);
            }
        }

        if light_on_camera {
            suzanne
                .material()
                .set_vec3("light_pos", get_camera_3d().eye);
        }

        if show_many {
            mutate_camera_3d(|camera| {
                camera.eye = Vec3::new(
                    grid_center,
                    grid_center * 0.4,
                    grid_center + grid_width * 0.6,
                );
                camera.target = Vec3::new(grid_center, -grid_center, 0.0);
            });

            suzanne.draw_many(transforms.clone());
        } else {
            suzanne.draw();
        }

        if show_grid {
            grid.draw();
        }

        if show_many {}

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
