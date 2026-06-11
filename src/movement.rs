use crate::bitmap::Bitmap;
use macroquad::rand;

fn try_move(bitmap: &mut Bitmap, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if !bitmap.is_in_bounds(nx, ny) {
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
        if bitmap.is_empty(x as isize - 1, y as isize) && try_move(bitmap, x, y, -1, 1) {
            return true;
        }
        if bitmap.is_empty(x as isize + 1, y as isize) && try_move(bitmap, x, y, 1, 1) {
            return true;
        }
    } else {
        if bitmap.is_empty(x as isize + 1, y as isize) && try_move(bitmap, x, y, 1, 1) {
            return true;
        }
        if bitmap.is_empty(x as isize - 1, y as isize) && try_move(bitmap, x, y, -1, 1) {
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
