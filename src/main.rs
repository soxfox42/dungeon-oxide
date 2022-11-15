mod components;
mod ecs;
mod systems;
mod tiles;
mod util;

use components::*;
use ecs::World;
use systems::*;
use util::draw_tiles;

use macroquad::prelude::*;

const TILE_SIZE: i32 = 16;
const LEVEL_WIDTH: usize = 16;

/// Global data passed to all systems
pub struct Context {
    spritesheet: Texture2D,
    tileset: Texture2D,
    level: &'static [u8],
}

const LEVELS: &[&[u8]] = &[include_bytes!("../levels/level1.dat")];

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
    let context = Context {
        spritesheet: Texture2D::from_file_with_format(
            include_bytes!("../assets/sprites.png"),
            Some(ImageFormat::Png),
        ),
        tileset: Texture2D::from_file_with_format(
            include_bytes!("../assets/tiles.png"),
            Some(ImageFormat::Png),
        ),
        level: LEVELS[0],
    };

    let mut world = World::new();
    register_components(&mut world);
    register_systems(&mut world);

    world.add_entity(|entity| {
        entity
            .with_component(Pos::new(32, 32))
            .with_component(Vel::new(0, 0))
            .with_component(Collider::new(16, 16))
            .with_component(Spr(0))
            .with_component(Player)
    });

    let render_target = render_target(256, 192);
    render_target.texture.set_filter(FilterMode::Nearest);
    let camera = Camera2D {
        zoom: vec2(2.0 / 256.0, 2.0 / 192.0),
        target: vec2(256.0 / 2.0, 192.0 / 2.0),
        render_target: Some(render_target),
        ..Default::default()
    };
    loop {
        set_camera(&camera);
        clear_background(BLACK);

        draw_tiles(context.level, context.tileset);

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
