use crate::components::*;
use crate::ecs::World;
use crate::Context;

use itertools::izip;
use macroquad::prelude::*;

const PLAYER_SPEED: i32 = 1;
pub fn move_player(world: &World<Context>, _ctx: &Context) {
    let mut pos = world.get_mut::<Pos>();
    let player = world.get::<Player>();

    for data in izip!(pos.iter_mut(), player.iter()) {
        if let (Some(pos), Some(_player)) = data {
            if is_key_down(KeyCode::Up) {
                pos.y -= PLAYER_SPEED;
            }
            if is_key_down(KeyCode::Down) {
                pos.y += PLAYER_SPEED;
            }
            if is_key_down(KeyCode::Left) {
                pos.x -= PLAYER_SPEED;
            }
            if is_key_down(KeyCode::Right) {
                pos.x += PLAYER_SPEED;
            }
        }
    }
}
