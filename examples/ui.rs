use icons::*;
use sge::prelude::*;
use sge_config::set_wireframe_line_width;
use ui::prelude::*;

fn main() -> anyhow::Result<()> {
    let opts = Opts::builder()
        .swap_interval(SwapInterval::DontWait)
        .title("UI".to_string())
        .build();
    init_custom(opts)?;

    // wait_for_events(); // can be useful for increasing performance in ui only apps

    let texture = include_texture!("../assets/textures/guy.jpg");
    let mut progress: f32 = rand();
    let mut show_message = false;
    let mut clear_color = w95::PRIMARY;
    let mut show_debug_info = false;

    loop {
        clear_screen(clear_color);

        if key_pressed(KeyCode::KeyD) && held_control() {
            toggle_wireframe();
        } else if key_pressed(KeyCode::KeyD) {
            show_debug_info = !show_debug_info;
        }

        if once_per_n_seconds(2.0 / 3.0) {
            progress = rand();
        }

        let mut ui_parts = vec![];

        ui_parts.push(scroll_window());
        ui_parts.push(w95_window(texture, progress, &mut clear_color));
        ui_parts.push(text_window());
        ui_parts.push(button_window(&mut show_message));
        ui_parts.push(align_window());
        ui_parts.push(flat_window());

        let padding = media_query(5.0, 10.0, 20.0);
        let ui = SizedBox::new(
            window_size(),
            Padding::all(padding, Grid::with_gap(2, 3, padding, ui_parts)),
        );
        draw_ui(ui, vec2(0.0, 0.0));

        if show_debug_info {
            run_egui(|ui| draw_debug_info(ui));
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

fn scroll_window() -> UiRef {
    BoxFill::new(
        Color::GRAY_900,
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
                                    Color::GRAY_800,
                                    Color::GRAY_700,
                                    7.0,
                                    Center::new(Text::new(n)),
                                ),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            ),
        ),
    )
}

fn w95_window(texture: TextureRef, progress: f32, clear_color: &mut Color) -> UiRef {
    let color_button = id!();
    let wait_button = id!();

    if button_clicked_last_frame(color_button) {
        *clear_color = rand_color();
    }

    if button_clicked_last_frame(wait_button) {
        toggle_wait_for_events();
    }

    w95::Card::thick(
        20.0,
        Scroll::new(
            id!(),
            Col::with_gap(
                10.0,
                [
                    black_text(format!("{:.0}", avg_fps())),
                    black_text("Hello world!"),
                    black_text("This is UI."),
                    w95::Button::text(color_button, "Background"),
                    w95::Button::text(
                        wait_button,
                        format!("Toggle wait for events: {}", get_wait_for_events()),
                    ),
                    Text::new_with_size_color("Styled text", 30, Color::RED_600),
                    ConstrainedBox::max_size(
                        vec2(300.0, 200.0),
                        w95::Card::new(0.0, ImageNode::from_texture(texture)),
                    ),
                    w95::ProgressBar::new(vec2(200.0, 20.0), progress, 1.0, id!()),
                    GradientFill::top_to_bottom(
                        Color::NEUTRAL_100,
                        w95::PRIMARY,
                        Center::new(AspectRatio::new(
                            4.0,
                            BoxFill::new(
                                Color::NEUTRAL_200,
                                CircleFill::new(Color::NEUTRAL_300).sized(150.0, 150.0),
                            ),
                        )),
                    )
                    .sized(200.0, 200.0),
                ],
            ),
        ),
    )
}

fn text_window() -> UiRef {
    BoxFill::new(
        Color::NEUTRAL_950,
        Padding::all(
            20.0,
            Scroll::new(
                id!(),
                Col::new([
                    Text::title("Title"),
                    Text::body(
                        "Lorem ipsum dolor sit amet, consectetur\n\n adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
                    ),
                    Text::h1("Heading 1"),
                    Text::body("Lorem ipsum dolor sit amet."),
                    Text::h2("Heading 1"),
                    Text::body("Lorem ipsum dolor sit amet."),
                    Text::mono_sized(
                        format!(
                            "{} {} {} {}",
                            ICON_PLUS_CIRCLE, ICON_BOMB, ICON_CARET_DOWN, ICON_CIRCLE_NOTCH
                        ),
                        40,
                    ),
                    Text::h3("Heading 1"),
                    Text::body("Lorem ipsum dolor sit amet."),
                    Text::italic("Lorem ipsum dolor sit amet."),
                    Text::bold("Lorem ipsum dolor sit amet."),
                    Text::bold_italic("Lorem ipsum dolor sit amet."),
                    Col::new(
                        Palette::NEUTRAL
                            .shades()
                            .map(|c| bold_italic_colored("Lorem ipsum dolor sit amet.", c)),
                    ),
                ]),
            ),
        ),
    )
}

fn button_window(show_message: &mut bool) -> UiRef {
    let button = id!();

    if button_clicked_last_frame(button) {
        *show_message = !*show_message;
    }

    library::flat::Card::bg0_expand(Col::with_gap(
        30.0,
        [
            Center::new(flat::Button::primary_text(button, "Click me!")),
            if *show_message {
                Center::new(Text::body("Thanks for clicking."))
            } else {
                EMPTY
            },
        ],
    ))
    .scissored()
}

fn align_window() -> UiRef {
    BoxFill::new(
        Color::BLACK,
        Stack::new([
            Align::top_left(square(Color::RED_500)),
            Align::top_center(square(Color::ORANGE_500)),
            Align::top_right(square(Color::YELLOW_500)),
            Align::center_right(square(Color::GREEN_500)),
            Align::bottom_right(square(Color::SKY_500)),
            Align::bottom_center(square(Color::BLUE_500)),
            Align::bottom_left(square(Color::PURPLE_500)),
            Align::center_left(square(Color::PINK_500)),
            Align::center(square(Color::WHITE)),
        ]),
    )
    .scissored()
}

fn flat_window() -> UiRef {
    use flat::*;

    let bars: Vec<_> = Palette::PALETTES
        .iter()
        .enumerate()
        .map(|(i, p)| flat::LoadingBar::new_with_speed(p.v400, i as f32 * 10.0 + 10.0).height(30.0))
        .collect();

    let input_id = id!();

    if text_input_changed(input_id) {
        println!("{}", text_input_value(input_id));
    }

    Card::bg0_expand(FlexCol::with_gap(
        10.0,
        [
            FlexBox::Flex(Col::with_gap(10.0, bars).scroll(id!())),
            FlexBox::Fixed(TextInput::with_prompt(BG2, "Start typing...", input_id)),
            // FlexBox::Fixed(Text::new("Hello world.")),
        ],
    ))
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
    Text::new_full(text, SANS_BOLD_ITALIC, 24, color, true, 1.0, false)
}
