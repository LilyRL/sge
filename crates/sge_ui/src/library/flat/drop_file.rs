use std::path::PathBuf;

use sge_vectors::Vec2;

use crate::{
    UiRef,
    base::{Border, BorderStyle, BorderType, BoxFill, Center, Text},
};

pub struct DropFile;

impl DropFile {
    pub fn new(dimensions: Vec2, id: usize) -> UiRef {
        fn base() -> UiRef {
            Border::all_style(
                BorderStyle::custom(10.0, super::BG2, BorderType::Dashed(40.0)),
                Center::new(Text::new("Drop file here")),
            )
        }

        fn hovered(_: PathBuf) -> UiRef {
            BoxFill::new(
                super::BG1,
                Border::all_style(
                    BorderStyle::custom(10.0, super::BG3, BorderType::Dashed(40.0)),
                    Center::new(Text::new("Release to drop")),
                ),
            )
        }

        fn dropped(_: PathBuf) -> UiRef {
            base()
        }

        crate::base::DropFile::new(dimensions, id, base, hovered, dropped)
    }
}
