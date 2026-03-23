#![allow(clippy::new_ret_no_self)]

use super::*;

mod active_fill;
mod align;
mod aspect_ratio;
mod border;
mod box_fill;
mod button;
mod center;
mod circle;
mod col;
mod drawer;
mod empty;
mod fill;
mod fit;
mod flex;
mod gradient_fill;
mod grid;
mod hoverable;
mod image;
mod inactive_overlay;
mod input;
mod loading_bar;
mod padding;
mod progress_bar;
mod rounded_fill;
mod row;
mod scissor_box;
mod scroll;
mod sized_box;
mod stack;
mod text;
mod window;

pub use active_fill::*;
pub use align::*;
pub use aspect_ratio::*;
pub use border::*;
pub use box_fill::*;
pub use button::*;
pub use center::*;
pub use circle::*;
pub use col::*;
pub use drawer::*;
pub use empty::*;
pub use fill::*;
pub use fit::*;
pub use flex::*;
pub use gradient_fill::*;
pub use grid::*;
pub use hoverable::*;
pub use image::*;
pub use inactive_overlay::*;
pub use input::{TextInput, text_input_changed, text_input_value};
pub use loading_bar::*;
pub use padding::*;
pub use progress_bar::*;
pub use rounded_fill::*;
pub use row::*;
pub use scissor_box::*;
pub use scroll::*;
pub use sized_box::*;
pub use stack::*;
pub use text::*;
pub use window::*;

use sge_window::window_width;

pub fn media_query<T>(small: T, medium: T, large: T) -> T {
    let width = window_width();

    if width < 600.0 {
        small
    } else if width < 2000.0 {
        medium
    } else {
        large
    }
}
