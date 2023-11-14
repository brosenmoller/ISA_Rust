use sdl2::{rect::Rect, render::Canvas, video::Window, pixels::Color};

use crate::vector2int::Vector2Int;

pub struct GridTile {
    pub position: Vector2Int,
    color: Color,
    grid_tile_size: u32,
    rect: Rect,
}

impl GridTile {
    pub fn new(position: Vector2Int, color: Color, grid_tile_size: u32) -> GridTile {
        let grid_tile = GridTile { 
            position, 
            color,
            grid_tile_size,
            rect: Rect::new(
                0, 
                0,
                (grid_tile_size as f32 * 0.9) as u32,
                (grid_tile_size as f32 * 0.9) as u32,
            ),
        };

        return grid_tile;
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>){

        self.rect.x = self.position.x * self.grid_tile_size as i32;
        self.rect.y = self.position.y * self.grid_tile_size as i32;

        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).unwrap();
    }
}