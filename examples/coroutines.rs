use sge::prelude::*;

#[main("Title")]
async fn main() -> anyhow::Result<()> {
    let coroutine = start_coroutine(count());

    loop {
        if coroutine.is_done() {
            draw_text("Done", Vec2::ZERO);
        }

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

async fn count() {
    for i in 0..100 {
        draw_text(i, Vec2::ZERO);

        next_frame().await;
    }
}
