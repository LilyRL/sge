use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    let opts = EngineCreationOptions::builder()
        .title("Logging".to_string())
        .min_log_level(LevelFilter::max())
        .log_verbosity(Verbosity::Medium)
        .build();

    init_custom(opts)?;
    let mut controller = PanningCameraController::new();
    set_max_drawn_log_lines(100);
    show_debug_info();

    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");
    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");
    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");
    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");

    loop {
        controller.update();
        draw_logs();

        let mut font = default_font();
        let texture = font.texture();
        draw_texture_world(texture, Vec2::ZERO, 1000.0);
        draw_square_outline_world(Vec2::ZERO, 1000.0, 2.0, Color::NEUTRAL_500);

        run_egui(|_| {});

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
