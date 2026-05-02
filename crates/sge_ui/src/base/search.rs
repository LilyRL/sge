use super::*;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug)]
pub struct Search {
    input: State<InputState>,
    gap: f32,
    options: Vec<(String, Child)>,
    state: State<SearchState>,
}

#[derive(Debug, Default)]
pub struct SearchState {
    pub scores: Vec<usize>,
}

impl Search {
    pub fn new(id: usize, input_id: usize, options: Vec<(String, Child)>, gap: f32) -> UiRef {
        Self {
            input: State::from_id(input_id),
            options,
            gap,
            state: State::from_id(id),
        }
        .to_ref()
    }

    pub fn seperate(
        id: usize,
        input_id: usize,
        text: Vec<String>,
        display: Vec<Child>,
        gap: f32,
    ) -> UiRef {
        let options = text.into_iter().zip(display.into_iter()).collect();
        Self::new(id, input_id, options, gap)
    }
}

impl UiNode for Search {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::INFINITY
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let input = self.input.get_or_default();
        let state = self.state.get_or_default();

        if input.changed || state.scores.is_empty() {
            let matcher = SkimMatcherV2::default();
            state.scores = self
                .options
                .iter()
                .map(|(text, _)| matcher.fuzzy_match(text, &input.value).unwrap_or(0) as usize)
                .collect();
        }

        let mut options = self
            .options
            .iter()
            .zip(state.scores.iter())
            .filter(|(_, s)| **s > 0 || input.value.len() < 3)
            .collect::<Vec<_>>();
        options.sort_by_key(|(_, score)| std::cmp::Reverse(*score));

        Scroll::new(
            self.state.to_id() ^ id!(),
            Col::with_gap(
                self.gap,
                options
                    .into_iter()
                    .map(|((_, child), _)| child.clone())
                    .collect::<Vec<_>>(),
            ),
        )
        .draw(area, ui)
    }
}
