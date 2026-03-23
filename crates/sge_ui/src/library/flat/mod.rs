use sge_color::{Color, schemes::ColorScheme};

pub const COLOR_SCHEME: ColorScheme = ColorScheme::LACKLUSTER;
pub const BUTTON_COLOR: Color = COLOR_SCHEME.bg2;
pub const BUTTON_HOVER_COLOR: Color = COLOR_SCHEME.bg3;
pub const PRIMARY_TEXT_COLOR: Color = COLOR_SCHEME.fg0;
pub const BG0: Color = COLOR_SCHEME.bg0;
pub const BG1: Color = COLOR_SCHEME.bg1;
pub const BG2: Color = COLOR_SCHEME.bg2;
pub const BG3: Color = COLOR_SCHEME.bg3;
pub const BG4: Color = COLOR_SCHEME.bg4;

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

pub mod expandable;
pub use expandable::*;
