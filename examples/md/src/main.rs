use markdown::mdast::Node;
use sge::{ui::flat::Hyperlink, *};
use ui::*;

const BG: Color = Color::WHITE;
const BG2: Color = Color::NEUTRAL_100;
const FG: Color = Color::NEUTRAL_900;

#[main("Markdown viewer")]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("provide a path to md file as first arguement");
    let text = std::fs::read_to_string(path)?;
    let ast = markdown::to_mdast(&text, &markdown::ParseOptions::default()).unwrap();
    let ui = markdown_to_ui(ast.clone());
    let dimensions = ui.preferred_dimensions().round();

    let texture = create_empty_render_texture(dimensions.x as u32, dimensions.y as u32)?;
    start_rendering_to_texture(texture);
    draw_ui(ui, Vec2::ZERO);
    end_rendering_to_texture();

    let mut y_offset = 0.0 - window_height() / 4.0;

    loop {
        clear_screen(Color::NEUTRAL_100);

        y_offset += scroll_diff().y * 20.0;

        let ui = markdown_to_ui(ast.clone());
        draw_ui(ui, window_center() + vec2(-dimensions.x / 2.0, y_offset));

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    let bytes = texture.color_texture.download_to_image().to_png_bytes();
    write_file("output.png", bytes).await?;

    Ok(())
}

fn markdown_to_ui(node: Node) -> UiRef {
    match node {
        Node::Root(root) => BoxFill::new(
            BG,
            Padding::all(
                30.0,
                Col::new(
                    root.children
                        .into_iter()
                        .map(markdown_to_ui)
                        .collect::<Vec<_>>(),
                ),
            ),
        )
        .width(800.0)
        .fit_vertical(),
        Node::Paragraph(paragraph) => Col::new(
            paragraph
                .children
                .into_iter()
                .map(markdown_to_ui)
                .collect::<Vec<_>>(),
        ),
        Node::Text(text) => Text::new_with_color(text.value, FG).padding_bottom(20.0),
        Node::Heading(heading) => match heading.children.first() {
            Some(Node::Text(text)) => heading_node(text.value.clone(), heading.depth),
            _ => heading_node("Unknown heading".to_string(), heading.depth),
        },
        Node::Code(code) => BoxFill::new(
            BG2,
            Padding::tblr(10.0, 15.0, 10.0, 10.0, code_node(code.value)),
        )
        .fit_vertical()
        .padding_vertical(10.0),
        Node::Link(link) => {
            flat::Hyperlink::with_title_dark(link.url.clone(), link.title.unwrap_or(link.url))
        }
        Node::ThematicBreak(_) => BoxFill::new(Color::NEUTRAL_400, EMPTY)
            .height(2.0)
            .padding(10.0),
        Node::Strong(strong) => match strong.children.first() {
            Some(Node::Text(text)) => {
                Text::new_full(text.value.clone(), SANS_BOLD, 18, FG, true, 1.0, false)
                    .padding_bottom(20.0)
            }
            _ => Text::new_full(
                "Unknown strong".to_string(),
                SANS_DISPLAY,
                18,
                FG,
                true,
                1.0,
                false,
            )
            .padding_bottom(20.0),
        },
        Node::Emphasis(em) => match em.children.first() {
            Some(Node::Text(text)) => {
                Text::new_full(text.value.clone(), SANS_ITALIC, 18, FG, false, 1.0, false)
                    .padding_bottom(20.0)
            }
            _ => Text::new_full(
                "Unknown emphasis".to_string(),
                SANS_DISPLAY,
                18,
                FG,
                false,
                1.0,
                false,
            )
            .padding_bottom(20.0),
        },
        Node::Blockquote(blockquote) => FlexRow::with_gap(
            30.0,
            [
                FlexBox::Fixed(BoxFill::new(Color::NEUTRAL_400, EMPTY).width(10.0)),
                FlexBox::Flex(
                    Col::new(
                        blockquote
                            .children
                            .into_iter()
                            .map(markdown_to_ui)
                            .collect::<Vec<_>>(),
                    )
                    .expand_x(),
                ),
            ],
        )
        .fit()
        .padding_vertical(20.0),
        a => panic!("{:#?}", a),
    }
}

fn heading_node(text: String, depth: u8) -> UiRef {
    let font_size = match depth {
        1 => 32,
        2 => 28,
        3 => 24,
        4 => 20,
        5 => 18,
        _ => 16,
    };

    Text::new_full(text, SANS_DISPLAY, font_size, FG, false, 1.0, true)
        .padding_bottom(20.0)
        .padding_top(10.0)
}

fn code_node(text: String) -> UiRef {
    Text::new_full(text, MONO, 16, FG, false, 1.0, false)
}
