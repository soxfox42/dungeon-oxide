use crate::Context;
use crate::components::*;
use crate::ecs::World;
use crate::util::draw_tile;

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
