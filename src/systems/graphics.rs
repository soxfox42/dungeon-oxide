use crate::components::*;
use crate::ecs::World;
use crate::util::*;
use crate::Context;

use itertools::izip;
use macroquad::prelude::*;

pub fn draw_sprites(world: &World<Context>, ctx: &Context) {
    let pos = world.get::<Pos>();
    let spr = world.get::<Spr>();
    for data in izip!(pos.iter(), spr.iter()) {
        if let (Some(pos), Some(spr)) = data {
            draw_tile(ctx.spritesheet, spr.0, pos.x as f32, pos.y as f32);
        }
    }
}

pub fn draw_health(world: &World<Context>, _ctx: &Context) {
    let player = world.get::<Player>();
    let health = world.get::<Health>();
    for data in izip!(player.iter(), health.iter()) {
        if let (Some(_player), Some(health)) = data {
            draw_text(&format!("Health: {}", health.0), 10.0, 15.0, 10.0, WHITE);
        }
    }
}
