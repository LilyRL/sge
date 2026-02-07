use engine_4::prelude::*;
use engine_4::ui::prelude::*;

fn main() -> anyhow::Result<()> {
    init("UI")?;
    let texture = include_texture!("../assets/textures/guy.jpg");
    let mut progress: f32 = rand();
    let mut show_message = false;
    let mut clear_color = w95::PRIMARY;

    loop {
        clear_screen(clear_color);

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
        draw_ui(ui, vec2(20.0, 600.0));

        // ------

        let show_blinking = time_seconds().is_multiple_of(2);

        if once_per_n_seconds(2.0 / 3.0) {
            progress = rand();
        }

        let button_id = id!();
        let ui = Fit::new(w95::Card::new(
            20.0,
            Col::with_gap(
                10.0,
                [
                    black_text(format!("{:.0}", avg_fps())),
                    black_text("Hello world!"),
                    black_text("This is UI."),
                    w95::Button::text(button_id, "Click me!"),
                    Text::new_with_size_color("Styled text", 30, Color::RED_600),
                    ConstrainedBox::max_size(
                        vec2(300.0, 200.0),
                        w95::Card::new(0.0, ImageNode::from_texture(texture)),
                    ),
                    w95::ProgressBar::new(vec2(200.0, 20.0), progress, 1.0, id!()),
                    if show_blinking {
                        black_text("I blink")
                    } else {
                        EMPTY
                    },
                ],
            ),
        ));

        draw_ui(ui, Vec2::splat(20.0));

        if ui_button_clicked(button_id) {
            clear_color = random_color();
        }

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
                            Text::italic("Lorem ipsum dolor sit amet."),
                            Text::bold("Lorem ipsum dolor sit amet."),
                            Text::bold_italic("Lorem ipsum dolor sit amet."),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::WHITE),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_100),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_200),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_300),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_400),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_500),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_600),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_700),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_800),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::NEUTRAL_900),
                            bold_italic_colored("Lorem ipsum dolor sit amet.", Color::BLACK),
                        ]),
                    ),
                ),
            ),
        );

        draw_ui(ui, vec2(500.0, 20.0));

        // ------

        let button_id = id!();
        let ui = library::flat::Card::sized(
            vec2(300.0, 200.0),
            Color::NEUTRAL_900,
            Col::with_gap(
                30.0,
                [
                    Center::new(flat::Button::text(
                        Color::NEUTRAL_600,
                        Color::NEUTRAL_500,
                        button_id,
                        "Click me!",
                    )),
                    if show_message {
                        Center::new(Text::body("Thanks for clicking."))
                    } else {
                        EMPTY
                    },
                ],
            ),
        );

        draw_ui(ui, vec2(500.0, 540.0));

        if ui_button_clicked(button_id) {
            show_message = !show_message;
        }

        // ------

        let size = vec2(300.0, 300.0);
        let ui = SizedBox::new(
            size,
            BoxFill::new(
                Color::BLACK,
                Stack::new(
                    size,
                    [
                        Align::top_left(square(Color::RED_500)),
                        Align::top_center(square(Color::ORANGE_500)),
                        Align::top_right(square(Color::YELLOW_500)),
                        Align::center_right(square(Color::GREEN_500)),
                        Align::bottom_right(square(Color::SKY_500)),
                        Align::bottom_center(square(Color::BLUE_500)),
                        Align::bottom_left(square(Color::PURPLE_500)),
                        Align::center_left(square(Color::PINK_500)),
                        Align::center(square(Color::WHITE)),
                    ],
                ),
            ),
        );

        draw_ui(ui, vec2(500.0, 760.0));

        // ------

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

fn square(color: Color) -> UiRef {
    BoxFill::new(color, EmptyBox::new(50.0, 50.0))
}

// example simple widget. if you want to make something more complicated check out the UiNode trait
// if you want to create a widget with retained mutable state, check out the source for ProgressBar
fn black_text(text: impl ToString) -> UiRef {
    Text::new_with_color(text, Color::BLACK)
}

fn bold_italic_colored(text: impl ToString, color: Color) -> UiRef {
    Text::new_full(text, SANS_BOLD_ITALIC, 24, color, true)
}
