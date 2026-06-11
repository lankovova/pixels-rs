use macroquad::rand;

use crate::{
    bitmap::Bitmap,
    constants::{CELLS_X_AMOUNT, CELLS_Y_AMOUNT},
};

fn try_move(bitmap: &mut Bitmap, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if nx < 0 || ny < 0 || nx >= CELLS_X_AMOUNT as isize || ny >= CELLS_Y_AMOUNT as isize {
        return false;
    }

    let nx = nx as usize;
    let ny = ny as usize;

    let target = bitmap.get(nx, ny);

    // TODO: Make it so some moves could swap real elements (sand sinking into the water)
    if target.is_none() {
        bitmap.swap_cells(x, y, nx, ny);
        return true;
    }

    return false;
}

pub fn try_fall(bitmap: &mut Bitmap, x: usize, y: usize) -> bool {
    try_move(bitmap, x, y, 0, 1)
}

pub fn try_diagonal_fall(bitmap: &mut Bitmap, x: usize, y: usize) -> bool {
    if rand::gen_range(0.0, 1.0) < 0.5 {
        // TODO: For diagonal movement check if there is no blocking pixels on the sides
        if try_move(bitmap, x, y, -1, 1) || try_move(bitmap, x, y, 1, 1) {
            return true;
        }
    } else {
        if try_move(bitmap, x, y, 1, 1) || try_move(bitmap, x, y, -1, 1) {
            return true;
        }
    }

    return false;
}

pub fn try_sideways(bitmap: &mut Bitmap, x: usize, y: usize) -> bool {
    if rand::gen_range(0.0, 1.0) < 0.5 {
        if try_move(bitmap, x, y, -1, 0) || try_move(bitmap, x, y, 1, 0) {
            return true;
        }
    } else {
        if try_move(bitmap, x, y, 1, 0) || try_move(bitmap, x, y, -1, 0) {
            return true;
        }
    }

    return false;
}
