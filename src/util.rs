use macroquad::prelude::*;

use crate::components::{Collider, Pos};
use crate::{LEVEL_WIDTH, TILE_SIZE};

pub fn draw_tile(spritesheet: Texture2D, tile: i32, x: f32, y: f32) {
    let row_len = spritesheet.width() as i32 / TILE_SIZE;
    let (tile_x, tile_y) = (tile % row_len * TILE_SIZE, tile / row_len * TILE_SIZE);
    draw_texture_ex(
        spritesheet,
        x,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                tile_x as f32,
                tile_y as f32,
                TILE_SIZE as f32,
                TILE_SIZE as f32,
            )),
            ..Default::default()
        },
    );
}

pub fn draw_tiles(tiles: &[u8], spritesheet: Texture2D) {
    for (i, tile) in tiles.iter().enumerate() {
        let x = i % LEVEL_WIDTH * TILE_SIZE as usize;
        let y = i / LEVEL_WIDTH * TILE_SIZE as usize;
        draw_tile(spritesheet, *tile as i32, x as f32, y as f32);
    }
}

pub fn aabb(pos1: Pos, coll1: Collider, pos2: Pos, coll2: Collider) -> bool {
    let Pos { x: x1, y: y1 } = pos1;
    let Pos { x: x2, y: y2 } = pos2;
    let Collider { w: w1, h: h1 } = coll1;
    let Collider { w: w2, h: h2 } = coll2;

    return x1 <= x2 + w2 && x1 + w1 >= x2 && y1 <= y2 + h2 && y1 + h1 >= y2;
}
