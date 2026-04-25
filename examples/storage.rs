use sge::prelude::*;

struct MyState {
    score: usize,
}

#[main("Storage")]
fn main() -> anyhow::Result<()> {
    let state = MyState { score: 0 };

    storage_store_state(state);

    loop {
        clear_screen(Color::GRAY_900);

        show_score();
        increment_score();

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn show_score() {
    let state = storage_get_state::<MyState>();
    draw_text_size(state.score.to_string(), Vec2::splat(100.0), 100);
}

fn increment_score() {
    let state = storage_get_state_mut::<MyState>();
    state.score += 1;
}
