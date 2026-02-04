use engine_4::prelude::*;
use engine_4::ui::*;

fn main() -> anyhow::Result<()> {
    init("UI")?;
    let texture = include_texture!("../assets/textures/guy.jpg");
    let mut progress: f32 = rand();

    loop {
        clear_screen(Color::hex(0xC0C0C0));

        let ui = SizedBox::wh(
            400.0,
            800.0,
            BoxFill::new(
                Color::GRAY_700,
                Scroll::new(
                    id!(),
                    Padding::all(
                        10.0,
                        Col::with_gap(
                            10.0,
                            (0..100)
                                .map(|n| {
                                    SizedBox::height(
                                        40.0,
                                        RoundedHoverFill::new(
                                            Color::GRAY_600,
                                            Color::GRAY_500,
                                            7.0,
                                            Center::new(Text::new(n)),
                                        ),
                                    )
                                })
                                .collect::<Vec<_>>(),
                        ),
                    ),
                ),
            ),
        );
        draw_ui(ui, vec2(20.0, 500.0));

        // ------

        let show_blinking = time_seconds().is_multiple_of(2);

        if once_per_second() {
            progress = rand();
        }

        let ui = Fit::new(card(
            20.0,
            Col::with_gap(
                10.0,
                [
                    black_text(format!("{:.0}", avg_fps())),
                    black_text("Hello world!"),
                    black_text("This is UI."),
                    Text::new_with_size_color("Styled text", 30, Color::RED_600),
                    ConstrainedBox::max_size(
                        vec2(300.0, 200.0),
                        card(0.0, ImageNode::from_texture(texture)),
                    ),
                    SizedBox::wh(
                        200.0,
                        20.0,
                        Border::all(
                            4.0,
                            Color::GRAY_500,
                            ProgressBar::new(progress, 1.0, Color::GRAY_500, id!()),
                        ),
                    ),
                    if show_blinking {
                        black_text("I blink")
                    } else {
                        EMPTY
                    },
                ],
            ),
        ));

        draw_ui(ui, Vec2::splat(20.0));

        // ------

        let ui = SizedBox::wh(
            500.0,
            500.0,
            BoxFill::new(
                Color::NEUTRAL_800,
                Padding::all(
                    20.0,
                    Scroll::new(
                        id!(),
                        Col::new([
                            Text::title("Title"),
                            Text::body("Lorem ipsum dolor sit amet."),
                            Text::h1("Heading 1"),
                            Text::body("Lorem ipsum dolor sit amet."),
                            Text::h2("Heading 1"),
                            Text::body("Lorem ipsum dolor sit amet."),
                            Text::h3("Heading 1"),
                            Text::body("Lorem ipsum dolor sit amet."),
                        ]),
                    ),
                ),
            ),
        );

        draw_ui(ui, vec2(500.0, 20.0));

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

// example simple widget. if you want to make something more complicated check out the UiNode trait
// if you want to create a widget with retained mutable state, check out the source for Scroll
fn black_text(text: impl ToString) -> UiRef {
    Text::new_with_color(text, Color::BLACK)
}

// slightly more complex wrapper widget.
fn card(padding: f32, child: UiRef) -> UiRef {
    Border::tblr(
        BorderStyle::new(10.0, Color::hex(0xFCFCFC)),
        BorderStyle::new(10.0, Color::hex(0x08080E)),
        BorderStyle::new(10.0, Color::hex(0xFCFCFC)),
        BorderStyle::new(10.0, Color::hex(0x08080E)),
        BoxFill::new(Color::hex(0xC0C0C0), Padding::all(padding, child)),
    )
}
