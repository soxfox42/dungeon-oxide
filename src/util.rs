use macroquad::prelude::*;

const TILE_SIZE: usize = 16;
const LEVEL_WIDTH: usize = 16;

pub fn draw_tile(spritesheet: Texture2D, tile: usize, x: f32, y: f32) {
    let row_len = spritesheet.width() as usize / TILE_SIZE;
    let (tile_x, tile_y) = (tile % row_len * TILE_SIZE, tile / row_len * TILE_SIZE);
    draw_texture_ex(spritesheet, x, y, WHITE, DrawTextureParams {
        source: Some(Rect::new(tile_x as f32, tile_y as f32, TILE_SIZE as f32, TILE_SIZE as f32)),
        ..Default::default()
    });
}

pub fn draw_tiles(tiles: &[u8], spritesheet: Texture2D) {
    for (i, tile) in tiles.iter().enumerate() {
        let x = i % LEVEL_WIDTH * TILE_SIZE;
        let y = i / LEVEL_WIDTH * TILE_SIZE;
        draw_tile(spritesheet, *tile as usize, x as f32, y as f32);
    }
}
