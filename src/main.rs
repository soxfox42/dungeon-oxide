use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Oxide".into(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_text("Hello, world!", 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
