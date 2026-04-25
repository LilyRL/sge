use sge_color::{Color, schemes::ColorScheme};

pub const COLOR_SCHEME: ColorScheme = ColorScheme::LACKLUSTER;
pub const BUTTON_COLOR: Color = COLOR_SCHEME.bg2;
pub const BUTTON_HOVER_COLOR: Color = COLOR_SCHEME.bg3;
pub const PRIMARY_TEXT_COLOR: Color = COLOR_SCHEME.fg0;
pub const FG0: Color = COLOR_SCHEME.fg0;
pub const FG1: Color = COLOR_SCHEME.fg1;
pub const FG2: Color = COLOR_SCHEME.fg2;
pub const FG3: Color = COLOR_SCHEME.fg3;
pub const FG4: Color = COLOR_SCHEME.fg4;
pub const BG0: Color = COLOR_SCHEME.bg0;
pub const BG1: Color = COLOR_SCHEME.bg1;
pub const BG2: Color = COLOR_SCHEME.bg2;
pub const BG3: Color = COLOR_SCHEME.bg3;
pub const BG4: Color = COLOR_SCHEME.bg4;
pub const ERROR: Color = COLOR_SCHEME.red;

pub mod progress_bar;
pub use progress_bar::*;

pub mod button;
pub use button::*;

pub mod card;
pub use card::*;

pub mod loading_bar;
pub use loading_bar::*;

pub mod input;
pub use input::*;

pub mod drawer;
pub use drawer::*;

pub mod window;
pub use window::*;

pub mod slider;
pub use slider::*;

pub mod line_chart;
pub use line_chart::*;

pub mod selectbox;
pub use selectbox::*;

pub mod color_selector;
pub use color_selector::*;

pub mod async_image;
pub use async_image::*;
