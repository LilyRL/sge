use d2::DrawQueue2D;
use d3::DrawQueue3D;
use glium::Rect;
use materials::init_materials;
use pipeline::RenderPipeline;

pub mod api;
pub mod d2;
pub mod d3;
pub mod materials;
pub mod object_3d;
pub mod pipeline;
pub mod post_processing;
pub mod scissor;
pub mod shapes_3d;

pub struct DrawQueues {
    pub draw_queue_2d: DrawQueue2D,
    pub world_draw_queue_2d: DrawQueue2D,
    pub draw_queue_3d: DrawQueue3D,
}

pub struct RenderState {
    pub render_pipeline: RenderPipeline,
    pub texture_pipeline: Option<RenderPipeline>,
    pub scissor_stack: Vec<Rect>,
}

global::global!(RenderState, render_state);

pub fn init() {
    object_3d::init();
    pipeline::init();
    set_render_state(RenderState {
        render_pipeline: RenderPipeline::screen(),
        texture_pipeline: None,
        scissor_stack: vec![],
    });
    log::info!("Initialized render state");
    init_materials();
    log::info!("Initialized materials");
}
