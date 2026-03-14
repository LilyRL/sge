use fps_ticker::Fps;

pub const FRAME_BACKLOG: usize = 1000;

global::global!(DebugInfo, debug_info);

pub struct DebugInfo {
    pub fps: Fps,
    pub frame_offset: usize,
    pub frames: [FrameInfo; FRAME_BACKLOG],
    pub max: FrameInfo,
}

#[derive(Clone, Copy)]
pub struct FrameInfo {
    pub vertex_count: usize,
    pub index_count: usize,
    pub draw_calls: usize,
    pub drawn_objects: usize,
    pub engine_time: f64,
}

impl FrameInfo {
    pub const ZERO: Self = Self {
        vertex_count: 0,
        index_count: 0,
        draw_calls: 0,
        drawn_objects: 0,
        engine_time: 0.0,
    };
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            fps: Fps::default(),
            frame_offset: 0,
            frames: [FrameInfo::ZERO; FRAME_BACKLOG],
            max: FrameInfo::ZERO,
        }
    }

    pub fn next_frame(&mut self) {
        let current_frame = *self.current_frame();
        self.max.index_count = self.max.index_count.max(current_frame.index_count);
        self.max.vertex_count = self.max.vertex_count.max(current_frame.vertex_count);
        self.max.draw_calls = self.max.draw_calls.max(current_frame.draw_calls);
        self.max.drawn_objects = self.max.drawn_objects.max(current_frame.drawn_objects);
        self.max.engine_time = self.max.engine_time.max(current_frame.engine_time);

        self.frame_offset = (self.frame_offset + 1) % FRAME_BACKLOG;
        self.fps.tick();
        self.frames[self.frame_offset] = FrameInfo::ZERO;
    }

    pub fn current_frame(&self) -> &FrameInfo {
        &self.frames[self.frame_offset]
    }

    pub fn previous_frame(&self) -> &FrameInfo {
        &self.frames[(self.frame_offset + FRAME_BACKLOG - 1) % FRAME_BACKLOG]
    }

    pub fn current_frame_mut(&mut self) -> &mut FrameInfo {
        &mut self.frames[self.frame_offset]
    }
}

impl Default for DebugInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub fn init() {
    set_debug_info(DebugInfo::new());
    log::info!("Initialized debugger");
}

pub fn avg_fps() -> f64 {
    get_debug_info().fps.avg()
}

pub fn min_fps() -> f64 {
    get_debug_info().fps.min()
}

pub fn max_fps() -> f64 {
    get_debug_info().fps.max()
}

pub fn get_engine_time() -> f64 {
    get_debug_info().previous_frame().engine_time
}

pub fn get_max_engine_time() -> f64 {
    get_debug_info().max.engine_time
}

pub fn get_drawn_objects() -> usize {
    get_debug_info().previous_frame().drawn_objects
}

pub fn get_max_drawn_objects() -> usize {
    get_debug_info().max.drawn_objects
}

pub fn get_draw_calls() -> usize {
    get_debug_info().previous_frame().draw_calls
}

pub fn get_max_draw_calls() -> usize {
    get_debug_info().max.draw_calls
}

pub fn get_vertex_count() -> usize {
    get_debug_info().previous_frame().vertex_count
}

pub fn get_max_vertex_count() -> usize {
    get_debug_info().max.vertex_count
}

pub fn get_index_count() -> usize {
    get_debug_info().previous_frame().index_count
}

pub fn get_max_index_count() -> usize {
    get_debug_info().max.index_count
}
