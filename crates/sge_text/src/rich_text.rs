use crate::{MONO, wrapped_text::get_space_width};
use sge_api::shapes_2d::draw_line_to;
use sge_color::{Color, str_to_color};
use sge_rendering::{d2::Renderer2D, dq2d, wdq2d};
use sge_types::Area;
use sge_vectors::Vec2;
use thiserror::Error;

use crate::draw_text_to;

use super::{FontRef, TextDimensions, TextDrawParams};

#[derive(Debug, Clone)]
pub struct RichText {
    pub blocks: Vec<RichTextBlock>,
}

#[derive(Error, Debug)]
pub enum RichTextParseError {
    #[error("Could not parse color: {0}")]
    UnknownFormat(String),
    #[error("Expected token: `{0}` at character: {1}.")]
    ExpectedToken(char, usize),
    #[error("Unexpected token: `{0}`, expected: `{1}` at character: {2}.")]
    UnexpectedToken(char, char, usize),
}

struct RichTextParser {
    chars: Vec<char>,
    i: usize,
    blocks: Vec<RichTextBlock>,
    color: Color,
    underline: bool,
    strikethrough: bool,
}

impl RichTextParser {
    fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect::<Vec<char>>(),
            i: 0,
            blocks: vec![],
            color: Color::NEUTRAL_100,
            underline: false,
            strikethrough: false,
        }
    }

    fn parse(mut self) -> Result<RichText, RichTextParseError> {
        while let Some(c) = self.next() {
            if c == '{' {
                self.next();
                let format_string = self.consume_until('}')?;
                self.consume('}')?;
                match str_to_color(&format_string) {
                    Some(c) => self.color = c,
                    None => {
                        match format_string
                            .replace("_", "")
                            .replace("-", "")
                            .to_lowercase()
                            .trim()
                        {
                            "underline" | "ul" => self.underline = true,
                            "nounderline" | "noul" => self.underline = false,
                            "strikethrough" | "st" => self.strikethrough = true,
                            "nostrikethrough" | "nost" => self.strikethrough = false,
                            "r" | "reset" => {
                                self.underline = false;
                                self.strikethrough = false;
                                self.color = Color::NEUTRAL_100;
                            }
                            _ => return Err(RichTextParseError::UnknownFormat(format_string)),
                        }
                    }
                }
            } else {
                let mut text = String::new();
                text.push(c);
                loop {
                    if self.peek() == Some('{') || self.peek().is_none() {
                        break;
                    }
                    if let Some(next_c) = self.next() {
                        text.push(next_c);
                    }
                }
                self.blocks.push(RichTextBlock {
                    color: self.color,
                    underline: self.underline,
                    strikethrough: self.strikethrough,
                    text,
                });
            }
        }

        Ok(RichText {
            blocks: self.blocks,
        })
    }

    fn consume(&mut self, token: char) -> Result<(), RichTextParseError> {
        if let Some(c) = self.next() {
            if c == token {
                Ok(())
            } else {
                Err(RichTextParseError::UnexpectedToken(c, token, self.i))
            }
        } else {
            Err(RichTextParseError::ExpectedToken(token, self.i))
        }
    }

    fn consume_until(&mut self, token: char) -> Result<String, RichTextParseError> {
        let mut string = String::new();
        string.push(self.current());

        loop {
            if self.peek() == Some(token) {
                return Ok(string);
            }

            match self.next() {
                Some(c) => string.push(c),
                None => return Ok(string),
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.i).copied()
    }

    fn current(&self) -> char {
        self.chars[self.i - 1]
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars.get(self.i);
        self.i += 1;
        c.copied()
    }
}

pub fn rich_text(input: impl AsRef<str>) -> Result<RichText, RichTextParseError> {
    RichText::parse(input)
}

pub fn rich_text_blocks(blocks: Vec<RichTextBlock>) -> RichText {
    RichText::new(blocks)
}

#[derive(Clone, Copy)]
pub struct RichTextDrawParams {
    pub font: Option<FontRef>,
    pub font_size: usize,
    pub position: Vec2,

    pub do_dpi_scaling: bool,
    pub line_spacing: f32,
}

impl Default for RichTextDrawParams {
    fn default() -> Self {
        Self {
            font: None,
            font_size: 16,
            position: Vec2::ZERO,
            do_dpi_scaling: true,
            line_spacing: 1.5,
        }
    }
}

impl RichTextDrawParams {
    pub fn to_text_params(&self, color: Color) -> TextDrawParams {
        TextDrawParams {
            font: self.font,
            font_size: self.font_size,
            color,
            position: self.position,
            do_dpi_scaling: self.do_dpi_scaling,
        }
    }
}

impl RichText {
    pub fn new(blocks: Vec<RichTextBlock>) -> Self {
        Self { blocks }
    }

    pub fn parse(input: impl AsRef<str>) -> Result<RichText, RichTextParseError> {
        RichTextParser::new(input.as_ref()).parse()
    }

    fn draw_to(&self, params: RichTextDrawParams, renderer: Renderer2D) -> TextDimensions {
        let left_edge = params.position.x;
        let mut cursor = params.position;
        let mut max_width = 0.0f32;
        let line_height = params.font_size as f32 * params.line_spacing;

        for block in &self.blocks {
            let lines: Vec<&str> = block.text.lines().collect();

            if lines.is_empty() {
                let newline_count = block.text.matches('\n').count();
                if newline_count > 0 {
                    cursor.y += line_height * newline_count as f32;
                    cursor.x = left_edge;
                }
                continue;
            }

            for (i, line) in lines.iter().enumerate() {
                let mut text_params = params.to_text_params(block.color);
                text_params.position = cursor;

                let dimensions = draw_text_to(line, text_params, renderer);

                let cursor_x_before = cursor.x;
                cursor.x += dimensions.size.x;
                max_width = max_width.max(cursor.x - params.position.x);

                if block.strikethrough {
                    let strike_y = cursor.y + line_height / 2.0;
                    draw_line_to(
                        Vec2::new(cursor_x_before, strike_y),
                        Vec2::new(cursor.x, strike_y),
                        2.0,
                        block.color,
                        renderer,
                    );
                }

                if block.underline {
                    let underline_y = cursor.y + line_height;
                    draw_line_to(
                        Vec2::new(cursor_x_before, underline_y),
                        Vec2::new(cursor.x, underline_y),
                        2.0,
                        block.color,
                        renderer,
                    );
                }

                if i < lines.len() - 1 {
                    cursor.y += line_height;
                    cursor.x = left_edge;
                }
            }

            if block.text.ends_with('\n') || block.text.ends_with("\r\n") {
                cursor.y += line_height;
                cursor.x = left_edge;
            }
        }

        TextDimensions {
            size: Vec2::new(max_width, cursor.y - params.position.y + line_height),
            final_cursor_pos: cursor,
        }
    }

    pub fn draw(&self, params: RichTextDrawParams) -> TextDimensions {
        self.draw_to(params, dq2d())
    }

    pub fn draw_world(&self, params: RichTextDrawParams) -> TextDimensions {
        self.draw_to(params, wdq2d())
    }

    pub fn print_to_stdout(&self) {
        use std::io::Write;
        use termcolor::{Color as TermColor, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        for block in &self.blocks {
            let pixel = block.color.to_pixel();
            let term_color = TermColor::Rgb(pixel.r(), pixel.g(), pixel.b());
            stdout
                .set_color(ColorSpec::new().set_fg(Some(term_color)))
                .unwrap();
            write!(&mut stdout, "{}", &block.text).unwrap();
        }

        writeln!(&mut stdout).unwrap();
    }

    pub fn draw_in_area(&self, area: Area, params: RichTextDrawParams) -> TextDimensions {
        self.draw_in_area_to(area, params, dq2d())
    }

    pub fn draw_in_area_world(&self, area: Area, params: RichTextDrawParams) -> TextDimensions {
        self.draw_in_area_to(area, params, wdq2d())
    }

    fn draw_in_area_to(
        &self,
        area: Area,
        params: RichTextDrawParams,
        renderer: Renderer2D,
    ) -> TextDimensions {
        use sge_window::dpi_scaling;

        let max_width = area.width();
        let line_height = params.font_size as f32 * params.line_spacing;
        let left_edge = area.top_left().x;

        let dpi_scaling_factor = if params.do_dpi_scaling {
            dpi_scaling()
        } else {
            1.0
        };
        let font_size_scaled = (params.font_size as f32 * dpi_scaling_factor).ceil();

        let font_ref = params.font.unwrap_or(MONO);
        let space_width = get_space_width(font_ref.get_mut(), font_size_scaled as usize);

        #[derive(Clone)]
        struct StyledWord {
            text: String,
            color: sge_color::Color,
            underline: bool,
            strikethrough: bool,
            width: f32,
            is_newline: bool,
        }

        let mut styled_words: Vec<StyledWord> = Vec::new();

        for block in &self.blocks {
            let paragraphs: Vec<&str> = block.text.split('\n').collect();
            for (pi, paragraph) in paragraphs.iter().enumerate() {
                if pi > 0 {
                    styled_words.push(StyledWord {
                        text: String::new(),
                        color: block.color,
                        underline: block.underline,
                        strikethrough: block.strikethrough,
                        width: 0.0,
                        is_newline: true,
                    });
                }

                let mut remaining = *paragraph;
                while !remaining.is_empty() {
                    let (chunk, rest) = match remaining.find(' ') {
                        Some(pos) => (&remaining[..pos + 1], &remaining[pos + 1..]),
                        None => (remaining, ""),
                    };
                    remaining = rest;

                    let mut layout = fontdue::layout::Layout::new(
                        fontdue::layout::CoordinateSystem::PositiveYDown,
                    );
                    layout.append(
                        &[&font_ref.get_mut().font],
                        &fontdue::layout::TextStyle::new(chunk, font_size_scaled, 0),
                    );
                    let mut word_width = 0.0f32;
                    for gp in layout.glyphs() {
                        let glyph = crate::Glyph {
                            character: gp.parent,
                            size: font_size_scaled as usize,
                        };
                        if !font_ref.get_mut().contains(glyph) {
                            font_ref.get_mut().cache_glyph(glyph);
                        }
                        let char_info = &font_ref.get_mut().characters[&glyph];
                        word_width = word_width.max(gp.x + char_info.advance);
                    }

                    styled_words.push(StyledWord {
                        text: chunk.to_string(),
                        color: block.color,
                        underline: block.underline,
                        strikethrough: block.strikethrough,
                        width: word_width,
                        is_newline: false,
                    });
                }
            }
        }

        let mut lines: Vec<Vec<StyledWord>> = Vec::new();
        let mut current_line: Vec<StyledWord> = Vec::new();
        let mut current_width = 0.0f32;

        for word in styled_words {
            if word.is_newline {
                lines.push(std::mem::take(&mut current_line));
                current_width = 0.0;
                continue;
            }

            if current_line.is_empty() || current_width + word.width <= max_width {
                current_width += word.width;
                current_line.push(word);
            } else {
                lines.push(std::mem::take(&mut current_line));
                let mut w = word;
                if w.text.starts_with(' ') {
                    w.text.remove(0);
                    w.width -= space_width;
                }
                current_width = w.width;
                current_line.push(w);
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let mut cursor = area.top_left() + Vec2::new(0.0, params.position.y);
        cursor.x = left_edge;
        let start_y = cursor.y;
        let mut max_actual_width = 0.0f32;

        for line_words in &lines {
            cursor.x = left_edge;
            for word in line_words {
                let text_params = TextDrawParams {
                    font: params.font,
                    font_size: params.font_size,
                    color: word.color,
                    position: cursor,
                    do_dpi_scaling: params.do_dpi_scaling,
                };
                let dims = draw_text_to(&word.text, text_params, renderer);
                cursor.x += dims.size.x;
                max_actual_width = max_actual_width.max(cursor.x - left_edge);

                if word.strikethrough {
                    let strike_y = cursor.y + line_height / 2.0;
                    draw_line_to(
                        Vec2::new(cursor.x - dims.size.x, strike_y),
                        Vec2::new(cursor.x, strike_y),
                        2.0,
                        word.color,
                        renderer,
                    );
                }
                if word.underline {
                    let underline_y = cursor.y + line_height;
                    draw_line_to(
                        Vec2::new(cursor.x - dims.size.x, underline_y),
                        Vec2::new(cursor.x, underline_y),
                        2.0,
                        word.color,
                        renderer,
                    );
                }
            }
            cursor.y += line_height;
        }

        let total_height = cursor.y - start_y;
        TextDimensions {
            size: Vec2::new(max_actual_width, total_height),
            final_cursor_pos: cursor,
        }
    }

    pub fn measure(&self, params: RichTextDrawParams) -> TextDimensions {
        let mut text = String::new();
        for block in &self.blocks {
            text.push_str(&block.text);
        }
        let params = TextDrawParams {
            font: params.font,
            font_size: params.font_size,
            color: Color::WHITE,
            position: Vec2::ZERO,
            do_dpi_scaling: params.do_dpi_scaling,
        };
        crate::measure_text_ex(&text, params)
    }

    pub fn measure_in_area(&self, area: Area, params: RichTextDrawParams) -> Vec2 {
        let mut text = String::new();
        for block in &self.blocks {
            text.push_str(&block.text);
        }

        crate::measure_wrapped_text(
            &text,
            area.width(),
            params.font,
            params.font_size,
            params.do_dpi_scaling,
            params.line_spacing,
        )
    }
}

#[derive(Debug, Clone)]
pub struct RichTextBlock {
    pub color: Color,
    pub text: String,
    pub underline: bool,
    pub strikethrough: bool,
}

impl RichTextBlock {
    pub fn new(text: String, color: Color) -> Self {
        Self {
            color,
            text,
            underline: false,
            strikethrough: false,
        }
    }

    pub fn new_underline(text: String, color: Color) -> Self {
        Self {
            color,
            text,
            underline: true,
            strikethrough: false,
        }
    }

    pub fn new_strikethrough(text: String, color: Color) -> Self {
        Self {
            color,
            text,
            underline: false,
            strikethrough: true,
        }
    }

    pub fn custom(text: String, color: Color, underline: bool, strikethrough: bool) -> Self {
        Self {
            color,
            text,
            underline,
            strikethrough,
        }
    }

    pub fn from_str<S: AsRef<str>>(s: S, color: Color) -> Self {
        Self {
            color,
            text: s.as_ref().to_string(),
            underline: false,
            strikethrough: false,
        }
    }
}
