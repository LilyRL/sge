use bevy_math::Vec2;
use egui_glium::egui_winit::egui::Window;
use egui_plot::{Line, Plot, PlotPoints};
use sge_debugging::{FRAME_BACKLOG, avg_fps, get_debug_info};
use sge_programs::get_programs_state;
use sge_rendering::{
    get_render_state,
    materials::get_materials_state,
    object_3d::get_objects_state,
    pipeline::{RenderStep, get_render_textures_state},
};
use sge_text::draw_text;
use sge_textures::get_texture_state;

pub mod grid;

pub fn draw_debug_info(ui: &egui_glium::egui_winit::egui::Context) {
    let debug = get_debug_info();
    let current_frame = debug.previous_frame();

    let vertex_points: PlotPoints = (0..FRAME_BACKLOG)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            [i as f64, frame.vertex_count as f64]
        })
        .collect();
    let index_points: PlotPoints = (0..FRAME_BACKLOG)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            [i as f64, frame.index_count as f64]
        })
        .collect();
    let draw_call_points: PlotPoints = (0..FRAME_BACKLOG)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            [i as f64, frame.draw_calls as f64]
        })
        .collect();
    let drawn_object_points: PlotPoints = (0..FRAME_BACKLOG)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            [i as f64, frame.drawn_objects as f64]
        })
        .collect();
    let engine_time_points: PlotPoints = (0..FRAME_BACKLOG)
        .map(|i| {
            let frame = &debug.frames[(i + debug.frame_offset) % FRAME_BACKLOG];
            [i as f64, frame.engine_time]
        })
        .collect();

    let vertex_line = Line::new(vertex_points);
    let index_line = Line::new(index_points);
    let draw_call_line = Line::new(draw_call_points);
    let drawn_object_line = Line::new(drawn_object_points);
    let engine_time_line = Line::new(engine_time_points);

    Window::new("Debug info").show(ui, |ui| {
        for (id, max, label, line, current) in [
            (
                "vertex_plot",
                debug.max.vertex_count,
                "Vertex count",
                vertex_line,
                current_frame.vertex_count as f32,
            ),
            (
                "index_plot",
                debug.max.index_count,
                "Indice count",
                index_line,
                current_frame.index_count as f32,
            ),
            (
                "draw_plot",
                debug.max.draw_calls,
                "Draw call count",
                draw_call_line,
                current_frame.draw_calls as f32,
            ),
            (
                "object_plot",
                debug.max.drawn_objects,
                "Drawn object count",
                drawn_object_line,
                current_frame.drawn_objects as f32,
            ),
            (
                "engine_time_plot",
                debug.max.engine_time.ceil() as usize,
                "Engine time (ms)",
                engine_time_line,
                current_frame.engine_time as f32,
            ),
        ] {
            ui.label(format!("{}: {}", label, current));
            Plot::new(id)
                .height(100.0)
                .include_y(max as f64 * 1.5)
                .include_y(0.0)
                .allow_scroll(false)
                .allow_drag(false)
                .allow_zoom(false)
                .y_axis_label(label)
                .show(ui, |ui| ui.line(line));
        }

        ui.label(format!("Textures: {}", get_texture_state().len()));
        ui.label(format!(
            "Render textures: {}",
            get_render_textures_state().len()
        ));
        ui.label(format!("Programs: {}", get_programs_state().len()));
        ui.label(format!("Materials: {}", get_materials_state().len()));
        ui.label(format!("Objects: {}", get_objects_state().len()));

        ui.label(format!("FPS: {:.1}", debug.fps.avg()));
        ui.label(format!(
            "Engine time: {:.3}ms",
            debug.current_frame().engine_time
        ));
    });
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
