use std::{collections::HashMap, hash::Hash};

use crate::utils::EngineCreate;
use bevy_math::{IVec2, Rect, Vec2};
use engine_4_macros::gen_ref_type;
use error_union::ErrorUnion;
use fontdue::{Metrics, layout::TextStyle};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};

use crate::{
    api::{default_font, dpi_scaling},
    color::{Color, u8::Pixel},
    draw_queue_2d::DrawQueue2D,
    get_state,
    image::Image,
    prelude::{SpriteKey, TextureAtlas, Transform2D},
    textures::TextureRef,
};

pub mod rich_text;

pub struct EngineFont {
    font: fontdue::Font,
    atlas: TextureAtlas,
    characters: HashMap<Glyph, CharacterInfo>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CharacterInfo {
    #[allow(unused)]
    pub offset: IVec2,
    pub advance: f32,
    pub sprite: SpriteKey,
}

#[derive(Default)]
pub struct TextDimensions {
    pub size: Vec2,
    pub final_cursor_pos: Vec2,
    // pub offset_y: f32,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Glyph {
    character: char,
    size: usize, // using usize because i just dont like the idea of using a hashmap of f32 keys, right? that sounds bad right?
}

gen_ref_type!(EngineFont, FontRef, fonts);

#[allow(unused)]
impl FontRef {
    pub fn draw_text(&self, text: impl AsRef<str>, position: Vec2, size: usize) -> TextDimensions {
        draw_text_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                ..Default::default()
            },
        )
    }

    pub fn draw_text_world(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
    ) -> TextDimensions {
        draw_text_world_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                ..Default::default()
            },
        )
    }

    pub fn draw_text_world_ex(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
        color: Color,
        do_dpi_scaling: bool,
    ) -> TextDimensions {
        draw_text_world_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                color,
                do_dpi_scaling,
            },
        )
    }

    pub fn draw_text_ex(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
        color: Color,
        do_dpi_scaling: bool,
    ) -> TextDimensions {
        draw_text_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                color,
                do_dpi_scaling,
            },
        )
    }

    pub fn draw_multiline_text(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
        line_spacing: f32,
    ) -> TextDimensions {
        draw_multiline_text_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                ..Default::default()
            },
            line_spacing,
        )
    }

    pub fn draw_multiline_text_world(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
        line_spacing: f32,
    ) -> TextDimensions {
        draw_multiline_text_world_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                ..Default::default()
            },
            line_spacing,
        )
    }

    pub fn draw_multiline_text_ex(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
        color: Color,
        do_dpi_scaling: bool,
        line_spacing: f32,
    ) -> TextDimensions {
        draw_multiline_text_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                color,
                do_dpi_scaling,
            },
            line_spacing,
        )
    }

    pub fn draw_multiline_text_world_ex(
        &self,
        text: impl AsRef<str>,
        position: Vec2,
        size: usize,
        color: Color,
        do_dpi_scaling: bool,
        line_spacing: f32,
    ) -> TextDimensions {
        draw_multiline_text_world_ex(
            text,
            TextDrawParams {
                font: Some(*self),
                font_size: size,
                position,
                color,
                do_dpi_scaling,
            },
            line_spacing,
        )
    }
}

#[derive(ErrorUnion, Debug)]
pub enum FontError {
    Fontdue(&'static str),
    Texture(glium::texture::TextureCreationError),
}

impl EngineFont {
    pub(crate) fn load_from_bytes(bytes: &[u8]) -> Result<EngineFont, FontError> {
        Self::load_from_bytes_with_atlas(TextureAtlas::new()?, bytes)
    }

    pub(crate) fn load_from_bytes_with_atlas(
        atlas: TextureAtlas,
        bytes: &[u8],
    ) -> Result<Self, FontError> {
        Ok(Self {
            font: fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())?,
            characters: HashMap::new(),
            atlas,
        })
    }

    #[allow(unused)]
    pub(crate) fn descent(&self, font_size: f32) -> f32 {
        self.font
            .horizontal_line_metrics(font_size)
            .unwrap()
            .descent
    }

    fn rasterize_glyph(&self, glyph: Glyph) -> (Metrics, Vec<u8>) {
        self.font.rasterize(glyph.character, glyph.size as f32)
    }

    pub(crate) fn cache_glyph(&mut self, glyph: Glyph) {
        if self.contains(glyph) {
            return;
        }

        let (metrics, bitmap) = self.rasterize_glyph(glyph);
        let sprite = self.atlas.cache_sprite(&Image::new(
            metrics.width,
            metrics.height,
            bitmap
                .iter()
                .map(|coverage| Pixel::from_rgba(255, 255, 255, *coverage))
                .collect(),
        ));
        let advance = metrics.advance_width;
        let offset = (metrics.xmin, metrics.ymin).into();

        let character_info = CharacterInfo {
            advance,
            offset,
            sprite,
        };

        self.characters.insert(glyph, character_info);
    }

    pub(crate) fn contains(&self, glyph: Glyph) -> bool {
        self.characters.contains_key(&glyph)
    }

    pub(crate) fn measure_text(
        &mut self,
        text: impl AsRef<str>,
        font_size: f32,
        do_dpi_scaling: bool,
    ) -> TextDimensions {
        let text = text.as_ref();

        if text.is_empty() {
            return TextDimensions::default();
        }

        let dpi_scaling = if do_dpi_scaling { dpi_scaling() } else { 1.0 };
        let font_size = (font_size * dpi_scaling).ceil();
        let mut layout =
            fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        layout.append(&[&self.font], &TextStyle::new(text, font_size, 0));

        let mut width = 0.0f32;

        for position in layout.glyphs() {
            let glyph = Glyph {
                character: position.parent,
                size: font_size as usize,
            };
            if !self.contains(glyph) {
                self.cache_glyph(glyph);
            }

            let char_info = self.characters[&glyph];

            let glyph_end = position.x + char_info.advance;
            width = width.max(glyph_end);
        }

        let size = Vec2::new(width, layout.height());
        TextDimensions {
            size,
            final_cursor_pos: size,
        }
    }

    pub fn ascii_character_list() -> Vec<char> {
        (0..255).filter_map(::std::char::from_u32).collect()
    }

    pub fn latin_character_list() -> Vec<char> {
        "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*(){}[].,:"
            .chars()
            .collect()
    }

    pub fn populate_font_cache(&mut self, characters: &[char], size: usize) {
        for character in characters {
            self.cache_glyph(Glyph {
                character: *character,
                size,
            });
        }
    }

    pub fn set_minify_filter(&mut self, filter_mode: MinifySamplerFilter) {
        self.atlas.set_minify_filter(filter_mode);
    }

    pub fn set_magnify_filter(&mut self, filter_mode: MagnifySamplerFilter) {
        self.atlas.set_magnify_filter(filter_mode);
    }

    pub fn use_linear_filtering(&mut self) {
        self.atlas.use_linear_filtering();
    }

    pub fn use_nearest_filtering(&mut self) {
        self.atlas.use_nearest_filtering();
    }

    pub fn texture(&mut self) -> TextureRef {
        self.atlas.texture().unwrap()
    }
}

pub fn create_ttf_font(bytes: &[u8]) -> Result<FontRef, FontError> {
    EngineFont::load_from_bytes(bytes).map(|f| f.create())
}

#[derive(Clone, Copy)]
pub struct TextDrawParams {
    pub font: Option<FontRef>,
    pub font_size: usize,
    pub color: Color,
    pub position: Vec2,
    /// scale the font size by the DPI scaling of your monitor
    pub do_dpi_scaling: bool,
}

impl Default for TextDrawParams {
    fn default() -> Self {
        Self {
            font: None,
            font_size: 16,
            color: Color::NEUTRAL_100,
            do_dpi_scaling: false,
            position: Vec2::ZERO,
        }
    }
}

pub const MONO: FontRef = FontRef(0);
pub const SANS: FontRef = FontRef(1);
pub const SANS_DISPLAY: FontRef = FontRef(2);

pub(crate) fn init_fonts() -> Result<(), FontError> {
    load_font(include_bytes!("../../assets/fonts/jetbrains.ttf")).map(|_| ())?;
    load_font(include_bytes!("../../assets/fonts/inter.ttf")).map(|_| ())?;
    load_font(include_bytes!("../../assets/fonts/inter-display-bold.ttf")).map(|_| ())?;

    Ok(())
}

fn draw_text_to(
    text: impl AsRef<str>,
    params: TextDrawParams,
    draw_queue: &mut DrawQueue2D,
) -> TextDimensions {
    let text = text.as_ref();
    let TextDrawParams {
        font,
        font_size,
        color,
        position: pos,
        do_dpi_scaling,
    } = params;

    if text.is_empty() {
        return TextDimensions::default();
    }

    let dpi_scaling = if do_dpi_scaling { dpi_scaling() } else { 1.0 };
    let font_size = (font_size as f32 * dpi_scaling).ceil();
    let mut font = font.unwrap_or(default_font());
    let mut layout = fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
    layout.append(&[&font.font], &TextStyle::new(text, font_size, 0));

    let mut width = 0.0f32;

    for glyph_position in layout.glyphs() {
        let glyph = Glyph {
            character: glyph_position.parent,
            size: font_size as usize,
        };

        if !font.contains(glyph) {
            font.cache_glyph(glyph);
        }

        let char_info = font.characters[&glyph];
        let sprite = font.atlas.get(char_info.sprite).unwrap();
        let rect = sprite.rect;
        let rectf: Rect = rect.into();

        // FIXED: Use character advance to handle spaces correctly
        // This accounts for both visual width AND advance (important for spaces!)
        let glyph_end = glyph_position.x + char_info.advance;
        width = width.max(glyph_end);

        let transform = Transform2D::from_scale_translation(
            Vec2::new(glyph_position.width as f32, glyph_position.height as f32),
            Vec2::new(glyph_position.x + pos.x, glyph_position.y + pos.y),
        );

        draw_queue.add_sprite(font.atlas.texture().unwrap(), transform, color, Some(rectf));
    }

    let size = Vec2::new(width, layout.height());
    TextDimensions {
        size,
        final_cursor_pos: size,
    }
}

pub fn draw_text_ex(text: impl AsRef<str>, params: TextDrawParams) -> TextDimensions {
    draw_text_to(text, params, get_state().draw_queue_2d())
}

pub fn draw_text(text: impl AsRef<str>, position: Vec2) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            ..Default::default()
        },
        get_state().draw_queue_2d(),
    )
}

pub fn draw_text_size(text: impl AsRef<str>, position: Vec2, size: usize) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            font_size: size,
            ..Default::default()
        },
        get_state().draw_queue_2d(),
    )
}

pub fn draw_text_world_ex(text: impl AsRef<str>, params: TextDrawParams) -> TextDimensions {
    draw_text_to(text, params, get_state().world_draw_queue_2d())
}

pub fn draw_text_world(text: impl AsRef<str>, position: Vec2) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            ..Default::default()
        },
        get_state().world_draw_queue_2d(),
    )
}

pub fn draw_text_size_world(text: impl AsRef<str>, position: Vec2, size: usize) -> TextDimensions {
    draw_text_to(
        text,
        TextDrawParams {
            position,
            font_size: size,
            ..Default::default()
        },
        get_state().world_draw_queue_2d(),
    )
}

pub fn load_font(bytes: &[u8]) -> Result<FontRef, FontError> {
    EngineFont::load_from_bytes(bytes).map(|f| f.create())
}

pub fn measure_text(text: impl AsRef<str>) -> TextDimensions {
    measure_text_ex(text, TextDrawParams::default())
}

/// Does not take translatation, rotation or colour into account. Feel free to exclude them.
pub fn measure_text_ex(text: impl AsRef<str>, params: TextDrawParams) -> TextDimensions {
    let TextDrawParams {
        font,
        font_size,
        do_dpi_scaling,
        ..
    } = params;

    font.unwrap_or(default_font())
        .measure_text(text, font_size as f32, do_dpi_scaling)
}

fn draw_multiline_text_to(
    text: impl AsRef<str>,
    params: TextDrawParams,
    line_spacing: f32,
    draw_queue: &mut DrawQueue2D,
) -> TextDimensions {
    let text = text.as_ref();
    let lines: Vec<&str> = text.lines().collect();

    if lines.is_empty() {
        return TextDimensions::default();
    }

    let mut max_width = 0.0f32;
    let mut current_y = params.position.y;
    let mut current_x = 0.0;
    let line_height = params.font_size as f32 * line_spacing;

    for line in &lines {
        if line.is_empty() {
            current_y += line_height;
            current_x = 0.0;
            continue;
        }

        let dims = draw_text_to(
            line,
            TextDrawParams {
                position: Vec2::new(params.position.x, current_y),
                ..params
            },
            draw_queue,
        );
        current_x = dims.size.x;
        max_width = max_width.max(dims.size.x);
        current_y += line_height;
    }

    let total_height = line_height * (lines.len() as f32);

    // Check if text ends with a newline - if so, cursor should be at start of next line
    let ends_with_newline = text.ends_with('\n') || text.ends_with("\r\n");
    let final_x = if ends_with_newline {
        params.position.x
    } else {
        current_x + params.position.x
    };
    let final_y = if ends_with_newline {
        params.position.y + total_height
    } else {
        params.position.y + total_height - line_height
    };

    TextDimensions {
        size: Vec2::new(max_width, total_height),
        final_cursor_pos: Vec2::new(final_x, final_y),
    }
}

pub fn draw_multiline_text(
    text: impl AsRef<str>,
    position: Vec2,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(
        text,
        TextDrawParams {
            position,
            ..Default::default()
        },
        line_spacing,
        get_state().draw_queue_2d(),
    )
}

pub fn draw_multiline_text_size(
    text: impl AsRef<str>,
    position: Vec2,
    size: usize,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(
        text,
        TextDrawParams {
            position,
            font_size: size,
            ..Default::default()
        },
        line_spacing,
        get_state().draw_queue_2d(),
    )
}

pub fn draw_multiline_text_ex(
    text: impl AsRef<str>,
    params: TextDrawParams,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(text, params, line_spacing, get_state().draw_queue_2d())
}

pub fn draw_multiline_text_world(
    text: impl AsRef<str>,
    position: Vec2,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(
        text,
        TextDrawParams {
            position,
            ..Default::default()
        },
        line_spacing,
        get_state().world_draw_queue_2d(),
    )
}

pub fn draw_multiline_text_size_world(
    text: impl AsRef<str>,
    position: Vec2,
    size: usize,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(
        text,
        TextDrawParams {
            position,
            font_size: size,
            ..Default::default()
        },
        line_spacing,
        get_state().world_draw_queue_2d(),
    )
}

pub fn draw_multiline_text_world_ex(
    text: impl AsRef<str>,
    params: TextDrawParams,
    line_spacing: f32,
) -> TextDimensions {
    draw_multiline_text_to(
        text,
        params,
        line_spacing,
        get_state().world_draw_queue_2d(),
    )
}

pub fn measure_multiline_text(text: impl AsRef<str>, line_spacing: f32) -> TextDimensions {
    measure_multiline_text_ex(text, TextDrawParams::default(), line_spacing)
}

pub fn measure_multiline_text_ex(
    text: impl AsRef<str>,
    params: TextDrawParams,
    line_spacing: f32,
) -> TextDimensions {
    let text = text.as_ref();
    let lines: Vec<&str> = text.lines().collect();

    if lines.is_empty() {
        return TextDimensions::default();
    }

    let mut max_width = 0.0f32;
    let line_height = params.font_size as f32 * line_spacing;
    let mut current_x = 0.0;

    for line in &lines {
        let dims = measure_text_ex(line, params);
        current_x = dims.size.x;
        max_width = max_width.max(dims.size.x);
    }

    let total_height = line_height * (lines.len() as f32);

    TextDimensions {
        size: Vec2::new(max_width, total_height),
        final_cursor_pos: Vec2::new(current_x, total_height),
    }
}
