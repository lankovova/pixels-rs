use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl},
    prelude::*,
};

use crate::constants::*;

pub fn get_hovered_cell(cell_width: f32, cell_height: f32) -> Option<CellCoords> {
    let (mouse_pos_x, mouse_pos_y) = mouse_position();

    let cell_x = (mouse_pos_x / cell_width).floor();
    let cell_y = (mouse_pos_y / cell_height).floor();

    if cell_x < 0.0
        || cell_x >= CELLS_X_AMOUNT as f32
        || cell_y < 0.0
        || cell_y >= CELLS_Y_AMOUNT as f32
    {
        return None;
    }

    return Some(CellCoords {
        x: cell_x as usize,
        y: cell_y as usize,
    });
}

pub fn lighten_hsl(color: Color, amount: f32) -> Color {
    let (h, s, l) = rgb_to_hsl(color);
    hsl_to_rgb(h, s, (l + amount).min(1.0))
}

#[derive(Debug)]
pub struct CellCoords {
    pub x: usize,
    pub y: usize,
}
