#[cfg(feature = "debugging")]
mod implementation;
#[cfg(feature = "debugging")]
pub use implementation::*;

#[cfg(feature = "debugging")]
#[inline]
pub fn debugger_add_vertices(vertices: usize) {
    let debug = get_debug_info();
    debug.current_frame_mut().vertex_count += vertices;
}

#[cfg(not(feature = "debugging"))]
#[inline]
pub fn debugger_add_vertices(_vertices: usize) {}

#[cfg(feature = "debugging")]
#[inline]
pub fn debugger_add_indices(indices: usize) {
    let debug = get_debug_info();
    debug.current_frame_mut().index_count += indices;
}

#[cfg(not(feature = "debugging"))]
#[inline]
pub fn debugger_add_indices(_indices: usize) {}

#[cfg(feature = "debugging")]
#[inline]
pub fn debugger_add_draw_calls(count: usize) {
    let debug = get_debug_info();
    debug.current_frame_mut().draw_calls += count;
}

#[cfg(not(feature = "debugging"))]
#[inline]
pub fn debugger_add_draw_calls(_count: usize) {}

#[cfg(feature = "debugging")]
#[inline]
pub fn debugger_add_drawn_objects(count: usize) {
    let debug = get_debug_info();
    debug.current_frame_mut().drawn_objects += count;
}

#[cfg(not(feature = "debugging"))]
#[inline]
pub fn debugger_add_drawn_objects(_count: usize) {}
