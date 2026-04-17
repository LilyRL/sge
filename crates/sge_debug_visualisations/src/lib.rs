use sge_api::shapes_2d::draw_rect;
use sge_color::Color;
use sge_debugging::{FRAME_BACKLOG, avg_fps, get_debug_info};
use sge_programs::get_programs_state;
use sge_rendering::{
    get_render_state,
    materials::get_materials_state,
    object_3d::get_objects_state,
    pipeline::{RenderStep, get_render_textures_state},
};
use sge_text::{draw_colored_text, draw_text};
use sge_textures::get_texture_state;
use sge_ui::prelude::*;
use sge_vectors::{Vec2, vec2};

pub mod grid;

pub fn draw_debug_info() {
    let debug = get_debug_info();
    let current_frame = debug.previous_frame();
    let max = debug.max;

    let vertex_points: Vec<_> = (1..FRAME_BACKLOG - 1)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            frame.vertex_count
        })
        .collect();
    let index_points: Vec<_> = (1..FRAME_BACKLOG - 1)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            frame.index_count
        })
        .collect();
    let draw_call_points: Vec<_> = (1..FRAME_BACKLOG - 1)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            frame.draw_calls
        })
        .collect();
    let engine_time_points: Vec<_> = (1..FRAME_BACKLOG - 1)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            frame.engine_time
        })
        .collect();

    let window = flat::FloatingWindow::with_size(
        "Debug Info",
        Vec2::new(439.0, 600.0),
        0xDEEB,
        Col::with_gap(
            5.0,
            [
                Row::with_gap(
                    20.0,
                    [
                        Col::with_gap(
                            5.0,
                            [
                                // vertices
                                Text::new(format!("Vertex count: {}", current_frame.vertex_count)),
                                flat::LineChart::with_y(
                                    vertex_points,
                                    200.0,
                                    200.0,
                                    max.vertex_count,
                                ),
                            ],
                        ),
                        Col::with_gap(
                            5.0,
                            [
                                // indices
                                Text::new(format!("Index count: {}", current_frame.index_count)),
                                flat::LineChart::with_y(
                                    index_points,
                                    200.0,
                                    200.0,
                                    max.index_count,
                                ),
                            ],
                        ),
                    ],
                ),
                Row::with_gap(
                    20.0,
                    [
                        Col::with_gap(
                            5.0,
                            [
                                // draw calls
                                Text::new(format!("Draw call count: {}", current_frame.draw_calls)),
                                flat::LineChart::with_y(
                                    draw_call_points,
                                    200.0,
                                    200.0,
                                    max.draw_calls,
                                ),
                            ],
                        ),
                        Col::with_gap(
                            5.0,
                            [
                                // engine_time
                                Text::new(format!(
                                    "Engine time (ms): {:.3}",
                                    current_frame.engine_time
                                )),
                                flat::LineChart::with_y(
                                    engine_time_points,
                                    200.0,
                                    200.0,
                                    max.engine_time,
                                ),
                            ],
                        ),
                    ],
                ),
                // text
                Text::new(format!("Textures: {}", get_texture_state().len())),
                Text::new(format!(
                    "Render textures: {}",
                    get_render_textures_state().len()
                )),
                Text::new(format!("Programs: {}", get_programs_state().len())),
                Text::new(format!("Materials: {}", get_materials_state().len())),
                Text::new(format!("Objects: {}", get_objects_state().len())),
                Text::new(format!("FPS: {:.1}", debug.fps.avg())),
            ],
        ),
    );

    draw_ui_window(window);
}

pub fn debug_render_steps() {
    let state = get_render_state();

    eprintln!("\nRender steps");

    for step in &state.render_pipeline.steps {
        match step {
            RenderStep::Drawing(_) => eprintln!("- Draw step"),
            RenderStep::PostProcessing(_) => eprintln!("- Post processing step"),
        }
    }
}

pub fn draw_fps() {
    draw_text(format!("{:.1}FPS", avg_fps()), Vec2::new(10.0, 5.0));
}

pub fn draw_fps_bg() {
    let fps_text = format!("{:.1}FPS", avg_fps());
    let text_size = sge_text::measure_text(&fps_text).size;
    draw_rect(
        Vec2::ZERO,
        text_size + vec2(20.0, 10.0),
        Color::BLACK.with_alpha(0.5),
    );
    draw_text(format!("{:.1}FPS", avg_fps()), Vec2::new(10.0, 5.0));
}

pub fn draw_fps_black() {
    draw_colored_text(
        format!("{:.1}FPS", avg_fps()),
        Vec2::new(10.0, 5.0),
        Color::BLACK,
    );
}
