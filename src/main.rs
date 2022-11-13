mod ecs;

use ecs::{Component, World};

use ::rand::Rng;
use itertools::{izip, Itertools};
use macroquad::prelude::*;

struct Text(&'static str);
impl Component for Text {}

struct Position {
    x: f32,
    y: f32,
}
impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Component for Position {}

struct Point;
impl Component for Point {}

fn wiggle(world: &World) {
    let mut rng = ::rand::thread_rng();
    let mut pos_data = world.get_mut::<Position>();
    for pos in pos_data.iter_mut().flatten() {
        pos.x += rng.gen_range(-2.0..2.0);
        pos.y += rng.gen_range(-2.0..2.0);
    }
}

fn lines(world: &World) {
    let pos_data = world.get::<Position>();
    for (pos1, pos2) in pos_data.iter().flatten().tuple_windows() {
        draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 1.0, BLUE);
    }
}

fn points(world: &World) {
    let pos_data = world.get::<Position>();
    let point_data = world.get::<Point>();
    for data in izip!(pos_data.iter(), point_data.iter()) {
        let (Some(pos), Some(_)) = data else { continue; };
        draw_circle(pos.x, pos.y, 5.0, WHITE);
    }
}

fn text(world: &World) {
    let pos_data = world.get::<Position>();
    let text_data = world.get::<Text>();
    for data in izip!(pos_data.iter(), text_data.iter()) {
        let (Some(pos), Some(text)) = data else { continue; };
        draw_text(text.0, pos.x, pos.y, 20.0, WHITE);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Oxide".into(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    world.register::<Text>();
    world.register::<Position>();
    world.register::<Point>();

    world.add_entity(|entity| {
        entity
            .with_component(Point)
            .with_component(Position::new(200.0, 100.0))
    });
    world.add_entity(|entity| {
        entity
            .with_component(Text("Hello, World!"))
            .with_component(Position::new(100.0, 200.0))
    });
    world.add_entity(|entity| {
        entity
            .with_component(Text("This program uses an ECS framework."))
            .with_component(Position::new(300.0, 400.0))
    });
    world.add_entity(|entity| {
        entity
            .with_component(Text("It involves multiple systems."))
            .with_component(Position::new(150.0, 450.0))
    });
    world.add_entity(|entity| {
        entity
            .with_component(Point)
            .with_component(Position::new(500.0, 50.0))
    });

    // update
    world.system(wiggle);

    // render
    world.system(lines);
    world.system(points);
    world.system(text);

    loop {
        clear_background(BLACK);
        world.tick();
        next_frame().await;
    }
}
