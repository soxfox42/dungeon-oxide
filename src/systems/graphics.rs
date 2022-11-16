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
            draw_tile(ctx.tileset, spr.0, pos.x as f32, pos.y as f32);
        }
    }
}

pub fn draw_attack(world: &World<Context>, _ctx: &Context) {
    let pos = world.get::<Pos>();
    let player = world.get::<Player>();
    for data in izip!(pos.iter(), player.iter()) {
        if let (Some(pos), Some(player)) = data {
            if player.attack {
                draw_circle_lines(pos.x as f32 + 7.5, pos.y as f32 + 7.5, 10.0, 1.0, BLUE);
            }
        }
    }
}

pub fn draw_health(world: &World<Context>, ctx: &Context) {
    let player = world.get::<Player>();
    let health = world.get::<Health>();
    for data in izip!(player.iter(), health.iter()) {
        if let (Some(_player), Some(health)) = data {
            for i in 0..5 {
                draw_tile(ctx.tileset, if health.0 > i { 238 } else { 239 }, (i * 14) as f32, 0.0);
            }
        }
    }
}
