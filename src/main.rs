mod ecs;

use ecs::{Component, Data, World};

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

        world.execute(|mut pos: Data<PositionComponent>| {
            for pos in pos.iter_mut().flatten() {
                pos.x += 1.0;
            }
        });
        world.execute(|pos: Data<PositionComponent>| {
            for pos1 in pos.iter().flatten() {
                for pos2 in pos.iter().flatten() {
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 1.0, BLUE);
                }
            }
        });
        world.execute(|pos: Data<PositionComponent>| {
            for pos in pos.iter().flatten() {
                draw_circle(pos.x, pos.y, 5.0, WHITE);
            }
        });

        next_frame().await;
    }
}
