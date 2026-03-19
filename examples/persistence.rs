use sge::prelude::*;

const PATH: &'static str = "/tmp/config.bin";

#[derive(PartialEq, Debug)]
#[persistent]
struct Config {
    counter: u64,
    foo: Foo,
    color: Color,
}

#[derive(PartialEq, Debug)]
#[persistent]
struct Foo {
    bar: bool,
    baz: Vec2,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            counter: 1,
            foo: Foo {
                bar: false,
                baz: vec2(100.0, 250.0),
            },
            color: Color::RED_300,
        }
    }
}

fn main() -> anyhow::Result<()> {
    init("Persistence")?;

    let mut config = Config::load(PATH).unwrap_or_else(|e| {
        println!("Could not load config: {e}");
        let _ = std::fs::remove_file(PATH);
        Default::default()
    });
    let texture = create_empty_render_texture(2000, 40)?;

    loop {
        draw_big_number(texture, config.color, config.counter);
        sharpen_screen(1.0);

        if key_held(KeyCode::Space) {
            config.counter += 1;
        }

        if key_held(KeyCode::KeyC) {
            config.color = config.color.hue_rotate_oklch(1.0);
        }

        if key_pressed(KeyCode::KeyT) {
            test();
        }

        if key_pressed(KeyCode::KeyD) {
            config = Config::default();
            let _ = std::fs::remove_file(PATH);
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    // please note that if the app is force quit (e.g.: Ctrl-C) this code will not run
    config.save(PATH)?;

    Ok(())
}

fn draw_big_number(texture: RenderTextureRef, bg: Color, number: u64) {
    start_rendering_to_texture(texture);
    clear_screen(bg);
    let mut size = draw_text_ex(number.to_string(), Vec2::ZERO, Color::BLACK, 40).size;
    end_rendering_to_texture();

    let height = size.y;
    size.y -= height * 0.25;
    size.x -= 7.0;

    clear_screen(bg);
    draw_texture_ex(
        texture.color_texture,
        Transform2D::from_scale_translation(window_size(), Vec2::ZERO),
        Color::WHITE,
        Some(bevy_math::Rect::from_corners(
            vec2(4.0, height * 0.25),
            size,
        )),
    );
}

fn test() {
    let path = "/tmp/config-test.bin";
    let value = Config {
        counter: 19823793,
        foo: Config::default().foo,
        color: Color::BLUE_400,
    };
    value.save(path).unwrap();

    for _ in 0..1000 {
        let loaded = Config::load(path).unwrap();
        loaded.save(path).unwrap();
    }

    let loaded = Config::load(path).unwrap();
    assert_eq!(value, loaded);
    println!("Test suceeded");
}
