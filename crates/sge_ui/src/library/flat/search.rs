use crate::base::{BoxFill, Col, Fit, Padding, Text};
use crate::{UiRef, id};

pub struct Search;

impl Search {
    pub fn new(id: usize, options: impl Into<Vec<(String, UiRef)>>) -> UiRef {
        let input_id = id!() ^ id;

        let options = options
            .into()
            .into_iter()
            .map(|(s, c)| {
                (
                    s,
                    Fit::vertical(BoxFill::new(super::BG2, Padding::all(20.0, c))),
                )
            })
            .collect();

        Col::with_gap(
            10.0,
            [
                super::TextInput::new(super::BG2, input_id).width(400.0),
                crate::base::Search::new(id, input_id, options, 10.0).sized_wh(400.0, 600.0),
            ],
        )
        .padding(10.0)
        .fill(super::BG1)
        .sized_wh(420.0, 670.0)
    }

    pub fn text<T, V>(id: usize, options: V) -> UiRef
    where
        T: ToString,
        V: Into<Vec<T>>,
    {
        let input_id = id!() ^ id;

        let options = options
            .into()
            .into_iter()
            .map(|s| {
                (
                    s.to_string(),
                    Fit::vertical(BoxFill::new(
                        super::BG2,
                        Padding::tblr(10.0, 15.0, 10.0, 10.0, Text::nowrap(s)),
                    )),
                )
            })
            .collect();

        Col::with_gap(
            10.0,
            [
                super::TextInput::new(super::BG2, input_id).width(400.0),
                crate::base::Search::new(id, input_id, options, 10.0).sized_wh(400.0, 600.0),
            ],
        )
        .padding(10.0)
        .fill(super::BG1)
        .sized_wh(420.0, 670.0)
    }
}
