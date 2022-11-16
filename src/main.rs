mod components;
mod ecs;
mod loader;
mod systems;
mod tiles;
mod util;

use components::*;
use ecs::World;
use loader::load_level;
use systems::*;
use util::draw_tiles;

use macroquad::prelude::*;

const TILE_SIZE: i32 = 16;
const LEVEL_WIDTH: usize = 16;

/// Global data passed to all systems
pub struct Context {
    tileset: Texture2D,
    map: &'static [u8],
}

const LEVELS: &[&str] = &[
    include_str!("../levels/level1.json"),
    include_str!("../levels/level2.json"),
];

const MAPS: &[&[u8]] = &[
    include_bytes!("../levels/level1.dat"),
    include_bytes!("../levels/level2.dat"),
];

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Oxide".into(),
        window_width: 1024,
        window_height: 768,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut context = Context {
        tileset: Texture2D::from_file_with_format(
            include_bytes!("../assets/tiles.png"),
            Some(ImageFormat::Png),
        ),
        map: MAPS[0],
    };

    let mut world = World::new();
    register_components(&mut world);
    register_systems(&mut world);

    let mut current_level = 0;
    let map_id = load_level(&mut world, LEVELS[current_level]);
    context.map = MAPS[map_id];

    let render_target = render_target(256, 192);
    render_target.texture.set_filter(FilterMode::Nearest);
    let camera = Camera2D {
        zoom: vec2(2.0 / 256.0, 2.0 / 192.0),
        target: vec2(256.0 / 2.0, 192.0 / 2.0),
        render_target: Some(render_target),
        ..Default::default()
    };
    loop {
        if is_key_pressed(KeyCode::Comma) {
            current_level = (current_level - 1).rem_euclid(LEVELS.len());
            world = World::new();
            register_components(&mut world);
            register_systems(&mut world);
            let map_id = load_level(&mut world, LEVELS[current_level]);
            context.map = MAPS[map_id];
        }
        if is_key_pressed(KeyCode::Period) {
            current_level = (current_level + 1).rem_euclid(LEVELS.len());
            world = World::new();
            register_components(&mut world);
            register_systems(&mut world);
            let map_id = load_level(&mut world, LEVELS[current_level]);
            context.map = MAPS[map_id];
        }
        let player_idx = world.get::<Player>().iter().position(Option::is_some).unwrap();
        if world.get::<Spr>()[player_idx].is_none() {
            world = World::new();
            register_components(&mut world);
            register_systems(&mut world);
            let map_id = load_level(&mut world, LEVELS[current_level]);
            context.map = MAPS[map_id];
        }

        set_camera(&camera);
        clear_background(BLACK);

        draw_tiles(context.map, context.tileset);

        world.tick(&context);

        set_default_camera();
        draw_texture_ex(
            render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
