use macroquad::prelude::*;

const TILE_SIZE: usize = 16;

pub fn draw_tile(spritesheet: Texture2D, tile: usize, x: f32, y: f32) {
    let row_len = spritesheet.width() as usize / TILE_SIZE;
    let (tile_x, tile_y) = (tile % row_len, tile / row_len);
    draw_texture_ex(spritesheet, x, y, WHITE, DrawTextureParams {
        source: Some(Rect::new(tile_x as f32, tile_y as f32, TILE_SIZE as f32, TILE_SIZE as f32)),
        ..Default::default()
    });
}
