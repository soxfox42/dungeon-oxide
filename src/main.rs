mod ecs;

use ecs::{Component, World};

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

struct PositionComponent {
    x: f32,
    y: f32,
}
impl PositionComponent {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Component for PositionComponent {}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    world.register::<TextComponent>();
    world.register::<PositionComponent>();

    world.entity(|entity| {
        entity
            .component(TextComponent("Hello"))
            .component(PositionComponent::new(10.0, 20.0))
    });
    world.entity(|entity| {
        entity
            .component(TextComponent("This is component-based text."))
            .component(PositionComponent::new(30.0, 70.0))
    });

    loop {
        clear_background(BLACK);

        world.execute(|(text, pos): (&TextComponent, &PositionComponent)| {
            draw_text(text.0, pos.x, pos.y, 20.0, WHITE);
        });

        next_frame().await
    }
}
