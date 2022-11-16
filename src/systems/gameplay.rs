use std::cmp;

use crate::ecs::World;
use crate::util::aabb;
use crate::{components::*, tiles};
use crate::{Context, LEVEL_WIDTH, TILE_SIZE};

use itertools::izip;
use macroquad::prelude::*;

const PLAYER_SPEED: i32 = 2;
pub fn player_input(world: &World<Context>, _ctx: &Context) {
    let pos = world.get::<Pos>();
    let mut health = world.get_mut::<Health>();
    let mut vel = world.get_mut::<Vel>();
    let mut player = world.get_mut::<Player>();

    let player_pos = pos[player.iter().position(Option::is_some).unwrap()].unwrap();

    for data in izip!(vel.iter_mut(), player.iter_mut()) {
        if let (Some(vel), Some(player)) = data {
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                vel.y = vel.y.min(-PLAYER_SPEED);
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                vel.y = vel.y.max(PLAYER_SPEED);
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                vel.x = vel.x.min(-PLAYER_SPEED);
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                vel.x = vel.y.max(PLAYER_SPEED);
            }

            if is_key_down(KeyCode::Space) {
                player.attack = true;
                for data in izip!(pos.iter(), health.iter_mut()) {
                    if let (Some(pos), Some(health)) = data {
                        let dx = player_pos.x - pos.x;
                        let dy = player_pos.y - pos.y;
                        let distance_sq = (dx as f64).powi(2) + (dy as f64).powi(2);
                        if distance_sq > 0.0 && distance_sq < 400.0 {
                            health.0 -= 1;
                        }
                    }
                }
            } else {
                player.attack = false;
            }
        }
    }
}

fn collide(pos: (i32, i32), collider: Collider, level: &[u8]) -> bool {
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

pub fn decelerate(world: &World<Context>, _ctx: &Context) {
    let mut vel = world.get_mut::<Vel>();

    for vel in vel.iter_mut().flatten() {
        vel.x -= vel.x.signum();
        vel.y -= vel.y.signum();
    }
}

pub fn update_health(world: &World<Context>, _ctx: &Context) {
    let mut health = world.get_mut::<Health>();
    let mut mods = world.get_mut::<HealthMod>();
    let pos = world.get::<Pos>();
    let mut vel = world.get_mut::<Vel>();
    let colliders = world.get::<Collider>();

    for (i, health) in health.iter_mut().enumerate() {
        if health.is_none() || colliders[i].is_none() {
            continue;
        }
        let health = health.as_mut().unwrap();
        for (j, modifier) in mods.iter_mut().enumerate() {
            if i == j || modifier.is_none() || colliders[j].is_none() {
                continue;
            }
            let modifier = modifier.as_mut().unwrap();
            if modifier.cooldown > 0 {
                continue;
            }

            if aabb(
                pos[i].unwrap(),
                colliders[i].unwrap(),
                pos[j].unwrap(),
                colliders[j].unwrap(),
            ) {
                health.0 += modifier.health;
                modifier.cooldown = 20;
                if let Some(ref mut vel) = vel[i] {
                    vel.x = (pos[i].unwrap().x - pos[j].unwrap().x) * 2 / 3;
                    vel.y = (pos[i].unwrap().y - pos[j].unwrap().y) * 2 / 3;
                }
                if let Some(ref mut vel) = vel[j] {
                    vel.x = (pos[j].unwrap().x - pos[i].unwrap().x) * 2 / 3;
                    vel.y = (pos[j].unwrap().y - pos[i].unwrap().y) * 2 / 3;
                }
            }
        }
    }

    for modifier in mods.iter_mut().flatten() {
        if modifier.cooldown > 0 {
            modifier.cooldown -= 1;
        }
    }
}

pub fn move_followers(world: &World<Context>, _ctx: &Context) {
    let pos = world.get::<Pos>();
    let mut vel = world.get_mut::<Vel>();
    let follow = world.get::<Follow>();

    for data in izip!(pos.iter(), vel.iter_mut(), follow.iter()) {
        if let (Some(my_pos), Some(vel), Some(follow)) = data {
            if pos[follow.0].is_none() {
                panic!("attempted to follow position-less entity");
            }
            let other_pos = pos[follow.0].unwrap();
            vel.x = cmp::max_by((other_pos.x - my_pos.x).signum(), vel.x, |a, b| a.abs().cmp(&b.abs()));
            vel.y = cmp::max_by((other_pos.y - my_pos.y).signum(), vel.y, |a, b| a.abs().cmp(&b.abs()));
        }
    }
}

pub fn move_pushables(world: &World<Context>, ctx: &Context) {
    let mut pos = world.get_mut::<Pos>();
    let vel = world.get::<Vel>();
    let colliders = world.get::<Collider>();
    let push = world.get::<Push>();
    let player = world.get::<Player>();

    let player_idx = player
        .iter()
        .position(Option::is_some)
        .expect("Player entity missing.");
    let player_pos = pos[player_idx].unwrap();
    let player_coll = colliders[player_idx].unwrap();
    let player_vel = vel[player_idx].unwrap();

    let mut new_pos = vec![None; pos.len()];
    let mut new_player_pos = player_pos;

    for (i, data) in izip!(pos.iter(), colliders.iter(), push.iter()).enumerate() {
        if let (Some(push_pos), Some(push_coll), Some(_push)) = data {
            if aabb(*push_pos, *push_coll, player_pos, player_coll) {
                if !collide(
                    (push_pos.x + player_vel.x, push_pos.y + player_vel.y),
                    *push_coll,
                    ctx.level,
                ) {
                    new_pos[i] = Some(Pos {
                        x: push_pos.x + player_vel.x,
                        y: push_pos.y + player_vel.y,
                    })
                } else {
                    new_player_pos = Pos {
                        x: player_pos.x - player_vel.x,
                        y: player_pos.y - player_vel.y,
                    };
                }
            }
        }
    }

    pos[player_idx] = Some(new_player_pos);

    for (pos, new_pos) in pos.iter_mut().zip(new_pos.iter()) {
        if let Some(new_pos) = new_pos {
            *pos = Some(*new_pos);
        }
    }
}

pub fn remove_dead(world: &World<Context>, _ctx: &Context) {
    let mut spr = world.get_mut::<Spr>();
    let mut colliders = world.get_mut::<Collider>();
    let health = world.get::<Health>();

    for (spr, collider, health) in izip!(spr.iter_mut(), colliders.iter_mut(), health.iter()) {
        if let Some(health) = health {
            if health.0 <= 0 {
                *spr = None;
                *collider = None;
            }
        }
    }
}
