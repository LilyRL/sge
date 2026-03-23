use sge::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Sound")?;

    let sound = load_sound_from_bytes(include_bytes!("../assets/sounds/vine-boom.mp3"))?;

    loop {
        if key_pressed(KeyCode::Space) {
            play_sound(sound);
        } else if key_pressed(KeyCode::KeyR) {
            play_sound_ex(sound).speed(1.5).start();
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
