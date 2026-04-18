use sge::prelude::*;
use ui::*;

const NODES: &[(&str, fn() -> UiRef)] = &[
    ("Align", align),
    ("Aspect Ratio", aspect_ratio),
    ("Border", border),
    ("Button", button),
    ("Center", center),
    ("Chart/line", line_chart),
    ("Circle", circle),
    ("Drawer", drawer),
    ("Fill/active", active_fill),
    ("Fill/box", box_fill),
    ("Fill/fill", fill),
    ("Fill/gradient", gradient_fill),
    ("Fill/rounded", rounded_fill),
    ("Hoverable", hoverable),
    ("Image", image),
    ("Inactive Overlay", inactive_overlay),
    ("Layout/col", col),
    ("Layout/grid", grid),
    ("Layout/row", row),
    ("Loading Bar", loading_bar),
    ("Progress Bar", progress_bar),
    ("Scissor Box", scissor_box),
    ("Scroll", scroll),
    ("Selectbox", selectbox),
    ("Sized Box", sized_box),
    ("Slider", slider),
    ("Stack", stack),
    ("Text Input", input),
    ("Window", window),
];
const SCHEME: ColorScheme = ColorScheme::LACKLUSTER;

fn main() -> anyhow::Result<()> {
    init("Ui Showcase")?;
    let select = id!();

    loop {
        clear_screen(SCHEME.bg1);

        let selected = select_box_value(select).unwrap_or(0);

        let ui = SizedBox::new(
            window_size(),
            FlexRow::new([
                FlexBox::Fixed(
                    SizedBox::wh(
                        200.0,
                        window_height(),
                        flat::SelectBox::new_text(
                            SCHEME.bg1,
                            SCHEME.bg2,
                            select,
                            NODES.iter().map(|n| n.0),
                        )
                        .scroll(id!()),
                    )
                    .fill(SCHEME.bg1)
                    .scroll(id!()),
                ),
                FlexBox::Flex(BoxFill::new(
                    SCHEME.bg0,
                    Padding::all(
                        20.0,
                        FlexCol::new([
                            FlexBox::Flex(Col::with_gap(
                                40.0,
                                [
                                    Text::h1_no_padding(NODES[selected].0),
                                    (NODES[selected].1)(),
                                ],
                            )),
                            FlexBox::Fixed(Text::mono(avg_fps() as usize)),
                        ]),
                    ),
                )),
            ]),
        );

        draw_ui(ui, Vec2::ZERO);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

fn center() -> UiRef {
    Center::new(text("Hello from the center of the panel"))
}

fn aspect_ratio() -> UiRef {
    Col::with_gap(
        20.0,
        [
            AspectRatio::new(
                2.0 / 1.0,
                BoxFill::new(SCHEME.blue, Center::new(text("Hello world"))),
            )
            .max_width(400.0),
            AspectRatio::new(
                1.0 / 2.0,
                BoxFill::new(SCHEME.red, Center::new(text("Hello world"))),
            )
            .max_height(400.0),
        ],
    )
}

fn active_fill() -> UiRef {
    SizedBox::new(
        Vec2::splat(300.0),
        ActiveFill::new(
            SCHEME.bg1,
            SCHEME.bg2,
            SCHEME.bg3,
            50.0,
            Center::new(text("Try hovering or clicking")),
        ),
    )
}

fn align() -> UiRef {
    fn container(x: AlignType, y: AlignType) -> UiRef {
        RoundedFill::new(
            SCHEME.bg1,
            20.0,
            Align::new(
                x,
                y,
                RoundedFill::new(
                    SCHEME.bg2,
                    10.0,
                    Center::new(Text::mono(format!("{}\n{}", x, y))),
                )
                .square(100.0),
            )
            .padding(10.0),
        )
        .square(200.0)
    }

    Fit::new(Grid::with_gap(
        3,
        3,
        20.0,
        [
            container(AlignType::Start, AlignType::Start),
            container(AlignType::Center, AlignType::Start),
            container(AlignType::End, AlignType::Start),
            container(AlignType::Start, AlignType::Center),
            container(AlignType::Center, AlignType::Center),
            container(AlignType::End, AlignType::Center),
            container(AlignType::Start, AlignType::End),
            container(AlignType::Center, AlignType::End),
            container(AlignType::End, AlignType::End),
        ],
    ))
}

fn border() -> UiRef {
    let b = BorderStyle::new(20.0, SCHEME.bg2);
    let c = BorderStyle::new(20.0, SCHEME.bg3);

    Border::tblr(c, c, b, b, BoxFill::new(SCHEME.bg1, EMPTY)).square(200.0)
}

fn box_fill() -> UiRef {
    BoxFill::new(SCHEME.green, EMPTY).square(200.0)
}

fn button() -> UiRef {
    #[derive(Default)]
    struct ButtonExampleState {
        enabled: bool,
        counter: usize,
    }

    storage_init_state(ButtonExampleState::default());

    let state = storage_get_state_mut::<ButtonExampleState>();
    let button_a = id!();
    let button_b = id!();

    if button_clicked_last_frame(button_a) {
        state.enabled = !state.enabled;
    }

    if button_clicked_last_frame(button_b) {
        state.counter += 1;
    }

    Col::with_gap(
        20.0,
        [
            flat::Button::primary(button_a, Text::nowrap("Click me!")),
            if state.enabled {
                text("Thanks for clicking!")
            } else {
                EMPTY
            },
            flat::Button::primary_text(button_b, "Increment"),
            text(state.counter),
        ],
    )
}

fn line_chart() -> UiRef {
    let data: Vec<_> = (0..90)
        .map(|i| sin((i as f32 * 5.0).to_radians()) + 1.0)
        .collect();

    flat::LineChart::new(data, 500.0, 200.0)
}

fn circle() -> UiRef {
    CircleFill::new(SCHEME.bg2).sized_wh(200.0, 200.0)
}

fn col() -> UiRef {
    fn b(n: usize) -> UiRef {
        BoxFill::new(SCHEME.bg2, Center::new(text(n))).square(100.0)
    }

    Col::with_gap(20.0, [b(0), b(1), b(2), b(3), b(4)])
        .padding(20.0)
        .fill(SCHEME.bg1)
        .fit()
}

fn drawer() -> UiRef {
    flat::Drawer::new(
        "Open drawer...",
        SCHEME.bg1,
        id!(),
        Col::with_gap(20.0,
            [
                Text::new("Porro voluptate quis voluptatum voluptatem aspernatur et. Natus quos debitis repellendus voluptas voluptatem sit odit. Tenetur voluptatem quis quibusdam. Iusto et repellat et et eos consequuntur. Et nesciunt distinctio eveniet et et velit nihil."),
                aspect_ratio()
            ]
        ).scroll(id!()).sized_wh(600.0, 400.0),
    )
    .max_width(400.0)
}

fn fill() -> UiRef {
    let value: &'static mut f32 = State::from_id(id!()).get_or_default();

    Col::with_gap(
        20.0,
        [
            RoundedFill::new(SCHEME.fg0, *value, EMPTY).square(200.0),
            flat::Slider::alternate(value, 0.0, 100.0, id!()).max_width(200.0),
        ],
    )
}

fn gradient_fill() -> UiRef {
    Grid::new(
        2,
        2,
        [
            GradientFill::top_to_bottom(SCHEME.bg3, SCHEME.bg0, EMPTY),
            GradientFill::bottom_to_top(SCHEME.bg3, SCHEME.bg0, EMPTY),
            GradientFill::left_to_right(SCHEME.bg3, SCHEME.bg0, EMPTY),
            GradientFill::right_to_left(SCHEME.bg3, SCHEME.bg0, EMPTY),
        ],
    )
    .square(400.0)
}

fn row() -> UiRef {
    fn b(n: usize) -> UiRef {
        BoxFill::new(SCHEME.bg2, Center::new(text(n))).square(100.0)
    }

    Row::with_gap(20.0, [b(0), b(1), b(2), b(3), b(4)])
        .padding(20.0)
        .fill(SCHEME.bg1)
        .fit()
}

fn grid() -> UiRef {
    fn b(n: usize) -> UiRef {
        BoxFill::new(SCHEME.bg2, Center::new(text(n))).square(100.0)
    }

    Grid::with_gap(
        3,
        3,
        20.0,
        [b(0), b(1), b(2), b(3), b(4), b(5), b(6), b(7), b(8)],
    )
    .padding(20.0)
    .fill(SCHEME.bg1)
    .fit()
}

fn hoverable() -> UiRef {
    let id = id!();

    Col::with_gap(
        20.0,
        [
            Hoverable::new(id, EMPTY).fill(SCHEME.bg2).square(200.0),
            if hovered_last_frame(id) {
                Text::body("Hovered")
            } else {
                EMPTY
            },
        ],
    )
}

fn image() -> UiRef {
    struct ImageExampleState {
        image: TextureRef,
    }

    if !storage_exists::<ImageExampleState>() {
        let texture = include_texture!("../assets/textures/guy.jpg");
        storage_store_state(ImageExampleState { image: texture });
    }

    let image = storage_get_state::<ImageExampleState>().image;

    ImageNode::from_texture(image)
}

fn inactive_overlay() -> UiRef {
    InactiveOverlay::new(Color::BLACK.with_alpha(0.5), image()).fit()
}

fn input() -> UiRef {
    flat::TextInput::with_prompt(SCHEME.bg2, "Enter text...", id!()).max_width(300.0)
}

fn loading_bar() -> UiRef {
    LoadingBar::new(Color::SKY_400, Color::SKY_400.darken(0.05)).sized_wh(300.0, 30.0)
}

fn progress_bar() -> UiRef {
    struct LoadingBarExampleState {
        progress: usize,
    }

    if !storage_exists::<LoadingBarExampleState>() {
        storage_store_state(LoadingBarExampleState { progress: 0 });
    }

    let state = storage_get_state_mut::<LoadingBarExampleState>();

    let button = id!();

    if button_clicked_last_frame(button) {
        state.progress += 1;
    }

    if state.progress > 5 {
        state.progress = 0;
    }

    Col::with_gap(
        20.0,
        [
            ProgressBar::new(state.progress as f32, 5.0, SCHEME.fg1, id!())
                .fill(SCHEME.bg1)
                .sized_wh(300.0, 50.0),
            flat::Button::primary_text(button, "Increment"),
        ],
    )
}

fn rounded_fill() -> UiRef {
    let value: &'static mut f32 = State::from_id(id!()).get_or_default();

    Col::with_gap(
        20.0,
        [
            RoundedFill::new(SCHEME.fg0, *value, EMPTY).square(200.0),
            flat::Slider::alternate(value, 0.0, 100.0, id!()).max_width(200.0),
        ],
    )
}

fn scissor_box() -> UiRef {
    Col::new([
        ScissorBox::new(Text::title_nowrap("With Scissor Box")).max_width(350.0),
        Text::title_nowrap("Without Scissor Box").max_width(350.0),
    ])
    .fill(SCHEME.bg3)
    .fit()
}

fn scroll() -> UiRef {
    let items = (0..100)
        .map(|n| {
            SizedBox::height(
                40.0,
                RoundedHoverFill::new(SCHEME.bg2, SCHEME.bg3, 7.0, Center::new(Text::new(n))),
            )
        })
        .collect::<Vec<_>>();

    Scroll::new(
        id!(),
        Col::with_gap(10.0, items)
            .padding(10.0)
            .fill(SCHEME.bg1)
            .fit(),
    )
    .sized_wh(300.0, 600.0)
}

fn selectbox() -> UiRef {
    let choices = ["Option 1", "Option 2", "Option 3", "Option 4", "Option 5"];
    let id = id!();

    Col::with_gap(
        20.0,
        [
            flat::SelectBox::new_text(SCHEME.bg1, SCHEME.bg2, id, choices).max_width(300.0),
            Text::new(select_box_value(id).unwrap_or(0)),
        ],
    )
}

fn sized_box() -> UiRef {
    SizedBox::wh(
        300.0,
        200.0,
        BoxFill::new(SCHEME.bg2, Center::new(text("Sized Box"))),
    )
}

fn slider() -> UiRef {
    struct SliderExampleState {
        a: f32,
        b: f32,
        c: f32,
    }

    storage_init_state::<SliderExampleState>(SliderExampleState {
        a: 10.0,
        b: 5.0,
        c: 15.0,
    });

    let state = storage_get_state_mut::<SliderExampleState>();

    Col::with_gap(
        20.0,
        [
            flat::Slider::new(&mut state.a, 0.0, 30.0, id!()),
            flat::Slider::alternate(&mut state.b, 0.0, 30.0, id!()),
            flat::Slider::rounded(&mut state.c, 0.0, 30.0, id!()),
        ],
    )
    .max_width(300.0)
}

fn stack() -> UiRef {
    let square = BoxFill::new(SCHEME.bg3, EmptyBox::new(50.0, 50.0));

    Col::with_gap(
        20.0,
        [
            BoxFill::new(
                SCHEME.bg1,
                Stack::new([
                    Align::top_left(square),
                    Align::top_center(square),
                    Align::top_right(square),
                    Align::center_right(square),
                    Align::bottom_right(square),
                    Align::bottom_center(square),
                    Align::bottom_left(square),
                    Align::center_left(square),
                    Align::center(square),
                ]),
            )
            .max_size(Vec2::splat(300.0)),
            text("Overlays elements on top of eachother"),
        ],
    )
}

fn window() -> UiRef {
    let size = vec2(400.0, 600.0);
    flat::FloatingWindow::custom(
        "Example window",
        size,
        vec2((window_width() - size.x) / 2.0, 100.0),
        id!(),
        Center::new(text("This is a floating window. You can drag it around!")),
    )
    .max_width(300.0)
}

fn text(text: impl ToString) -> UiRef {
    Text::new_with_color(text, SCHEME.fg0)
}
