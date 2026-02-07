use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Title")?;

    let mut controller = PanningCameraController::new();

    let rich_text = rich_text(
        r#"This text is {red3}red{white}, and this text is {blue3}blue{white}.
{#abc}You may make your text any {rgb 1.0 1.0 1.0}hex code.
{oklch 0.7 0.1184 119.12}Check the docume{blue2}ntation for {yellow3}rich_text(){orange2} for more.

Lorem ipsum dolor sit amet consectetur adipiscing elit.
Quisque faucibus ex sapien vitae pellentesque sem placerat.
In id cursus mi pretium tellus duis convallis.
Tempus leo eu aenean sed diam urna tempor.
Pulvinar vivamus fringilla lacus nec metus bibendum egestas.
Iaculis massa nisl malesuada lacinia integer nunc posuere.
"#,
    )?;
    let params = RichTextDrawParams {
        font_size: 32,
        ..Default::default()
    };

    dbg!(&rich_text);

    loop {
        controller.update();

        draw_logs();
        rich_text.draw_world(params);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
