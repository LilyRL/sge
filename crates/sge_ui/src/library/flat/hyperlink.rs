use sge_color::Color;
use sge_text::rich_text::{RichText, RichTextBlock};

use crate::{
    UiRef,
    base::{self, RichTextNode},
};

pub struct Hyperlink;

impl Hyperlink {
    pub fn new(href: impl ToString) -> UiRef {
        let rich_text = RichTextNode::new(RichText::new(vec![RichTextBlock::custom(
            href.to_string(),
            Color::SKY_500,
            true,
            false,
        )]));

        base::Hyperlink::new(href, rich_text)
    }

    pub fn with_title(href: impl ToString, title: impl ToString) -> UiRef {
        let rich_text = RichTextNode::new(RichText::new(vec![RichTextBlock::custom(
            title.to_string(),
            Color::SKY_500,
            true,
            false,
        )]));

        base::Hyperlink::new(href, rich_text)
    }
}
