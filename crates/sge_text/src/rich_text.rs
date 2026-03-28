use bevy_math::Vec2;
use sge_color::{Color, str_to_color};
use sge_rendering::{d2::Renderer2D, dq2d, wdq2d};
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
    UnknownColor(String),
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
}

impl RichTextParser {
    fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect::<Vec<char>>(),
            i: 0,
            blocks: vec![],
            color: Color::NEUTRAL_100,
        }
    }

    fn parse(mut self) -> Result<RichText, RichTextParseError> {
        while let Some(c) = self.next() {
            if c == '{' {
                self.next();
                let color_string = self.consume_until('}')?;
                self.consume('}')?;
                self.color = str_to_color(&color_string)
                    .ok_or(RichTextParseError::UnknownColor(color_string))?;
                // self.consume_or_not(' ');
            } else {
                let text = self.consume_until('{')?;
                self.blocks.push(RichTextBlock {
                    color: self.color,
                    text,
                });
            }
        }

        Ok(RichText {
            blocks: self.blocks,
        })
    }

    // fn consume_or_not(&mut self, token: char) {
    //     if self.peek() == Some(token) {
    //         self.next();
    //     }
    // }

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

/// parses from a format of colors between curly braces, which give their color to all text
/// that comes after it, overridden with another color block.
/// format example:
///
/// ```text
/// if you provide no color block, it defaults to white
/// {RED_500}red text
/// {red5}this would give the same color, as it ignores underscores and zeros, and case
/// {#f00}another red
/// {#f00a}semi-transparent red
/// {#ff0000}another another red
/// {#ff0000aa}another semi-transparent red
/// {rgb(10, 250, 10)}green
/// {rgb(0.1, 0.9, 0.1)} a single space between the color block and the text is ignored
/// {rgb 10 250 10} this is also valid
/// {hsl 180 100 200} hsl is supported
/// {oklch 180 100 200} oklch is supported
/// {rgb 0.5 230 0.9} you can mix and match
/// {rgb 1 1 1} this will get parsed as almost black, because it prioritizes the (255, 255, 255)
/// format, so you should write {rgb 1.0 1.0 1.0} instead
/// ```
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
    /// scale the font size by the DPI scaling of your monitor
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

    /// parses from a format of colors between curly braces, which give their color to all text
    /// that comes after it, overridden with another color block.
    /// format example:
    ///
    /// ```text
    /// if you provide no color block, it defaults to white
    /// {RED_500}red text
    /// {red5}this would give the same color, as it ignores underscores and zeros, and case
    /// {#f00}another red
    /// {#f00a}semi-transparent red
    /// {#ff0000}another another red
    /// {#ff0000aa}another semi-transparent red
    /// {rgb(10, 250, 10)}green
    /// {rgb(0.1, 0.9, 0.1)} a single space between the color block and the text is ignored
    /// {rgb 10 250 10} this is also valid
    /// {hsl 180 100 200} hsl is supported
    /// {oklch 180 100 200} oklch is supported
    /// {rgb 0.5 230 0.9} you can mix and match
    /// {rgb 1 1 1} this will get parsed as almost black, because it prioritizes the (255, 255, 255)
    /// format, so you should write {rgb 1.0 1.0 1.0} instead
    /// ```
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

                cursor.x += dimensions.size.x;
                max_width = max_width.max(cursor.x - params.position.x);

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
}

#[derive(Debug, Clone)]
pub struct RichTextBlock {
    pub color: Color,
    pub text: String,
}

impl RichTextBlock {
    pub fn new(text: String, color: Color) -> Self {
        Self { color, text }
    }

    pub fn from_str<S: AsRef<str>>(s: S, color: Color) -> Self {
        Self {
            color,
            text: s.as_ref().to_string(),
        }
    }
}
