use crate::{components::*, tiles};
use crate::ecs::World;
use crate::{Context, LEVEL_WIDTH, TILE_SIZE};

use itertools::izip;
use macroquad::prelude::*;

const PLAYER_SPEED: i32 = 1;
pub fn player_input(world: &World<Context>, _ctx: &Context) {
    let mut vel = world.get_mut::<Vel>();
    let player = world.get::<Player>();

    for data in izip!(vel.iter_mut(), player.iter()) {
        if let (Some(vel), Some(_player)) = data {
            vel.x = 0;
            vel.y = 0;
            if is_key_down(KeyCode::Up) {
                vel.y -= PLAYER_SPEED;
            }
            if is_key_down(KeyCode::Down) {
                vel.y += PLAYER_SPEED;
            }
            if is_key_down(KeyCode::Left) {
                vel.x -= PLAYER_SPEED;
            }
            if is_key_down(KeyCode::Right) {
                vel.x += PLAYER_SPEED;
            }
        }
    }
}

pub fn collide(pos: (i32, i32), collider: Collider, level: &[u8]) -> bool {
    let (x1, y1) = pos;
    let (x2, y2) = (x1 + collider.w, y1 + collider.h);
    let tiles = [
        (x1 / TILE_SIZE, y1 / TILE_SIZE),
        (x2 / TILE_SIZE, y1 / TILE_SIZE),
        (x1 / TILE_SIZE, y2 / TILE_SIZE),
        (x2 / TILE_SIZE, y2 / TILE_SIZE),
    ];
    for (x, y) in tiles {
        let idx = y as usize * LEVEL_WIDTH + x as usize;
        if tiles::SOLID[level[idx] as usize] {
            return true;
        }
    }
    false
}

pub fn apply_velocities(world: &World<Context>, ctx: &Context) {
    let mut pos = world.get_mut::<Pos>();
    let vel = world.get::<Vel>();
    let colliders = world.get::<Collider>();

    for data in izip!(pos.iter_mut(), vel.iter(), colliders.iter()) {
        if let (Some(pos), Some(vel), Some(collider)) = data {
            if !collide((pos.x + vel.x, pos.y), *collider, ctx.level) {
                pos.x += vel.x;
            }
            if !collide((pos.x, pos.y + vel.y), *collider, ctx.level) {
                pos.y += vel.y;
            }
        } else if let (Some(pos), Some(vel), _) = data {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}
