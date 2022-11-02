mod ecs;

use ecs::{Component, Data, World};

use ::rand::Rng;
use itertools::{Itertools, izip};
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

struct DotComponent;
impl Component for DotComponent {}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    world.register::<TextComponent>();
    world.register::<PositionComponent>();
    world.register::<DotComponent>();

    let mut rng = ::rand::thread_rng();

    world.entity(|entity| {
        entity
            .component(DotComponent)
            .component(PositionComponent::new(200.0, 100.0))
    });
    world.entity(|entity| {
        entity
            .component(TextComponent("Hello, World!"))
            .component(PositionComponent::new(100.0, 200.0))
    });
    world.entity(|entity| {
        entity
            .component(TextComponent("This program uses an ECS framework."))
            .component(PositionComponent::new(300.0, 400.0))
    });
    world.entity(|entity| {
        entity
            .component(TextComponent("It involves multiple systems."))
            .component(PositionComponent::new(150.0, 450.0))
    });
    world.entity(|entity| {
        entity
            .component(DotComponent)
            .component(PositionComponent::new(500.0, 50.0))
    });

    loop {
        clear_background(BLACK);

        world.execute(|mut pos: Data<PositionComponent>| {
            for pos in pos.iter_mut().flatten() {
                pos.x += rng.gen_range(-2.0..2.0);
                pos.y += rng.gen_range(-2.0..2.0);
            }
        });
        world.execute(|pos: Data<PositionComponent>| {
            for (pos1, pos2) in pos.iter().flatten().tuple_windows() {
                draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 1.0, BLUE);
            }
        });
        world.execute(
            |(pos, text): (Data<PositionComponent>, Data<TextComponent>)| {
                for data in izip!(pos.iter(), text.iter()) {
                    if let (Some(pos), Some(text)) = data {
                        draw_text(text.0, pos.x, pos.y, 20.0, WHITE);
                    }
                }
            },
        );
        world.execute(
            |(pos, dot): (Data<PositionComponent>, Data<DotComponent>)| {
                for data in izip!(pos.iter(), dot.iter()) {
                    if let (Some(pos), Some(_dot)) = data {
                        draw_circle(pos.x, pos.y, 5.0, WHITE);
                    }
                }
            },
        );

        next_frame().await;
    }
}
