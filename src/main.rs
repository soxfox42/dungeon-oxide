mod ecs;

use ecs::{World, Component};

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Oxide".into(),
        high_dpi: true,
        ..Default::default()
    }
}

struct TextComponent(&'static str);
impl Component for TextComponent {}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    world.register::<TextComponent>();

    world.entity(|entity| entity.component(TextComponent("Hello")));

    for component in world.component::<TextComponent>() {
        if let Some(TextComponent(text)) = component {
            println!("{}", text);
        }
    }

    loop {
        clear_background(BLACK);
        draw_text("Hello, world!", 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
