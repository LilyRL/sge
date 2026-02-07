use engine_color::Color;

pub mod card;
pub use card::*;

pub mod progress_bar;
pub use progress_bar::*;

pub mod button;
pub use button::*;

pub const PRIMARY: Color = Color::hex(0xC0C0C0);
pub const SECONDARY: Color = Color::hex(0x008081);
pub const TERTIARY: Color = Color::hex(0x000181);
pub const INFO: Color = Color::hex(0x1E90FF);
pub const SUCCESS: Color = Color::hex(0x00BF9A);
pub const WARNING: Color = Color::hex(0xF5B759);
pub const DANGER: Color = Color::hex(0xFA5252);
pub const BLACK: Color = Color::hex(0x08080E);
pub const WHITE: Color = Color::hex(0xFCFCFC);
