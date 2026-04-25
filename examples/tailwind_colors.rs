use sge::prelude::*;
use ui::prelude::*;

#[main("Tailwind colors")]
fn main() -> anyhow::Result<()> {
    wait_for_events();

    let mut hovered =
        AnimationController::new(Color::BLACK, Color::BLACK, 0.05, LinearEasingFunction);

    loop {
        clear_screen(hovered.value());

        let palettes = Palette::PALETTES;
        let width = palettes.len().isqrt();
        let height = (palettes.len() as f32 / width as f32).ceil() as usize;

        let base = id!();

        let ui = AspectRatio::new(
            width as f32 / height as f32 * 4.0 / 3.0,
            Grid::with_gap(
                height,
                width,
                10.0 * dpi_scaling(),
                palettes
                    .iter()
                    .enumerate()
                    .map(|(pi, p)| {
                        Grid::new(
                            3,
                            4,
                            p.shades()
                                .iter()
                                .enumerate()
                                .map(|(ci, &c)| {
                                    Hoverable::new(
                                        hover_id_encode(base, pi, ci),
                                        Fill::new(c, EMPTY),
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>(),
            ),
        )
        .center()
        .padding(min_window_dimension() / 10.0);

        draw_ui(ui, Vec2::ZERO);

        for &element in all_elements_interacted_this_frame() {
            if (element ^ base) < (1 << 10) {
                let (pi, ci) = hover_id_decode(base, element);
                let color = palettes[pi].shades()[ci];
                hovered.now_animate_towards(color);
            }
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

// yeah not ideal. i'm working on making this kind of thing easier to do with the ui
//
// for now we're stuck with id based state

fn hover_id_encode(base: usize, p: usize, c: usize) -> usize {
    base ^ (p << 5 | c)
}

fn hover_id_decode(base: usize, id: usize) -> (usize, usize) {
    let both = base ^ id;
    let p = both >> 5;
    let c = both & 0b11111;
    (p, c)
}
