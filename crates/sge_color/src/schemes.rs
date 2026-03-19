#![allow(unused)]

use super::Color;

#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
pub struct ColorScheme {
    pub bg0: Color,
    pub bg1: Color,
    pub bg2: Color,
    pub bg3: Color,
    pub bg4: Color,

    pub fg0: Color,
    pub fg1: Color,
    pub fg2: Color,
    pub fg3: Color,
    pub fg4: Color,

    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub blue: Color,
    pub purple: Color,
    pub aqua: Color,
    pub gray: Color,

    pub light_red: Color,
    pub light_orange: Color,
    pub light_yellow: Color,
    pub light_green: Color,
    pub light_blue: Color,
    pub light_purple: Color,
    pub light_aqua: Color,
    pub light_gray: Color,
}

impl ColorScheme {
    pub fn palette(&self) -> [Color; 26] {
        [
            self.bg0,
            self.bg1,
            self.bg2,
            self.bg3,
            self.bg4,
            self.fg4,
            self.fg2,
            self.fg0,
            self.fg1,
            self.fg3,
            self.red,
            self.orange,
            self.yellow,
            self.green,
            self.blue,
            self.purple,
            self.aqua,
            self.gray,
            self.light_red,
            self.light_orange,
            self.light_yellow,
            self.light_green,
            self.light_blue,
            self.light_purple,
            self.light_aqua,
            self.light_gray,
        ]
    }
}

impl ColorScheme {
    pub const GRUVBOX_DARK: Self = Self {
        bg0: Color::from_rgba_u8(0x1e, 0x1e, 0x1e, 255),
        bg1: Color::from_rgba_u8(0x26, 0x24, 0x23, 255),
        bg2: Color::from_rgba_u8(0x2e, 0x2a, 0x29, 255),
        bg3: Color::from_rgba_u8(0x3f, 0x39, 0x35, 255),
        bg4: Color::from_rgba_u8(0x53, 0x4a, 0x42, 255),

        fg0: Color::from_rgba_u8(0xcb, 0xb8, 0x90, 255),
        fg1: Color::from_rgba_u8(0xaf, 0x9f, 0x81, 255),
        fg2: Color::from_rgba_u8(0x97, 0x87, 0x71, 255),
        fg3: Color::from_rgba_u8(0x7f, 0x70, 0x61, 255),
        fg4: Color::from_rgba_u8(0x68, 0x5c, 0x51, 255),

        red: Color::from_rgba_u8(0xf7, 0x30, 0x28, 255),
        orange: Color::from_rgba_u8(0xfb, 0x6a, 0x16, 255),
        yellow: Color::from_rgba_u8(0xf7, 0xb1, 0x25, 255),
        green: Color::from_rgba_u8(0xaa, 0xb0, 0x1e, 255),
        blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255),
        purple: Color::from_rgba_u8(0xc7, 0x70, 0x89, 255),
        aqua: Color::from_rgba_u8(0x7d, 0xb6, 0x69, 255),
        gray: Color::from_rgba_u8(0x7f, 0x70, 0x61, 255),

        light_red: Color::from_rgba_u8(0xf7, 0x30, 0x28, 255),
        light_orange: Color::from_rgba_u8(0xfb, 0x6a, 0x16, 255),
        light_yellow: Color::from_rgba_u8(0xfa, 0xee, 0xbb, 255),
        light_green: Color::from_rgba_u8(0x35, 0x6a, 0x46, 255),
        light_blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255),
        light_purple: Color::from_rgba_u8(0x7b, 0x2b, 0x5e, 255),
        light_aqua: Color::from_rgba_u8(0x71, 0x95, 0x86, 255),
        light_gray: Color::from_rgba_u8(0xaf, 0x9f, 0x81, 255),
    };

    pub const GRUVBOX_LIGHT: Self = Self {
        bg0: Color::from_rgba_u8(0xf8, 0xf4, 0xcd, 255),
        bg1: Color::from_rgba_u8(0xef, 0xdf, 0xae, 255),
        bg2: Color::from_rgba_u8(0xe6, 0xd4, 0xa3, 255),
        bg3: Color::from_rgba_u8(0xcb, 0xb8, 0x90, 255),
        bg4: Color::from_rgba_u8(0xaf, 0x9f, 0x81, 255),

        fg0: Color::from_rgba_u8(0x26, 0x24, 0x23, 255),
        fg1: Color::from_rgba_u8(0x3f, 0x39, 0x35, 255),
        fg2: Color::from_rgba_u8(0x53, 0x4a, 0x42, 255),
        fg3: Color::from_rgba_u8(0x68, 0x5c, 0x51, 255),
        fg4: Color::from_rgba_u8(0x7f, 0x70, 0x61, 255),

        red: Color::from_rgba_u8(0x89, 0x00, 0x09, 255),
        orange: Color::from_rgba_u8(0x9d, 0x28, 0x07, 255),
        yellow: Color::from_rgba_u8(0xa5, 0x63, 0x11, 255),
        green: Color::from_rgba_u8(0x66, 0x62, 0x0d, 255),
        blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255),
        purple: Color::from_rgba_u8(0x7b, 0x2b, 0x5e, 255),
        aqua: Color::from_rgba_u8(0x35, 0x6a, 0x46, 255),
        gray: Color::from_rgba_u8(0x68, 0x5c, 0x51, 255),

        light_red: Color::from_rgba_u8(0xf7, 0x30, 0x28, 255),
        light_orange: Color::from_rgba_u8(0xfb, 0x6a, 0x16, 255),
        light_yellow: Color::from_rgba_u8(0xf7, 0xb1, 0x25, 255),
        light_green: Color::from_rgba_u8(0xaa, 0xb0, 0x1e, 255),
        light_blue: Color::from_rgba_u8(0x0e, 0x53, 0x65, 255),
        light_purple: Color::from_rgba_u8(0xc7, 0x70, 0x89, 255),
        light_aqua: Color::from_rgba_u8(0x7d, 0xb6, 0x69, 255),
        light_gray: Color::from_rgba_u8(0x97, 0x87, 0x71, 255),
    };

    pub const CATPPUCCIN_LATTE: Self = Self {
        bg0: Color::from_rgba_u8(0xef, 0xf1, 0xf5, 255),
        bg1: Color::from_rgba_u8(0xe6, 0xe9, 0xef, 255),
        bg2: Color::from_rgba_u8(0xdc, 0xe0, 0xe8, 255),
        bg3: Color::from_rgba_u8(0xcc, 0xd0, 0xda, 255),
        bg4: Color::from_rgba_u8(0xbc, 0xc0, 0xcc, 255),

        fg0: Color::from_rgba_u8(0x4c, 0x4f, 0x69, 255),
        fg1: Color::from_rgba_u8(0x5c, 0x5f, 0x77, 255),
        fg2: Color::from_rgba_u8(0x6c, 0x6f, 0x85, 255),
        fg3: Color::from_rgba_u8(0x7c, 0x7f, 0x93, 255),
        fg4: Color::from_rgba_u8(0xac, 0xb0, 0xbe, 255),

        red: Color::from_rgba_u8(0xd2, 0x0f, 0x39, 255),
        orange: Color::from_rgba_u8(0xfe, 0x64, 0x0b, 255),
        yellow: Color::from_rgba_u8(0xdf, 0x8e, 0x1d, 255),
        green: Color::from_rgba_u8(0x40, 0xa0, 0x2b, 255),
        blue: Color::from_rgba_u8(0x1e, 0x66, 0xf5, 255),
        purple: Color::from_rgba_u8(0x88, 0x39, 0xef, 255),
        aqua: Color::from_rgba_u8(0x17, 0x92, 0x99, 255),
        gray: Color::from_rgba_u8(0x8c, 0x8f, 0xa1, 255),

        light_red: Color::from_rgba_u8(0xe6, 0x45, 0x53, 255),
        light_orange: Color::from_rgba_u8(0xdc, 0x8a, 0x78, 255),
        light_yellow: Color::from_rgba_u8(0xdd, 0x78, 0x78, 255),
        light_green: Color::from_rgba_u8(0x04, 0xa5, 0xe5, 255),
        light_blue: Color::from_rgba_u8(0x72, 0x87, 0xfd, 255),
        light_purple: Color::from_rgba_u8(0xea, 0x76, 0xcb, 255),
        light_aqua: Color::from_rgba_u8(0x20, 0x9f, 0xb5, 255),
        light_gray: Color::from_rgba_u8(0x9c, 0xa0, 0xb0, 255),
    };

    pub const CATPPUCCIN_FRAPPE: Self = Self {
        bg0: Color::from_rgba_u8(0x30, 0x34, 0x46, 255),
        bg1: Color::from_rgba_u8(0x29, 0x2c, 0x3c, 255),
        bg2: Color::from_rgba_u8(0x23, 0x26, 0x34, 255),
        bg3: Color::from_rgba_u8(0x41, 0x45, 0x59, 255),
        bg4: Color::from_rgba_u8(0x51, 0x57, 0x6d, 255),

        fg0: Color::from_rgba_u8(0xc6, 0xd0, 0xf5, 255),
        fg1: Color::from_rgba_u8(0xb5, 0xbf, 0xe2, 255),
        fg2: Color::from_rgba_u8(0xa5, 0xad, 0xce, 255),
        fg3: Color::from_rgba_u8(0x94, 0x9c, 0xbb, 255),
        fg4: Color::from_rgba_u8(0x62, 0x68, 0x80, 255),

        red: Color::from_rgba_u8(0xe7, 0x82, 0x84, 255),
        orange: Color::from_rgba_u8(0xef, 0x9f, 0x76, 255),
        yellow: Color::from_rgba_u8(0xe5, 0xc8, 0x90, 255),
        green: Color::from_rgba_u8(0xa6, 0xd1, 0x89, 255),
        blue: Color::from_rgba_u8(0x8c, 0xaa, 0xee, 255),
        purple: Color::from_rgba_u8(0xca, 0x9e, 0xe6, 255),
        aqua: Color::from_rgba_u8(0x81, 0xc8, 0xbe, 255),
        gray: Color::from_rgba_u8(0x83, 0x8b, 0xa7, 255),

        light_red: Color::from_rgba_u8(0xea, 0x99, 0x9c, 255),
        light_orange: Color::from_rgba_u8(0xf2, 0xd5, 0xcf, 255),
        light_yellow: Color::from_rgba_u8(0xee, 0xbe, 0xbe, 255),
        light_green: Color::from_rgba_u8(0x99, 0xd1, 0xdb, 255),
        light_blue: Color::from_rgba_u8(0xba, 0xbb, 0xf1, 255),
        light_purple: Color::from_rgba_u8(0xf4, 0xb8, 0xe4, 255),
        light_aqua: Color::from_rgba_u8(0x85, 0xc1, 0xdc, 255),
        light_gray: Color::from_rgba_u8(0x73, 0x79, 0x94, 255),
    };

    pub const CATPPUCCIN_MACCHIATO: Self = Self {
        bg0: Color::from_rgba_u8(0x24, 0x27, 0x3a, 255),
        bg1: Color::from_rgba_u8(0x1e, 0x20, 0x30, 255),
        bg2: Color::from_rgba_u8(0x18, 0x19, 0x26, 255),
        bg3: Color::from_rgba_u8(0x36, 0x3a, 0x4f, 255),
        bg4: Color::from_rgba_u8(0x49, 0x4d, 0x64, 255),

        fg0: Color::from_rgba_u8(0xca, 0xd3, 0xf5, 255),
        fg1: Color::from_rgba_u8(0xb8, 0xc0, 0xe0, 255),
        fg2: Color::from_rgba_u8(0xa5, 0xad, 0xcb, 255),
        fg3: Color::from_rgba_u8(0x93, 0x9a, 0xb7, 255),
        fg4: Color::from_rgba_u8(0x5b, 0x60, 0x78, 255),

        red: Color::from_rgba_u8(0xed, 0x87, 0x96, 255),
        orange: Color::from_rgba_u8(0xf5, 0xa9, 0x7f, 255),
        yellow: Color::from_rgba_u8(0xee, 0xd4, 0x9f, 255),
        green: Color::from_rgba_u8(0xa6, 0xda, 0x95, 255),
        blue: Color::from_rgba_u8(0x8a, 0xad, 0xf4, 255),
        purple: Color::from_rgba_u8(0xc6, 0xa0, 0xf6, 255),
        aqua: Color::from_rgba_u8(0x8b, 0xd5, 0xca, 255),
        gray: Color::from_rgba_u8(0x80, 0x87, 0xa2, 255),

        light_red: Color::from_rgba_u8(0xee, 0x99, 0xa0, 255),
        light_orange: Color::from_rgba_u8(0xf4, 0xdb, 0xd6, 255),
        light_yellow: Color::from_rgba_u8(0xf0, 0xc6, 0xc6, 255),
        light_green: Color::from_rgba_u8(0x91, 0xd7, 0xe3, 255),
        light_blue: Color::from_rgba_u8(0xb7, 0xbd, 0xf8, 255),
        light_purple: Color::from_rgba_u8(0xf5, 0xbd, 0xe6, 255),
        light_aqua: Color::from_rgba_u8(0x7d, 0xc4, 0xe4, 255),
        light_gray: Color::from_rgba_u8(0x6e, 0x73, 0x8d, 255),
    };

    pub const CATPPUCCIN_MOCHA: Self = Self {
        bg0: Color::from_rgba_u8(0x1e, 0x1e, 0x2e, 255),
        bg1: Color::from_rgba_u8(0x18, 0x18, 0x25, 255),
        bg2: Color::from_rgba_u8(0x11, 0x11, 0x1b, 255),
        bg3: Color::from_rgba_u8(0x31, 0x32, 0x44, 255),
        bg4: Color::from_rgba_u8(0x45, 0x47, 0x5a, 255),

        fg0: Color::from_rgba_u8(0xcd, 0xd6, 0xf4, 255),
        fg1: Color::from_rgba_u8(0xba, 0xc2, 0xde, 255),
        fg2: Color::from_rgba_u8(0xa6, 0xad, 0xc8, 255),
        fg3: Color::from_rgba_u8(0x93, 0x99, 0xb2, 255),
        fg4: Color::from_rgba_u8(0x58, 0x5b, 0x70, 255),

        red: Color::from_rgba_u8(0xf3, 0x8b, 0xa8, 255),
        orange: Color::from_rgba_u8(0xfa, 0xb3, 0x87, 255),
        yellow: Color::from_rgba_u8(0xf9, 0xe2, 0xaf, 255),
        green: Color::from_rgba_u8(0xa6, 0xe3, 0xa1, 255),
        blue: Color::from_rgba_u8(0x89, 0xb4, 0xfa, 255),
        purple: Color::from_rgba_u8(0xcb, 0xa6, 0xf7, 255),
        aqua: Color::from_rgba_u8(0x94, 0xe2, 0xd5, 255),
        gray: Color::from_rgba_u8(0x7f, 0x84, 0x9c, 255),

        light_red: Color::from_rgba_u8(0xeb, 0xa0, 0xac, 255),
        light_orange: Color::from_rgba_u8(0xf5, 0xe0, 0xdc, 255),
        light_yellow: Color::from_rgba_u8(0xf2, 0xcd, 0xcd, 255),
        light_green: Color::from_rgba_u8(0x89, 0xdc, 0xeb, 255),
        light_blue: Color::from_rgba_u8(0xb4, 0xbe, 0xfe, 255),
        light_purple: Color::from_rgba_u8(0xf5, 0xc2, 0xe7, 255),
        light_aqua: Color::from_rgba_u8(0x74, 0xc7, 0xec, 255),
        light_gray: Color::from_rgba_u8(0x6c, 0x70, 0x86, 255),
    };

    pub const LACKLUSTER: Self = Self {
        bg0: Color::from_rgba_u8(0x08, 0x08, 0x08, 255), // base-1
        bg1: Color::from_rgba_u8(0x10, 0x10, 0x10, 255), // base0
        bg2: Color::from_rgba_u8(0x19, 0x19, 0x19, 255), // base1
        bg3: Color::from_rgba_u8(0x2a, 0x2a, 0x2a, 255), // base2
        bg4: Color::from_rgba_u8(0x3a, 0x3a, 0x3a, 255), // base3 / comment

        fg0: Color::from_rgba_u8(0xdd, 0xdd, 0xdd, 255), // base9
        fg1: Color::from_rgba_u8(0xcc, 0xcc, 0xcc, 255), // base8
        fg2: Color::from_rgba_u8(0xaa, 0xaa, 0xaa, 255), // base7
        fg3: Color::from_rgba_u8(0x7a, 0x7a, 0x7a, 255), // base6 / punctuation
        fg4: Color::from_rgba_u8(0x55, 0x55, 0x55, 255), // base5

        red: Color::from_rgba_u8(0xd7, 0x00, 0x00, 255), // red
        orange: Color::from_rgba_u8(0xff, 0xaa, 0x88, 255), // orange
        yellow: Color::from_rgba_u8(0xad, 0xac, 0x5f, 255), // yellow
        green: Color::from_rgba_u8(0x67, 0x9f, 0x68, 255), // green
        blue: Color::from_rgba_u8(0x69, 0x87, 0xc5, 255), // blue
        purple: Color::from_rgba_u8(0xfa, 0xa8, 0xde, 255), // pink
        aqua: Color::from_rgba_u8(0x70, 0x80, 0x90, 255), // lack
        gray: Color::from_rgba_u8(0x44, 0x44, 0x44, 255), // base4

        light_red: Color::from_rgba_u8(0xd7, 0x00, 0x00, 255), // red (no bright variant)
        light_orange: Color::from_rgba_u8(0xff, 0xaa, 0x88, 255), // orange
        light_yellow: Color::from_rgba_u8(0xde, 0xee, 0xed, 255), // luster
        light_green: Color::from_rgba_u8(0x78, 0x99, 0x78, 255), // special
        light_blue: Color::from_rgba_u8(0x4f, 0x94, 0xcd, 255), // light-blue
        light_purple: Color::from_rgba_u8(0xfa, 0xa8, 0xde, 255), // pink
        light_aqua: Color::from_rgba_u8(0x77, 0x88, 0xaa, 255), // hint
        light_gray: Color::from_rgba_u8(0x7a, 0x7a, 0x7a, 255), // base6
    };
}
